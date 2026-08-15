use tauri::{AppHandle, Manager, Runtime, Size};

const SIZE_4K: (u32, u32) = (2560, 1440);
const SIZE_2K: (u32, u32) = (1920, 1200);
const SIZE_DEFAULT: (u32, u32) = (1280, 800);

const MIN_HEIGHT_4K: u32 = 2000;
const MIN_HEIGHT_2K: u32 = 1400;

pub fn size_for_monitor<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window("main") else {
        return Ok(());
    };
    let Some(monitor) = window.current_monitor()? else {
        return Ok(());
    };

    let height = monitor.size().height;
    let (w, h) = size_for_height(height);

    window.set_size(Size::Physical(tauri::PhysicalSize::new(w, h)))?;
    window.center()?;
    Ok(())
}

fn size_for_height(height: u32) -> (u32, u32) {
    match height {
        h if h >= MIN_HEIGHT_4K => SIZE_4K,
        h if h >= MIN_HEIGHT_2K => SIZE_2K,
        _ => SIZE_DEFAULT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_size_tier_by_height() {
        assert_eq!(size_for_height(2160), SIZE_4K);
        assert_eq!(size_for_height(2000), SIZE_4K);
        assert_eq!(size_for_height(1440), SIZE_2K);
        assert_eq!(size_for_height(1400), SIZE_2K);
        assert_eq!(size_for_height(1080), SIZE_DEFAULT);
        assert_eq!(size_for_height(768), SIZE_DEFAULT);
    }
}
