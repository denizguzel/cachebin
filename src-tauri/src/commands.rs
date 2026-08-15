use std::collections::HashMap;

use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::models::{LargeFile, PlatformInfo, ProjectArtifact, ScanResult};
use crate::{cleanup, history, large, platform, projects, scan, settings, wsl, zoom};

#[tauri::command]
pub fn zoom_by<R: Runtime>(delta: f64, app: AppHandle<R>) {
    zoom::zoom_by(&app, delta);
}

#[tauri::command]
pub fn reset_zoom<R: Runtime>(app: AppHandle<R>) {
    zoom::reset(&app);
}

#[tauri::command]
pub fn get_platform_info() -> Result<PlatformInfo, String> {
    platform::info()
}

#[tauri::command]
pub async fn scan_storage<R: Runtime>(app: AppHandle<R>) -> Result<ScanResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let result = scan::run(app.clone())?;
        let _ = scan::save_cached(&app, &result);
        Ok(result)
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
pub fn load_cached_scan<R: Runtime>(app: AppHandle<R>) -> Option<scan::CachedScan> {
    scan::load_cached(&app)
}

#[tauri::command]
pub async fn scan_projects<R: Runtime>(app: AppHandle<R>) -> Result<Vec<ProjectArtifact>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let env = std::env::vars().collect::<HashMap<_, _>>();
        let dirs = app.state::<settings::SettingsState>().0.lock().unwrap().scan_dirs.clone();
        projects::scan(&env, &dirs, app.state::<scan::ScanToken>().cancelled_flag())
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
pub async fn scan_large_files<R: Runtime>(app: AppHandle<R>) -> Result<Vec<LargeFile>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _ = large::mark_scanning(&app);
        let env = std::env::vars().collect::<HashMap<_, _>>();
        let scan_dirs = app.state::<settings::SettingsState>().0.lock().unwrap().scan_dirs.clone();
        let disabled = app.state::<settings::SettingsState>().0.lock().unwrap().disabled_distros.clone();
        let distros = wsl::discover()
            .into_iter()
            .filter(|distro| distro.version == 2 && distro.state.eq_ignore_ascii_case("running"))
            .filter(|distro| !disabled.contains(&distro.name))
            .collect::<Vec<_>>();
        let files = large::scan(&env, &distros, &scan_dirs, app.state::<scan::ScanToken>().cancelled_flag())?;
        let _ = large::save_cached(&app, &files);
        Ok(files)
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
pub fn get_settings<R: Runtime>(app: AppHandle<R>) -> settings::Settings {
    app.state::<settings::SettingsState>().0.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_scan_dir_options() -> Vec<String> {
    settings::SCAN_DIR_OPTIONS.iter().map(|dir| dir.to_string()).collect()
}

#[tauri::command]
pub fn update_settings<R: Runtime>(app: AppHandle<R>, settings: settings::Settings) -> settings::Settings {
    let state = app.state::<settings::SettingsState>();
    let mut current = state.0.lock().unwrap();
    *current = settings::sanitize(&settings);
    current.save(&app);
    current.clone()
}

#[tauri::command]
pub fn load_cached_large_files<R: Runtime>(app: AppHandle<R>) -> Option<large::CachedLargeFiles> {
    large::load_cached(&app)
}

#[tauri::command]
pub fn cancel_scan<R: Runtime>(app: AppHandle<R>) {
    scan::cancel(&app);
}

#[tauri::command]
pub async fn move_to_trash<R: Runtime>(paths: Vec<String>, app: AppHandle<R>) -> cleanup::TrashReport {
    tauri::async_runtime::spawn_blocking(move || {
        cleanup::move_to_trash(&paths, &mut |current, total| {
            let _ = app.emit("cleanup://progress", cleanup::CleanupProgress { current, total });
        })
    })
    .await
    .unwrap_or_else(|_| cleanup::TrashReport { moved: Vec::new(), failed: Vec::new() })
}

#[tauri::command]
pub fn get_history<R: Runtime>(app: AppHandle<R>) -> Vec<history::ActivityEvent> {
    history::load(&app)
}

#[tauri::command]
pub fn save_history<R: Runtime>(events: Vec<history::ActivityEvent>, app: AppHandle<R>) -> Result<(), String> {
    history::save(&app, &events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::ScanToken;
    use tauri::test::{mock_builder, mock_context, noop_assets};
    use tauri::Manager;

    fn mock_app() -> tauri::App<tauri::test::MockRuntime> {
        mock_builder()
            .build(mock_context(noop_assets()))
            .expect("mock app should build")
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn get_platform_info_returns_platform() {
        let info = get_platform_info().expect("platform info");
        assert_eq!(info.os_name, "Windows");
        assert!(info.total_bytes > 0);
    }

    #[test]
    fn cancel_scan_marks_token() {
        let app = mock_app();
        app.manage(ScanToken::default());

        assert!(!app.state::<ScanToken>().is_cancelled());
        cancel_scan(app.handle().clone());
        assert!(app.state::<ScanToken>().is_cancelled());
    }
}
