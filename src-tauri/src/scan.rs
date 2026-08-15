use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::models::{CacheEntry, ScanResult};
use crate::scanner::{self, Candidate};
use crate::size;
use crate::wsl;

pub struct ScanToken(AtomicBool);

impl Default for ScanToken {
    fn default() -> Self {
        Self(AtomicBool::new(false))
    }
}

impl ScanToken {
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }

    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn reset(&self) {
        self.0.store(false, Ordering::Relaxed);
    }

    pub fn cancelled_flag(&self) -> &AtomicBool {
        &self.0
    }
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressPayload {
    pub phase: String,
    pub environment: Option<String>,
    pub current: u64,
    pub total: u64,
}

pub fn cancel<R: Runtime>(app: &AppHandle<R>) {
    app.state::<ScanToken>().cancel();
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedScan {
    pub scanned_at: String,
    pub result: ScanResult,
}

pub fn load_cached<R: Runtime>(app: &AppHandle<R>) -> Option<CachedScan> {
    cache_path(app).and_then(|path| load_cached_from(&path))
}

fn load_cached_from(path: &Path) -> Option<CachedScan> {
    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
}

pub fn save_cached<R: Runtime>(app: &AppHandle<R>, result: &ScanResult) -> Result<(), String> {
    let Some(path) = cache_path(app) else {
        return Ok(());
    };
    save_cached_to(&path, result)
}

fn save_cached_to(path: &Path, result: &ScanResult) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|err| err.to_string())?;
    }
    let cached = CachedScan {
        scanned_at: crate::time::now_rfc3339(),
        result: result.clone(),
    };
    let json = serde_json::to_string_pretty(&cached).map_err(|err| err.to_string())?;
    fs::write(path, json).map_err(|err| err.to_string())
}

fn cache_path<R: Runtime>(app: &AppHandle<R>) -> Option<PathBuf> {
    app.path().app_data_dir().ok().map(|dir| dir.join("scan-cache.json"))
}

pub fn run<R: Runtime>(app: AppHandle<R>) -> Result<ScanResult, String> {
    let token = app.state::<ScanToken>();
    token.reset();

    let settings = app.state::<crate::settings::SettingsState>().0.lock().unwrap().clone();

    let env = std::env::vars().collect::<HashMap<_, _>>();
    let mut entries = Vec::new();

    let windows = scanner::windows_candidates(&env);
    scan_candidates(&app, &mut entries, &windows, "windows", None, &token)?;

    let distros = wsl::discover();
    let distros = distros
        .into_iter()
        .filter(|distro| distro.version == 2 && distro.state.eq_ignore_ascii_case("running"))
        .filter(|distro| !settings.disabled_distros.contains(&distro.name))
        .collect::<Vec<_>>();

    for distro in &distros {
        if token.is_cancelled() {
            return Err("Scan cancelled".into());
        }
        let candidates = scanner::wsl_candidates(distro);
        scan_candidates(&app, &mut entries, &candidates, "wsl", Some(&distro.name), &token)?;
    }

    let total_bytes = entries.iter().map(|entry| entry.size_bytes).sum();
    let location_count = entries.len();

    Ok(ScanResult {
        entries,
        total_bytes,
        location_count,
    })
}

fn scan_candidates<R: Runtime>(
    app: &AppHandle<R>,
    entries: &mut Vec<CacheEntry>,
    candidates: &[Candidate],
    phase: &str,
    environment: Option<&str>,
    token: &ScanToken,
) -> Result<(), String> {
    let found = collect_candidates(candidates, token, &mut |current, total| {
        emit(app, phase, environment, current, total);
    })?;
    entries.extend(found);
    Ok(())
}

fn collect_candidates(
    candidates: &[Candidate],
    token: &ScanToken,
    on_progress: &mut dyn FnMut(u64, u64),
) -> Result<Vec<CacheEntry>, String> {
    let mut found = Vec::new();
    on_progress(0, candidates.len() as u64);

    for (index, candidate) in candidates.iter().enumerate() {
        if token.is_cancelled() {
            return Err("Scan cancelled".into());
        }

        if let Some(entry) = build_entry(candidate, token)? {
            found.push(entry);
        }

        on_progress((index + 1) as u64, candidates.len() as u64);
    }

    Ok(found)
}

fn build_entry(candidate: &Candidate, token: &ScanToken) -> Result<Option<CacheEntry>, String> {
    if !candidate.path.is_dir() {
        return Ok(None);
    }

    let (size_bytes, last_modified) = size::dir_size(&candidate.path, token.cancelled_flag())?;
    if size_bytes == 0 {
        return Ok(None);
    }

    Ok(Some(CacheEntry {
        id: format!("{}-{}", candidate.environment.id(), candidate.location.id),
        category: candidate.location.category.to_string(),
        name: candidate
            .name
            .clone()
            .unwrap_or_else(|| candidate.location.name.to_string()),
        path: candidate.path.to_string_lossy().to_string(),
        environment: candidate.environment.clone(),
        size_bytes,
        risk: candidate.location.risk,
        description: candidate.location.description.to_string(),
        last_modified: last_modified.map(crate::time::format_time),
        rebuildable: candidate.location.rebuildable,
    }))
}

fn emit<R: Runtime>(app: &AppHandle<R>, phase: &str, environment: Option<&str>, current: u64, total: u64) {
    let _ = app.emit(
        "scan://progress",
        ProgressPayload {
            phase: phase.to_string(),
            environment: environment.map(str::to_string),
            current,
            total,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Environment, RiskLevel};
    use std::path::PathBuf;

    static TEST_LOCATION: scanner::CacheLocation = scanner::CacheLocation {
        id: "test-cache",
        category: "Test",
        name: "Test cache",
        description: "Unit test location",
        risk: RiskLevel::Safe,
        rebuildable: true,
    };

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("cachebin-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        dir
    }

    fn candidate(path: PathBuf, name: Option<String>) -> Candidate {
        Candidate {
            location: &TEST_LOCATION,
            path,
            name,
            environment: Environment::windows(),
        }
    }

    #[test]
    fn scan_token_flags() {
        let token = ScanToken::default();
        assert!(!token.is_cancelled());
        token.cancel();
        assert!(token.is_cancelled());
        token.reset();
        assert!(!token.is_cancelled());
    }

    #[test]
    fn dir_size_sums_nested_files() {
        let dir = temp_dir("size");
        std::fs::write(dir.join("a.txt"), vec![0u8; 1024 * 1024]).unwrap();
        std::fs::write(dir.join("sub").join("b.txt"), vec![0u8; 2 * 1024 * 1024]).unwrap();

        // Multi-megabyte files allocate exactly their logical size on standard filesystems,
        // keeping the assertion independent of cluster size.
        let (total, newest) = crate::size::dir_size(&dir, &ScanToken::default().cancelled_flag()).expect("size");
        assert_eq!(total, 3 * 1024 * 1024);
        assert!(newest.is_some());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn dir_size_is_zero_for_empty_dir() {
        let dir = temp_dir("empty");
        let (total, _newest) = crate::size::dir_size(&dir, &ScanToken::default().cancelled_flag()).expect("size");
        assert_eq!(total, 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn dir_size_is_hardlink_aware() {
        let dir = temp_dir("hardlink");
        let file = dir.join("a.txt");
        std::fs::write(&file, vec![0u8; 1024 * 1024]).unwrap();
        std::fs::hard_link(&file, dir.join("b.txt")).unwrap();

        // Two names share one 1 MiB inode. Unix counts the inode once; Windows std cannot report
        // link counts cheaply, so each name is counted in full.
        let (total, _newest) = crate::size::dir_size(&dir, &ScanToken::default().cancelled_flag()).expect("size");
        #[cfg(unix)]
        assert_eq!(total, 1024 * 1024);
        #[cfg(not(unix))]
        assert_eq!(total, 2 * 1024 * 1024);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn dir_size_honours_cancellation() {
        let dir = temp_dir("cancel-size");
        std::fs::write(dir.join("a.txt"), vec![0u8; 10]).unwrap();

        let token = ScanToken::default();
        token.cancel();

        assert_eq!(
            crate::size::dir_size(&dir, token.cancelled_flag()).err().as_deref(),
            Some("Scan cancelled")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn build_entry_ignores_missing_path() {
        let candidate = candidate(std::env::temp_dir().join("cachebin-does-not-exist"), None);
        assert!(build_entry(&candidate, &ScanToken::default()).expect("entry").is_none());
    }

    #[test]
    fn build_entry_creates_entry_from_directory() {
        let dir = temp_dir("entry");
        std::fs::write(dir.join("a.txt"), vec![0u8; 120]).unwrap();

        let entry = build_entry(&candidate(dir.clone(), Some("Custom name".into())), &ScanToken::default())
            .expect("entry should be created")
            .expect("entry should be present");

        assert_eq!(entry.id, "windows-test-cache");
        assert_eq!(entry.category, "Test");
        assert_eq!(entry.name, "Custom name");
        assert_eq!(entry.size_bytes, 120);
        assert_eq!(entry.risk, RiskLevel::Safe);
        assert!(entry.rebuildable);
        assert!(entry.last_modified.is_some());
        assert_eq!(entry.environment, Environment::windows());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn build_entry_skips_empty_directory() {
        let dir = temp_dir("empty-entry");
        assert!(build_entry(&candidate(dir.clone(), None), &ScanToken::default())
            .expect("entry")
            .is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn collect_candidates_reports_progress() {
        let token = ScanToken::default();
        let dir = temp_dir("progress");
        std::fs::write(dir.join("a.txt"), vec![0u8; 10]).unwrap();

        let candidates = vec![
            candidate(dir.clone(), None),
            candidate(std::env::temp_dir().join("cachebin-missing"), None),
        ];

        let mut progress = Vec::new();
        let entries = collect_candidates(&candidates, &token, &mut |current, total| {
            progress.push((current, total));
        })
        .unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(progress, vec![(0, 2), (1, 2), (2, 2)]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn collect_candidates_honours_cancellation() {
        let token = ScanToken::default();
        token.cancel();

        let dir = temp_dir("cancel");
        let candidates = vec![candidate(dir.clone(), None)];

        let result = collect_candidates(&candidates, &token, &mut |_, _| {});
        assert_eq!(result.err().as_deref(), Some("Scan cancelled"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn scan_cache_round_trips_through_disk() {
        let path = std::env::temp_dir().join(format!("cachebin-scan-cache-{}.json", std::process::id()));
        let result = ScanResult {
            entries: vec![],
            total_bytes: 4096,
            location_count: 2,
        };

        save_cached_to(&path, &result).expect("save");
        let cached = load_cached_from(&path).expect("cache should be present");

        assert_eq!(cached.result.total_bytes, 4096);
        assert_eq!(cached.result.location_count, 2);
        assert!(!cached.scanned_at.is_empty());

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn scan_cache_missing_file_returns_none() {
        let path = std::env::temp_dir().join(format!("cachebin-scan-cache-missing-{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);
        assert!(load_cached_from(&path).is_none());
    }
}
