use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

use crate::models::{Environment, LargeFile, WslDistro};
use crate::projects;
use crate::size;
use crate::wsl;

const MIN_LARGE_FILE_BYTES: u64 = 100 * 1024 * 1024;
const MAX_RESULTS: usize = 100;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanStatus {
    Scanning,
    Ready,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedLargeFiles {
    #[serde(default = "default_scan_status")]
    pub status: ScanStatus,
    pub scanned_at: String,
    pub files: Vec<LargeFile>,
}

fn default_scan_status() -> ScanStatus {
    ScanStatus::Ready
}

pub fn load_cached<R: Runtime>(app: &AppHandle<R>) -> Option<CachedLargeFiles> {
    cache_path(app).and_then(|path| load_cached_from(&path))
}

fn load_cached_from(path: &Path) -> Option<CachedLargeFiles> {
    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
}

/// Marks the cached scan as in progress, keeping any previous results so they survive a crash.
pub fn mark_scanning<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let Some(path) = cache_path(app) else {
        return Ok(());
    };
    let existing = load_cached_from(&path);
    write_cache(
        &path,
        &CachedLargeFiles {
            status: ScanStatus::Scanning,
            scanned_at: existing
                .as_ref()
                .map(|cached| cached.scanned_at.clone())
                .unwrap_or_else(crate::time::now_rfc3339),
            files: existing.map(|cached| cached.files).unwrap_or_default(),
        },
    )
}

/// Persists a finished scan. Overwrites the in-progress marker written by [`mark_scanning`].
pub fn save_cached<R: Runtime>(app: &AppHandle<R>, files: &[LargeFile]) -> Result<(), String> {
    let Some(path) = cache_path(app) else {
        return Ok(());
    };
    write_cache(
        &path,
        &CachedLargeFiles {
            status: ScanStatus::Ready,
            scanned_at: crate::time::now_rfc3339(),
            files: files.to_vec(),
        },
    )
}

/// Clears a stale in-progress marker left behind by a crash or app restart (no scan can be
/// running on process startup). Previous results are preserved.
pub fn reset_stale_scanning<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let Some(path) = cache_path(app) else {
        return Ok(());
    };
    let Some(mut cached) = load_cached_from(&path) else {
        return Ok(());
    };
    if cached.status == ScanStatus::Scanning {
        cached.status = ScanStatus::Ready;
        write_cache(&path, &cached)?;
    }
    Ok(())
}

fn write_cache(path: &Path, cached: &CachedLargeFiles) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|err| err.to_string())?;
    }
    let json = serde_json::to_string_pretty(cached).map_err(|err| err.to_string())?;
    fs::write(path, json).map_err(|err| err.to_string())
}

fn cache_path<R: Runtime>(app: &AppHandle<R>) -> Option<PathBuf> {
    app.path().app_data_dir().ok().map(|dir| dir.join("large-files-cache.json"))
}

/// Scans the largest files under the workspace roots (project folders + each running WSL distro's
/// home directories), returning files at least [`MIN_LARGE_FILE_BYTES`] in size, sorted by size.
pub fn scan(
    env: &HashMap<String, String>,
    distros: &[WslDistro],
    dirs: &[String],
    cancelled: &AtomicBool,
) -> Result<Vec<LargeFile>, String> {
    scan_with(env, distros, dirs, MIN_LARGE_FILE_BYTES, MAX_RESULTS, cancelled)
}

fn scan_with(
    env: &HashMap<String, String>,
    distros: &[WslDistro],
    dirs: &[String],
    min_bytes: u64,
    limit: usize,
    cancelled: &AtomicBool,
) -> Result<Vec<LargeFile>, String> {
    let mut files = Vec::new();

    for root in projects::project_scan_roots(env, dirs) {
        collect_from(&root, &Environment::windows(), min_bytes, limit, &mut files, cancelled)?;
    }

    for distro in distros {
        if cancelled.load(Ordering::Relaxed) {
            return Err("Scan cancelled".into());
        }
        let environment = Environment::wsl(&distro.name);
        for home in wsl::home_dirs(distro) {
            let root = PathBuf::from(&distro.root).join(home.trim_start_matches('/'));
            collect_from(&root, &environment, min_bytes, limit, &mut files, cancelled)?;
        }
    }

    files.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
    files.truncate(limit);

    Ok(files)
}

fn collect_from(
    root: &std::path::Path,
    environment: &Environment,
    min_bytes: u64,
    limit: usize,
    files: &mut Vec<LargeFile>,
    cancelled: &AtomicBool,
) -> Result<(), String> {
    let hits = match size::large_files(root, min_bytes, limit, cancelled) {
        Ok(hits) => hits,
        // An unreadable or missing root (permissions, a stopped distro's /root, ...) must not
        // fail the whole scan; only an explicit cancellation aborts it.
        Err(err) => {
            if cancelled.load(Ordering::Relaxed) {
                return Err(err);
            }
            return Ok(());
        }
    };
    for hit in hits {
        let name = hit
            .path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();
        if name.is_empty() {
            continue;
        }

        files.push(LargeFile {
            id: large_file_id(&hit.path),
            name,
            path: hit.path.to_string_lossy().to_string(),
            environment: environment.clone(),
            size_bytes: hit.size,
            file_type: file_type(&hit.path),
            last_modified: hit.modified.map(crate::time::format_time),
        });
    }
    Ok(())
}

/// Human-friendly type label derived from the file extension. Known families get a stable label;
/// numeric and unrecognized extensions collapse into `Other` instead of leaking a raw extension.
fn file_type(path: &std::path::Path) -> String {
    let Some(ext) = path.extension().map(|ext| ext.to_string_lossy().to_lowercase()) else {
        return "File".into();
    };
    if ext.is_empty() || ext.chars().all(|ch| ch.is_ascii_digit()) {
        return "Other".into();
    }

    match ext.as_str() {
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "iso" | "dmg" | "cab" => "Archive",
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "webm" | "flv" => "Video",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => "Audio",
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp" | "tiff" | "heic" => "Image",
        "db" | "sqlite" | "sqlite3" | "mdb" | "dmp" => "Database",
        "exe" | "dll" | "msi" | "bin" | "deb" | "rpm" | "apk" | "appx" => "Executable",
        "pdb" => "Binary",
        "log" => "Log",
        "ts" | "tsx" | "js" | "jsx" | "rs" | "go" | "java" | "kt" | "kts" | "c" | "h" | "cc"
        | "cpp" | "cxx" | "cs" | "py" | "rb" | "php" | "swift" | "dart" | "zig" | "hs" | "ex"
        | "exs" | "vue" | "svelte" | "sh" | "bash" | "zsh" | "fish" | "json" | "yaml" | "yml"
        | "toml" | "xml" | "sql" | "proto" | "graphql" => "Code",
        "pdf" | "doc" | "docx" | "txt" | "md" | "rtf" | "odt" | "epub" | "ppt" | "pptx" | "xls"
        | "xlsx" | "csv" => "Docs",
        "ttf" | "otf" | "woff" | "woff2" => "Font",
        _ => "Other",
    }
    .to_string()
}

fn large_file_id(path: &std::path::Path) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.hash(&mut hasher);
    format!("large-{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    const KIB: u64 = 64 * 1024;

    fn temp_home(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cachebin-large-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Projects").join("demo")).unwrap();
        dir
    }

    fn env_with_home(home: &std::path::Path) -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("USERPROFILE".into(), home.to_string_lossy().into_owned());
        env.insert("HOME".into(), home.to_string_lossy().into_owned());
        env
    }

    fn default_dirs() -> Vec<String> {
        crate::settings::SCAN_DIR_OPTIONS.iter().map(|dir| dir.to_string()).collect()
    }

    #[test]
    fn returns_only_files_above_threshold_sorted_by_size() {
        let home = temp_home("scan");
        std::fs::write(home.join("Projects").join("demo").join("big.iso"), vec![0u8; 300 * KIB as usize]).unwrap();
        std::fs::write(home.join("Projects").join("demo").join("bigger.zip"), vec![0u8; 500 * KIB as usize]).unwrap();
        std::fs::write(home.join("Projects").join("demo").join("small.log"), vec![0u8; KIB as usize]).unwrap();

        let env = env_with_home(&home);
        let files = scan_with(&env, &[], &default_dirs(), 100 * KIB, 10, &AtomicBool::new(false)).expect("scan");

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].name, "bigger.zip");
        assert_eq!(files[1].name, "big.iso");
        assert_eq!(files[0].file_type, "Archive");
        assert!(files[0].path.contains("Projects"));
        assert!(files[0].last_modified.is_some());

        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn returns_empty_when_no_files_meet_threshold() {
        let home = temp_home("none");
        std::fs::write(home.join("Projects").join("demo").join("small.log"), vec![0u8; KIB as usize]).unwrap();

        let env = env_with_home(&home);
        let files = scan_with(&env, &[], &default_dirs(), 100 * KIB, 10, &AtomicBool::new(false)).expect("scan");

        assert!(files.is_empty());

        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn honours_cancellation() {
        let home = temp_home("cancel");
        std::fs::write(home.join("Projects").join("demo").join("big.iso"), vec![0u8; 300 * KIB as usize]).unwrap();

        let env = env_with_home(&home);
        let cancelled = AtomicBool::new(true);
        assert!(scan_with(&env, &[], &default_dirs(), 100 * KIB, 10, &cancelled).is_err());

        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn collect_from_skips_unopenable_root() {
        let missing = std::env::temp_dir().join(format!("cachebin-large-missing-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&missing);

        let mut files = Vec::new();
        collect_from(
            &missing,
            &Environment::windows(),
            100 * KIB,
            10,
            &mut files,
            &AtomicBool::new(false),
        )
        .expect("unopenable root should be skipped, not fail");

        assert!(files.is_empty());
    }

    #[test]
    fn scan_cache_round_trips_through_disk() {
        let path = std::env::temp_dir().join(format!("cachebin-large-cache-{}.json", std::process::id()));
        let cached = CachedLargeFiles {
            status: ScanStatus::Ready,
            scanned_at: crate::time::now_rfc3339(),
            files: vec![LargeFile {
                id: "large-1".into(),
                name: "big.bin".into(),
                path: r"C:\Projects\big.bin".into(),
                environment: Environment::windows(),
                size_bytes: 1024,
                file_type: "BIN".into(),
                last_modified: None,
            }],
        };

        write_cache(&path, &cached).expect("save");
        let back = load_cached_from(&path).expect("cache should be present");

        assert_eq!(back.status, ScanStatus::Ready);
        assert_eq!(back.files.len(), 1);
        assert_eq!(back.files[0].name, "big.bin");
        assert!(!back.scanned_at.is_empty());

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn scan_cache_missing_file_returns_none() {
        let path = std::env::temp_dir().join(format!("cachebin-large-cache-missing-{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);
        assert!(load_cached_from(&path).is_none());
    }

    #[test]
    fn scan_status_round_trips_through_disk() {
        let path = std::env::temp_dir().join(format!("cachebin-large-cache-status-{}.json", std::process::id()));

        write_cache(
            &path,
            &CachedLargeFiles {
                status: ScanStatus::Scanning,
                scanned_at: crate::time::now_rfc3339(),
                files: Vec::new(),
            },
        )
        .expect("mark scanning");

        let marked = load_cached_from(&path).expect("cache");
        assert_eq!(marked.status, ScanStatus::Scanning);
        assert!(marked.files.is_empty());

        let mut ready = marked;
        ready.status = ScanStatus::Ready;
        write_cache(&path, &ready).expect("mark ready");
        assert_eq!(load_cached_from(&path).expect("cache").status, ScanStatus::Ready);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn classifies_file_types() {
        assert_eq!(file_type(std::path::Path::new("a.tar.gz")), "Archive");
        assert_eq!(file_type(std::path::Path::new("movie.mkv")), "Video");
        assert_eq!(file_type(std::path::Path::new("photo.png")), "Image");
        assert_eq!(file_type(std::path::Path::new("data.db")), "Database");
        assert_eq!(file_type(std::path::Path::new("setup.exe")), "Executable");
        assert_eq!(file_type(std::path::Path::new("archive.7z")), "Archive");
        assert_eq!(file_type(std::path::Path::new("notes.md")), "Docs");
        assert_eq!(file_type(std::path::Path::new("lib.rs")), "Code");
        assert_eq!(file_type(std::path::Path::new("font.ttf")), "Font");
        assert_eq!(file_type(std::path::Path::new("noext")), "File");
    }

    #[test]
    fn collapses_numeric_and_unknown_extensions_to_other() {
        assert_eq!(file_type(std::path::Path::new("page.1")), "Other");
        assert_eq!(file_type(std::path::Path::new("f.67")), "Other");
        assert_eq!(file_type(std::path::Path::new("static.a")), "Other");
        assert_eq!(file_type(std::path::Path::new("table.sst")), "Other");
    }
}
