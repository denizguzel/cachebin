use tauri::menu::{Menu, MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder};
use tauri::{AppHandle, Runtime};

pub mod id {
    pub const RELOAD: &str = "reload";
    pub const ZOOM_IN: &str = "zoom_in";
    pub const ZOOM_OUT: &str = "zoom_out";
    pub const RESET_ZOOM: &str = "reset_zoom";
}

const SHORTCUT_RELOAD: &str = "Reload\tF5";
const SHORTCUT_ZOOM_IN: &str = "Zoom In\tCtrl +";
const SHORTCUT_ZOOM_OUT: &str = "Zoom Out\tCtrl -";
const SHORTCUT_RESET_ZOOM: &str = "Reset Zoom\tCtrl+0";

pub fn create<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&PredefinedMenuItem::close_window(app, Some("Close Window"))?)
        .separator()
        .item(&PredefinedMenuItem::quit(app, Some("Quit"))?)
        .build()?;

    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .item(&PredefinedMenuItem::cut(app, Some("Cut"))?)
        .item(&PredefinedMenuItem::copy(app, Some("Copy"))?)
        .item(&PredefinedMenuItem::paste(app, Some("Paste"))?)
        .separator()
        .item(&PredefinedMenuItem::select_all(app, Some("Select All"))?)
        .build()?;

    let reload = MenuItem::with_id(app, id::RELOAD, SHORTCUT_RELOAD, true, None::<&str>)?;
    let zoom_in = MenuItem::with_id(app, id::ZOOM_IN, SHORTCUT_ZOOM_IN, true, None::<&str>)?;
    let zoom_out = MenuItem::with_id(app, id::ZOOM_OUT, SHORTCUT_ZOOM_OUT, true, None::<&str>)?;
    let reset_zoom = MenuItem::with_id(app, id::RESET_ZOOM, SHORTCUT_RESET_ZOOM, true, None::<&str>)?;

    let view_menu = SubmenuBuilder::new(app, "View")
        .item(&reload)
        .separator()
        .item(&zoom_in)
        .item(&zoom_out)
        .item(&reset_zoom)
        .build()?;

    let help_menu = SubmenuBuilder::new(app, "Help")
        .item(&PredefinedMenuItem::about(app, Some("About Cachebin"), None)?)
        .build()?;

    MenuBuilder::new(app)
        .items(&[&file_menu, &edit_menu, &view_menu, &help_menu])
        .build()
}

#[cfg(test)]
mod tests {
    use super::id;

    #[test]
    fn menu_ids_are_unique_and_non_empty() {
        let ids = [id::RELOAD, id::ZOOM_IN, id::ZOOM_OUT, id::RESET_ZOOM];

        assert!(ids.iter().all(|value| !value.is_empty()));
        for (index, value) in ids.iter().enumerate() {
            assert!(!ids.iter().skip(index + 1).any(|other| other == value));
        }
    }
}
