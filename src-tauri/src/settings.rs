use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tauri::{AppHandle, Manager, Runtime};

/// Windows home subdirectories offered as scan locations for projects and large files.
pub const SCAN_DIR_OPTIONS: &[&str] = &[
    "Documents", "Desktop", "Projects", "Code", "repos", "src", "source", "workspace", "Developer",
];

pub const RISK_FILTER_OPTIONS: &[&str] = &["all", "safe", "caution", "risky"];

#[derive(Serialize, Deserialize, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub zoom: f64,
    pub scan_dirs: Vec<String>,
    pub disabled_distros: Vec<String>,
    pub default_risk_filter: String,
    pub auto_scan_on_startup: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            scan_dirs: SCAN_DIR_OPTIONS.iter().map(|dir| dir.to_string()).collect(),
            disabled_distros: Vec::new(),
            default_risk_filter: "all".into(),
            auto_scan_on_startup: false,
        }
    }
}

/// Normalizes user-supplied settings: keeps only known scan dirs (deduplicated) and only valid
/// risk filter values.
pub fn sanitize(settings: &Settings) -> Settings {
    let mut cleaned = settings.clone();

    let mut scan_dirs: Vec<String> = settings
        .scan_dirs
        .iter()
        .filter(|dir| SCAN_DIR_OPTIONS.contains(&dir.as_str()))
        .cloned()
        .collect();
    scan_dirs.sort();
    scan_dirs.dedup();
    cleaned.scan_dirs = scan_dirs;

    if !RISK_FILTER_OPTIONS.contains(&cleaned.default_risk_filter.as_str()) {
        cleaned.default_risk_filter = "all".into();
    }

    cleaned
}

impl Settings {
    pub fn load<R: Runtime>(app: &AppHandle<R>) -> Self {
        Self::path(app).map(|path| Self::load_from(&path)).unwrap_or_default()
    }

    pub fn save<R: Runtime>(&self, app: &AppHandle<R>) {
        if let Some(path) = Self::path(app) {
            self.save_to(&path);
        }
    }

    fn load_from(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_default()
    }

    fn save_to(&self, path: &Path) {
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    fn path<R: Runtime>(app: &AppHandle<R>) -> Option<PathBuf> {
        app.path()
            .app_data_dir()
            .ok()
            .map(|dir| dir.join("settings.json"))
    }
}

pub struct SettingsState(pub Mutex<Settings>);

pub fn init<R: Runtime>(app: &AppHandle<R>) -> Settings {
    let settings = Settings::load(app);
    app.manage(SettingsState(Mutex::new(settings.clone())));
    settings
}

pub fn update_zoom<R: Runtime>(app: &AppHandle<R>, value: f64) {
    let state = app.state::<SettingsState>();
    let mut settings = state.0.lock().unwrap();
    settings.zoom = value;
    settings.save(app);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_settings() -> Settings {
        Settings {
            zoom: 1.5,
            scan_dirs: vec!["Documents".into(), "Projects".into()],
            disabled_distros: vec!["Debian".into()],
            default_risk_filter: "caution".into(),
            auto_scan_on_startup: true,
        }
    }

    #[test]
    fn default_settings_have_all_defaults() {
        let settings = Settings::default();
        assert_eq!(settings.zoom, 1.0);
        assert_eq!(settings.scan_dirs.len(), SCAN_DIR_OPTIONS.len());
        assert!(settings.disabled_distros.is_empty());
        assert_eq!(settings.default_risk_filter, "all");
        assert!(!settings.auto_scan_on_startup);
    }

    #[test]
    fn deserializes_empty_object_with_defaults() {
        let settings: Settings = serde_json::from_str("{}").unwrap();
        assert_eq!(settings.zoom, 1.0);
        assert_eq!(settings.default_risk_filter, "all");
        assert!(!settings.auto_scan_on_startup);
    }

    #[test]
    fn deserializes_missing_zoom_field() {
        let settings: Settings = serde_json::from_str(r#"{"unknown": true}"#).unwrap();
        assert_eq!(settings.zoom, 1.0);
    }

    #[test]
    fn serializes_and_deserializes_round_trip() {
        let settings = sample_settings();
        let json = serde_json::to_string(&settings).unwrap();
        let back: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(back.zoom, 1.5);
        assert_eq!(back.scan_dirs, vec!["Documents".to_string(), "Projects".to_string()]);
        assert_eq!(back.default_risk_filter, "caution");
        assert!(back.auto_scan_on_startup);
    }

    #[test]
    fn serializes_with_camel_case_keys() {
        let settings = sample_settings();
        let json = serde_json::to_value(&settings).unwrap();
        assert_eq!(json["scanDirs"], serde_json::json!(["Documents", "Projects"]));
        assert_eq!(json["disabledDistros"], serde_json::json!(["Debian"]));
        assert_eq!(json["defaultRiskFilter"], "caution");
        assert_eq!(json["autoScanOnStartup"], true);
        assert!(json.get("scan_dirs").is_none());
    }

    #[test]
    fn sanitize_keeps_only_known_scan_dirs_and_valid_filter() {
        let settings = Settings {
            scan_dirs: vec!["Projects".into(), "UnknownDir".into(), "Desktop".into(), "Projects".into()],
            default_risk_filter: "nonsense".into(),
            ..sample_settings()
        };

        let cleaned = sanitize(&settings);
        assert_eq!(cleaned.scan_dirs, vec!["Desktop".to_string(), "Projects".to_string()]);
        assert_eq!(cleaned.default_risk_filter, "all");
    }

    fn settings_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("cachebin-settings-{name}-{}.json", std::process::id()))
    }

    #[test]
    fn load_from_missing_file_returns_default() {
        let path = settings_path("missing");
        let _ = std::fs::remove_file(&path);
        assert_eq!(Settings::load_from(&path).zoom, 1.0);
    }

    #[test]
    fn load_from_corrupt_file_returns_default() {
        let path = settings_path("corrupt");
        std::fs::write(&path, "not json").unwrap();
        assert_eq!(Settings::load_from(&path).zoom, 1.0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_then_load_round_trips_through_disk() {
        let dir = std::env::temp_dir().join(format!("cachebin-settings-dir-{}", std::process::id()));
        let path = dir.join("nested").join("settings.json");

        sample_settings().save_to(&path);
        let back = Settings::load_from(&path);
        assert_eq!(back.zoom, 1.5);
        assert_eq!(back.scan_dirs, vec!["Documents".to_string(), "Projects".to_string()]);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
