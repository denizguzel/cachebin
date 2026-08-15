use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::SystemTime;

const MAX_WALK_ENTRIES: usize = 500_000;

/// A single file discovered while walking, with its disk footprint and modification time.
pub struct FileHit {
    pub path: PathBuf,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

/// Recursively sums the disk usage of `path`. Returns an error when `cancelled` is set so a long
/// scan can be aborted mid-directory.
///
/// On Windows this uses `NtQueryDirectoryFile` batch enumeration (the counterpart of the macOS
/// `FileManager.enumerator`): one directory query returns size, allocation and timestamps for
/// every entry in a directory, so there is no per-file syscall.
/// On Unix it walks with `walkdir`, where the link count is free from `stat`.
pub fn dir_size(path: &Path, cancelled: &AtomicBool) -> Result<(u64, Option<SystemTime>), String> {
    let mut hits = Vec::new();
    #[cfg(windows)]
    win::collect(path, cancelled, &mut |path, size, modified| {
        hits.push(FileHit { path: path.to_path_buf(), size, modified: Some(modified) });
    })?;
    #[cfg(not(windows))]
    walkdir_collect(path, cancelled, &mut |path, size, modified| {
        hits.push(FileHit { path: path.to_path_buf(), size, modified });
    })?;

    let mut total = 0u64;
    let mut newest = None;
    for hit in hits {
        total = total.saturating_add(hit.size);
        if let Some(modified) = hit.modified {
            if newest.map_or(true, |best: SystemTime| modified > best) {
                newest = Some(modified);
            }
        }
    }
    Ok((total, newest))
}

/// Collects files at least `threshold` bytes in size under `path`, up to `limit` results.
pub fn large_files(
    path: &Path,
    threshold: u64,
    limit: usize,
    cancelled: &AtomicBool,
) -> Result<Vec<FileHit>, String> {
    let mut hits = Vec::new();
    #[cfg(windows)]
    win::collect(path, cancelled, &mut |path, size, modified| {
        if size >= threshold && hits.len() < limit {
            hits.push(FileHit { path: path.to_path_buf(), size, modified: Some(modified) });
        }
    })?;
    #[cfg(not(windows))]
    walkdir_collect(path, cancelled, &mut |path, size, modified| {
        if size >= threshold && hits.len() < limit {
            hits.push(FileHit { path: path.to_path_buf(), size, modified });
        }
    })?;
    Ok(hits)
}

#[cfg(not(windows))]
fn walkdir_collect(
    path: &Path,
    cancelled: &AtomicBool,
    on_file: &mut dyn FnMut(&Path, u64, Option<SystemTime>),
) -> Result<(), String> {
    use std::sync::atomic::Ordering;

    use walkdir::WalkDir;

    let mut walked = 0usize;

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        if walked >= MAX_WALK_ENTRIES {
            break;
        }
        if cancelled.load(Ordering::Relaxed) {
            return Err("Scan cancelled".into());
        }
        walked += 1;

        if entry.file_type().is_dir() {
            continue;
        }

        let Ok(meta) = entry.metadata() else {
            continue;
        };
        if !meta.is_file() {
            continue;
        }

        // Logical size is used for collection; the caller folds hardlink counts in when summing.
        let size = meta.len() / link_count(&meta).max(1);
        let modified = meta.modified().ok();
        on_file(entry.path(), size, modified);
    }

    Ok(())
}

/// Hard-link count from metadata. A file with N links only frees `size / N` bytes when one link is
/// removed; pnpm / Bun / Cargo stores hardlink into project caches, so without this we overestimate.
#[cfg(not(windows))]
fn link_count(meta: &std::fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    meta.nlink()
}

#[cfg(windows)]
mod win {
    use std::collections::HashSet;
    use std::os::windows::ffi::OsStrExt;
    use std::path::{Path, PathBuf};
    use std::ptr;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, SystemTime};

    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE, NTSTATUS};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_BACKUP_SEMANTICS,
        FILE_LIST_DIRECTORY, FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
        OPEN_EXISTING,
    };
    use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;
    use windows_sys::Wdk::Storage::FileSystem::{
        FileIdBothDirectoryInformation, NtQueryDirectoryFile, FILE_ID_BOTH_DIR_INFORMATION,
    };

    const STATUS_NO_MORE_FILES: NTSTATUS = 0x8000_0006u32 as NTSTATUS;
    const EPOCH_DIFF_TICKS: u64 = 11_644_473_600 * 10_000_000; // 1601-01-01 to 1970-01-01, 100ns units

    pub fn collect(
        root: &Path,
        cancelled: &AtomicBool,
        on_file: &mut dyn FnMut(&Path, u64, SystemTime),
    ) -> Result<(), String> {
        let handle = open_dir(root)?;
        let mut buffer = vec![0u64; 8192]; // 64 KiB, heap-allocated and 8-byte aligned
        let mut walked = 0usize;
        let mut visited = HashSet::new();

        let result = walk_dir(handle, root, &mut buffer, &mut walked, cancelled, &mut visited, on_file);

        // SAFETY: `handle` was opened above and is still valid.
        unsafe {
            CloseHandle(handle);
        }
        result
    }

    fn open_dir(path: &Path) -> Result<HANDLE, String> {
        let wide = path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();

        // SAFETY: `wide` is a NUL-terminated UTF-16 path and all pointers stay valid for the call.
        let handle = unsafe {
            CreateFileW(
                wide.as_ptr(),
                FILE_LIST_DIRECTORY | FILE_READ_ATTRIBUTES,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                ptr::null(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS,
                ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            Err(format!("cannot open directory {}", path.display()))
        } else {
            Ok(handle)
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn walk_dir(
        dir: HANDLE,
        path: &Path,
        buffer: &mut [u64],
        walked: &mut usize,
        cancelled: &AtomicBool,
        visited: &mut HashSet<u64>,
        on_file: &mut dyn FnMut(&Path, u64, SystemTime),
    ) -> Result<(), String> {
        // Subdirectories are collected during parsing and recursed into afterwards: recursing
        // mid-parse would overwrite `buffer` (reused per directory query) and corrupt the entries
        // the parent is still reading.
        let mut subdirs: Vec<(PathBuf, u64)> = Vec::new();

        loop {
            if cancelled.load(Ordering::Relaxed) {
                return Err("Scan cancelled".into());
            }
            if *walked >= super::MAX_WALK_ENTRIES {
                return Ok(());
            }

            let mut io = IO_STATUS_BLOCK::default();
            // SAFETY: `buffer` is valid for writes of `buffer.len()` bytes and `io` for the result.
            let status = unsafe {
                NtQueryDirectoryFile(
                    dir,
                    ptr::null_mut(),
                    None,
                    ptr::null(),
                    &mut io,
                    buffer.as_mut_ptr() as *mut core::ffi::c_void,
                    (buffer.len() * size_of::<u64>()) as u32,
                    FileIdBothDirectoryInformation,
                    false,
                    ptr::null(),
                    false,
                )
            };

            if status == STATUS_NO_MORE_FILES {
                break;
            }
            if status != 0 {
                // An unreadable directory (permissions) should not fail the whole scan.
                return Ok(());
            }

            let base = buffer.as_ptr() as *const u8;
            let mut offset = 0usize;
            loop {
                if offset + size_of::<FILE_ID_BOTH_DIR_INFORMATION>() > buffer.len() * size_of::<u64>() {
                    break;
                }
                // SAFETY: entries are 8-byte aligned inside the aligned buffer, and each parsed
                // entry plus its file name stays within the buffer (`FileNameLength` bounds it).
                let info = unsafe { &*(base.add(offset) as *const FILE_ID_BOTH_DIR_INFORMATION) };
                let name_len = info.FileNameLength as usize;
                let name = if name_len == 0 {
                    String::new()
                } else {
                    let name_slice =
                        unsafe { std::slice::from_raw_parts(info.FileName.as_ptr(), name_len / 2) };
                    String::from_utf16_lossy(name_slice)
                };

                let is_dir = info.FileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0;
                let is_reparse = info.FileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0;

                if name != "." && name != ".." {
                    if is_dir && !is_reparse {
                        // Guard against directory cycles (junctions) via the file id. A zero file id
                        // (unusual filesystems) falls back to the reparse-point skip above.
                        subdirs.push((PathBuf::from(path).join(name), info.FileId as u64));
                    } else if !is_dir && !is_reparse {
                        if *walked >= super::MAX_WALK_ENTRIES {
                            return Ok(());
                        }
                        *walked += 1;
                        // Allocation size is the disk footprint (sparse files count their real use).
                        let size = info.AllocationSize.max(0) as u64;
                        let modified = filetime_to_systemtime(info.LastWriteTime);
                        on_file(&PathBuf::from(path).join(name), size, modified);
                    }
                }

                if info.NextEntryOffset == 0 {
                    break;
                }
                offset += info.NextEntryOffset as usize;
            }
        }

        // Parent buffer is fully parsed; it is now safe to reuse it while descending.
        for (child_path, id) in subdirs {
            if cancelled.load(Ordering::Relaxed) {
                return Err("Scan cancelled".into());
            }
            if *walked >= super::MAX_WALK_ENTRIES {
                return Ok(());
            }
            if id != 0 && !visited.insert(id) {
                continue;
            }

            let child = open_dir(&child_path)?;
            let result = walk_dir(
                child,
                &child_path,
                buffer,
                walked,
                cancelled,
                visited,
                on_file,
            );
            // SAFETY: `child` was opened above and is still valid.
            unsafe {
                CloseHandle(child);
            }
            result?;
        }

        Ok(())
    }

    fn filetime_to_systemtime(ft: i64) -> SystemTime {
        let ticks = ft as u64;
        if ticks >= EPOCH_DIFF_TICKS {
            let since_epoch = ticks - EPOCH_DIFF_TICKS;
            // 100 ns units → nanoseconds, preserving sub-second precision.
            SystemTime::UNIX_EPOCH + Duration::from_nanos(since_epoch * 100)
        } else {
            SystemTime::UNIX_EPOCH
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    const KIB: u64 = 64 * 1024; // 64 KiB is a whole-number multiple of any common cluster size.

    fn dir_with_files(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cachebin-size-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn total_of(path: &std::path::Path) -> u64 {
        dir_size(path, &AtomicBool::new(false))
            .expect("dir_size should succeed")
            .0
    }

    #[test]
    fn counts_files_across_nested_directories() {
        let root = dir_with_files("nested");
        std::fs::create_dir_all(root.join("a").join("b")).unwrap();
        std::fs::write(root.join("top.bin"), vec![0u8; 3 * KIB as usize]).unwrap();
        std::fs::write(root.join("a").join("one.bin"), vec![0u8; KIB as usize]).unwrap();
        std::fs::write(root.join("a").join("b").join("two.bin"), vec![0u8; 2 * KIB as usize]).unwrap();

        assert_eq!(total_of(&root), 6 * KIB);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn counts_dotfiles_and_regular_files() {
        let root = dir_with_files("dotfiles");
        std::fs::write(root.join(".cachefile"), vec![0u8; KIB as usize]).unwrap();
        std::fs::write(root.join("normal.bin"), vec![0u8; KIB as usize]).unwrap();

        assert_eq!(total_of(&root), 2 * KIB);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn handles_unicode_file_names() {
        let root = dir_with_files("unicode");
        std::fs::write(root.join("β-文件.bin"), vec![0u8; KIB as usize]).unwrap();

        assert_eq!(total_of(&root), KIB);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn spans_multiple_directory_queries() {
        // Enough 64 KiB entries to overflow the 64 KiB query buffer several times over.
        let root = dir_with_files("many");
        let count = 800usize;
        for index in 0..count {
            std::fs::write(root.join(format!("f{index:04}.bin")), vec![0u8; KIB as usize]).unwrap();
        }

        assert_eq!(total_of(&root), (count as u64) * KIB);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn counts_zero_byte_files_without_error() {
        let root = dir_with_files("zero");
        std::fs::write(root.join("empty.bin"), Vec::new()).unwrap();
        std::fs::write(root.join("one.bin"), vec![0u8; KIB as usize]).unwrap();

        assert_eq!(total_of(&root), KIB);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn reports_newest_modified_time() {
        let root = dir_with_files("newest");
        std::fs::write(root.join("a.bin"), vec![0u8; KIB as usize]).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(root.join("b.bin"), vec![0u8; KIB as usize]).unwrap();

        let (_, newest) = dir_size(&root, &AtomicBool::new(false)).expect("dir_size");
        assert!(newest.is_some());

        let b_modified = std::fs::metadata(root.join("b.bin"))
            .and_then(|meta| meta.modified())
            .unwrap();
        assert!(newest.unwrap() >= b_modified);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn honours_cancellation_mid_scan() {
        let root = dir_with_files("cancel");
        std::fs::write(root.join("a.bin"), vec![0u8; KIB as usize]).unwrap();

        let cancelled = AtomicBool::new(true);
        assert_eq!(dir_size(&root, &cancelled).err().as_deref(), Some("Scan cancelled"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn collects_files_above_threshold() {
        let root = dir_with_files("large");
        std::fs::write(root.join("big.bin"), vec![0u8; 200 * KIB as usize]).unwrap();
        std::fs::write(root.join("small.bin"), vec![0u8; KIB as usize]).unwrap();

        let hits = large_files(&root, 100 * KIB, 10, &AtomicBool::new(false)).expect("scan");

        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].size, 200 * KIB);
        assert!(hits[0].path.ends_with("big.bin"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn large_files_respects_limit() {
        let root = dir_with_files("large-limit");
        for index in 0..10 {
            std::fs::write(root.join(format!("f{index}.bin")), vec![0u8; 200 * KIB as usize]).unwrap();
        }

        let hits = large_files(&root, 100 * KIB, 3, &AtomicBool::new(false)).expect("scan");

        assert_eq!(hits.len(), 3);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn large_files_honours_cancellation() {
        let root = dir_with_files("large-cancel");
        std::fs::write(root.join("big.bin"), vec![0u8; 200 * KIB as usize]).unwrap();

        let cancelled = AtomicBool::new(true);
        assert_eq!(
            large_files(&root, 100 * KIB, 10, &cancelled).err().as_deref(),
            Some("Scan cancelled")
        );

        let _ = std::fs::remove_dir_all(&root);
    }
}
