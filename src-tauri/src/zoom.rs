use std::sync::Mutex;

use tauri::{AppHandle, Manager, Runtime};

use crate::settings;

const ZOOM_MIN: f64 = 0.5;
const ZOOM_MAX: f64 = 3.0;

pub struct ZoomState(Mutex<f64>);

pub fn init<R: Runtime>(app: &AppHandle<R>, initial: f64) {
    app.manage(ZoomState(Mutex::new(clamp_zoom(initial))));
    apply(app, initial);
}

pub fn zoom_by<R: Runtime>(app: &AppHandle<R>, delta: f64) {
    let state = app.state::<ZoomState>();
    let mut current = state.0.lock().unwrap();
    *current = clamp_zoom(*current + delta);
    let next = *current;
    drop(current);
    apply(app, next);
    settings::update_zoom(app, next);
}

pub fn reset<R: Runtime>(app: &AppHandle<R>) {
    apply(app, 1.0);
    settings::update_zoom(app, 1.0);
}

fn clamp_zoom(value: f64) -> f64 {
    value.clamp(ZOOM_MIN, ZOOM_MAX)
}

fn apply<R: Runtime>(app: &AppHandle<R>, scale: f64) {
    *app.state::<ZoomState>().0.lock().unwrap() = clamp_zoom(scale);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_zoom(scale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::Manager;
    use tauri::test::{mock_builder, mock_context, noop_assets};

    fn mock_app() -> tauri::App<tauri::test::MockRuntime> {
        mock_builder()
            .build(mock_context(noop_assets()))
            .expect("mock app should build")
    }

    fn zoom_of(app: &tauri::App<tauri::test::MockRuntime>) -> f64 {
        *app.state::<ZoomState>().0.lock().unwrap()
    }

    #[test]
    fn clamp_zoom_limits_to_range() {
        assert_eq!(clamp_zoom(0.0), ZOOM_MIN);
        assert_eq!(clamp_zoom(-1.0), ZOOM_MIN);
        assert_eq!(clamp_zoom(0.5), 0.5);
        assert_eq!(clamp_zoom(1.0), 1.0);
        assert_eq!(clamp_zoom(3.0), 3.0);
        assert_eq!(clamp_zoom(10.0), ZOOM_MAX);
    }

    #[test]
    fn zoom_range_constants_are_sane() {
        assert!(ZOOM_MIN < 1.0);
        assert!(ZOOM_MAX > 1.0);
    }

    #[test]
    fn init_manages_state_with_clamped_value() {
        let app = mock_app();
        init(app.handle(), 9.0);
        assert_eq!(zoom_of(&app), ZOOM_MAX);
    }

    #[test]
    fn apply_updates_and_clamps_state() {
        let app = mock_app();
        app.manage(ZoomState(Mutex::new(1.0)));

        apply(app.handle(), 1.2);
        assert_eq!(zoom_of(&app), 1.2);

        apply(app.handle(), 20.0);
        assert_eq!(zoom_of(&app), ZOOM_MAX);

        apply(app.handle(), -20.0);
        assert_eq!(zoom_of(&app), ZOOM_MIN);
    }
}
