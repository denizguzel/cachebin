mod cleanup;
mod commands;
mod history;
mod large;
mod menu;
mod models;
mod platform;
mod projects;
mod scan;
mod scanner;
mod settings;
mod size;
mod time;
mod tray;
mod window;
mod wsl;
mod zoom;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::zoom_by,
            commands::reset_zoom,
            commands::get_platform_info,
            commands::scan_storage,
            commands::load_cached_scan,
            commands::scan_projects,
            commands::scan_large_files,
            commands::load_cached_large_files,
            commands::cancel_scan,
            commands::move_to_trash,
            commands::get_history,
            commands::save_history,
            commands::get_settings,
            commands::get_scan_dir_options,
            commands::update_settings,
        ])
        .setup(|app| {
            let settings = settings::init(&app.handle());

            app.manage(scan::ScanToken::default());

            // No large-file scan can be running on process startup: clear a stale marker left by a crash.
            let _ = large::reset_stale_scanning(&app.handle());

            window::size_for_monitor(&app.handle())?;
            zoom::init(&app.handle(), settings.zoom);

            let app_menu = menu::create(&app.handle())?;
            app.set_menu(app_menu)?;

            tray::create(&app.handle())?;
            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            menu::id::ZOOM_IN => zoom::zoom_by(app, 0.1),
            menu::id::ZOOM_OUT => zoom::zoom_by(app, -0.1),
            menu::id::RESET_ZOOM => zoom::reset(app),
            menu::id::RELOAD => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.reload();
                }
            }
            _ => {}
        })
        .on_window_event(|window, event| {
            if window.label() == "tray-popup" {
                if let tauri::WindowEvent::Focused(false) = event {
                    let popup = window.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(120));
                        if !popup.is_focused().unwrap_or(false) {
                            let _ = popup.hide();
                        }
                    });
                    return;
                }
            }

            // Closing the window hides it to the system tray instead of quitting.
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


