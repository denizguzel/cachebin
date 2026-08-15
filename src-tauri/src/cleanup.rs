use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupProgress {
    pub current: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrashFailure {
    pub path: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrashReport {
    pub moved: Vec<String>,
    pub failed: Vec<TrashFailure>,
}

pub fn move_to_trash(paths: &[String], on_progress: &mut dyn FnMut(u64, u64)) -> TrashReport {
    let total = paths.len() as u64;
    let mut report = TrashReport { moved: Vec::new(), failed: Vec::new() };

    for (index, path) in paths.iter().enumerate() {
        on_progress(index as u64, total);
        match move_one(path) {
            Ok(()) => report.moved.push(path.clone()),
            Err(error) => report.failed.push(TrashFailure { path: path.clone(), error }),
        }
    }
    on_progress(total, total);

    report
}

fn move_one(path: &str) -> Result<(), String> {
    if let Some((distro, linux_path)) = wsl_path_info(path) {
        move_wsl_to_trash(&distro, &linux_path)
    } else {
        let path = Path::new(path);
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path.display()));
        }
        trash::delete(path).map_err(|err| format!("Failed to move {} to Trash: {err}", path.display()))
    }
}

/// Translates `\\wsl.localhost\<distro>\...` (and the `\\wsl$\...` alias) into the
/// distro name and the Linux path it maps to.
fn wsl_path_info(path: &str) -> Option<(String, String)> {
    let lower = path.to_ascii_lowercase();
    for prefix in [r"\\wsl.localhost\", r"\\wsl$\"] {
        if lower.starts_with(prefix) {
            let rest = &path[prefix.len()..];
            let (distro, rest) = rest.split_once('\\').or_else(|| rest.split_once('/'))?;
            if distro.is_empty() || rest.is_empty() {
                return None;
            }
            let linux_path = format!("/{}", rest.replace('\\', "/"));
            return Some((distro.to_string(), linux_path));
        }
    }
    None
}

/// Moves a path inside a WSL distro to that distro's XDG Trash (recoverable via the
/// distro's file manager). Runs a small POSIX script through `wsl.exe -d <distro> sh -s`;
/// the script is piped over stdin and the target path is passed base64-encoded so no
/// shell quoting ever crosses the Windows command line.
fn move_wsl_to_trash(distro: &str, linux_path: &str) -> Result<(), String> {
    let script = wsl_trash_script(linux_path);

    let mut child = Command::new("wsl.exe")
        .args(["-d", distro, "--", "sh", "-s", "cachebin-trash"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("failed to start wsl.exe for {distro}: {err}"))?;

    child
        .stdin
        .take()
        .expect("stdin was piped")
        .write_all(script.as_bytes())
        .map_err(|err| format!("failed to write trash script: {err}"))?;

    let output = child
        .wait_with_output()
        .map_err(|err| format!("wsl.exe failed: {err}"))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            Err(format!("wsl.exe exited with {}", output.status))
        } else {
            Err(stderr)
        }
    }
}

const WSL_TRASH_SCRIPT: &str = r#"
set -eu
TARGET="$(printf '%s' '__PATH_B64__' | base64 -d)"
TRASH_ROOT="${XDG_DATA_HOME:-$HOME/.local/share}/Trash"
FILES_DIR="$TRASH_ROOT/files"
INFO_DIR="$TRASH_ROOT/info"
mkdir -p "$FILES_DIR" "$INFO_DIR"
NAME="$(basename -- "$TARGET")"
DEST="$FILES_DIR/$NAME"
if [ "$DEST" = "$TARGET" ]; then
  echo "target is already inside the Trash directory" >&2
  exit 1
fi
I=1
while [ -e "$DEST" ]; do
  DEST="$FILES_DIR/$NAME.$I"
  I=$((I+1))
done
mv -- "$TARGET" "$DEST"
INFO_NAME="$(basename -- "$DEST")"
printf '[Trash Info]\nPath=%s\nDeletionDate=%s\n' "$TARGET" "$(date +%Y-%m-%dT%H:%M:%S)" > "$INFO_DIR/$INFO_NAME.trashinfo"
"#;

fn wsl_trash_script(linux_path: &str) -> String {
    WSL_TRASH_SCRIPT.replace("__PATH_B64__", &BASE64.encode(linux_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("cachebin-cleanup-{name}-{}.txt", std::process::id()))
    }

    fn noop_progress() -> impl FnMut(u64, u64) {
        |_, _| {}
    }

    #[test]
    fn wsl_path_info_parses_wsl_localhost() {
        let (distro, linux_path) = wsl_path_info(r"\\wsl.localhost\Ubuntu\home\dev\.cache\npm").expect("parse");
        assert_eq!(distro, "Ubuntu");
        assert_eq!(linux_path, "/home/dev/.cache/npm");
    }

    #[test]
    fn wsl_path_info_parses_wsl_dollar_alias() {
        let (distro, linux_path) = wsl_path_info(r"\\wsl$\Ubuntu\root\foo").expect("parse");
        assert_eq!(distro, "Ubuntu");
        assert_eq!(linux_path, "/root/foo");
    }

    #[test]
    fn wsl_path_info_ignores_local_and_other_unc_paths() {
        assert!(wsl_path_info(r"C:\Users\dev\.cache").is_none());
        assert!(wsl_path_info(r"\\server\share\foo").is_none());
        assert!(wsl_path_info(r"\\wsl.localhost\Ubuntu").is_none());
    }

    #[test]
    fn move_to_trash_reports_missing_path_as_failure() {
        let missing = temp_path("missing");
        let _ = std::fs::remove_file(&missing);

        let report = move_to_trash(&[missing.to_string_lossy().into_owned()], &mut noop_progress());
        assert!(report.moved.is_empty());
        assert_eq!(report.failed.len(), 1);
        assert!(report.failed[0].error.contains("does not exist"));
    }

    #[test]
    fn move_to_trash_sends_existing_file_to_trash_and_reports_partial_failure() {
        let missing = temp_path("missing");
        let existing = temp_path("existing");
        let _ = std::fs::remove_file(&missing);
        std::fs::write(&existing, b"content").unwrap();

        let report = move_to_trash(
            &[missing.to_string_lossy().into_owned(), existing.to_string_lossy().into_owned()],
            &mut noop_progress(),
        );

        assert_eq!(report.moved.len(), 1, "existing file should move to the system Trash");
        assert!(report.moved[0].contains("existing"));
        assert_eq!(report.failed.len(), 1, "missing file should be reported as failed");
        assert!(!existing.exists(), "file should no longer be at its original location");
    }

    #[test]
    fn move_to_trash_reports_progress() {
        let missing = temp_path("missing");
        let _ = std::fs::remove_file(&missing);

        let mut progress = Vec::new();
        let _ = move_to_trash(&[missing.to_string_lossy().into_owned()], &mut |current, total| {
            progress.push((current, total));
        });

        assert_eq!(progress, vec![(0, 1), (1, 1)]);
    }

    #[test]
    fn move_to_trash_empty_paths_returns_empty_report() {
        let report = move_to_trash(&[], &mut noop_progress());
        assert!(report.moved.is_empty());
        assert!(report.failed.is_empty());
    }

    #[test]
    fn wsl_trash_script_embeds_base64_path_without_leaking_it() {
        let path = "/home/dev/.cache/npm";
        let script = wsl_trash_script(path);

        let encoded = BASE64.encode(path);
        assert!(script.contains(&encoded));
        // The raw path must never appear: it would be parsed by the Windows command line.
        assert!(!script.contains(path));
        assert!(script.contains("mkdir -p \"$FILES_DIR\" \"$INFO_DIR\""));
        assert!(script.contains(".trashinfo"));
        assert!(script.contains("base64 -d"));
        assert!(script.contains("while [ -e \"$DEST\" ]"));
    }

    #[test]
    fn wsl_trash_script_handles_quoting_sensitive_paths() {
        let path = "/home/dev/my folder/'quoted'/$weird";
        let script = wsl_trash_script(path);

        let encoded = BASE64.encode(path);
        assert!(script.contains(&encoded));
        assert!(!script.contains("my folder"));
        assert!(!script.contains("'quoted'"));
        assert!(!script.contains("$weird"));
    }
}
