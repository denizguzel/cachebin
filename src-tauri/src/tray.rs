use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};

const POPUP_LABEL: &str = "tray-popup";
const POPUP_WIDTH: f64 = 360.0;
const POPUP_HEIGHT: f64 = 440.0;

pub fn create<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let show = MenuItemBuilder::with_id("show", "Show Cachebin").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit Cachebin").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

    TrayIconBuilder::with_id("cachebin-tray")
        .icon(app.default_window_icon().cloned().expect("bundled app icon"))
        .tooltip("Cachebin")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_window(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                position,
                ..
            } = event
            {
                toggle_popup(tray.app_handle(), position.x, position.y);
            }
        })
        .build(app)?;

    Ok(())
}

fn show_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn toggle_popup<R: Runtime>(app: &AppHandle<R>, tray_x: f64, tray_y: f64) {
    if let Some(popup) = app.get_webview_window(POPUP_LABEL) {
        let visible = popup.is_visible().unwrap_or(false);
        if visible {
            let _ = popup.hide();
        } else {
            position_popup(app, &popup, tray_x, tray_y);
            let _ = popup.show();
            let _ = popup.set_focus();
        }
        return;
    }

    let Ok(popup) = WebviewWindowBuilder::new(
        app,
        POPUP_LABEL,
        WebviewUrl::App("index.html".into()),
    )
    .title("Cachebin")
    .inner_size(POPUP_WIDTH, POPUP_HEIGHT)
    .decorations(false)
    .resizable(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .shadow(true)
    .visible(false)
    .focused(true)
    .build() else {
        return;
    };

    position_popup(app, &popup, tray_x, tray_y);
    let _ = popup.show();
    let _ = popup.set_focus();
}

fn position_popup<R: Runtime>(app: &AppHandle<R>, popup: &tauri::WebviewWindow<R>, tray_x: f64, tray_y: f64) {
    let Ok(monitors) = app.available_monitors() else {
        return;
    };
    let cursor_x = tray_x as i32;
    let cursor_y = tray_y as i32;
    let monitor = monitors.iter().find(|monitor| {
        let bounds_position = monitor.position();
        let bounds_size = monitor.size();
        cursor_x >= bounds_position.x
            && cursor_x < bounds_position.x + bounds_size.width as i32
            && cursor_y >= bounds_position.y
            && cursor_y < bounds_position.y + bounds_size.height as i32
    });
    let Some(monitor) = monitor.or_else(|| monitors.first()) else {
        return;
    };

    let area = monitor.work_area();
    let scale = monitor.scale_factor();
    // Use the actual outer size when available. Frameless windows can differ from their logical
    // inner size after DPI rounding and backend-specific shadow/border handling.
    let (popup_width, popup_height) = popup
        .outer_size()
        .map(|size| (size.width as f64, size.height as f64))
        .unwrap_or((POPUP_WIDTH * scale, POPUP_HEIGHT * scale));
    let safety_inset = 4.0 * scale;
    let min_x = area.position.x as f64 + safety_inset;
    let min_y = area.position.y as f64 + safety_inset;
    let max_x = (area.position.x + area.size.width as i32) as f64 - popup_width - safety_inset;
    let max_y = (area.position.y + area.size.height as i32) as f64 - popup_height - safety_inset;
    let x = (tray_x - popup_width / 2.0).clamp(min_x, max_x.max(min_x));
    let y = (tray_y - popup_height - 8.0 * scale).clamp(min_y, max_y.max(min_y));

    let _ = popup.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
        x.round() as i32,
        y.round() as i32,
    )));
}
