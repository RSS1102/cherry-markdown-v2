use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App,
};

use crate::utils::windows::restore_and_focus_window;

pub fn system_tray_menu(app: &mut App) -> Result<(), tauri::Error> {
    let show_main_window = MenuItem::with_id(
        app,
        "show_main_window",
        "Open Cherry Markdown",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_main_window, &quit])?;

    let _ = TrayIconBuilder::new()
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                let app = tray.app_handle();
                restore_and_focus_window(app, "main");
            }
            _ => {}
        })
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show_main_window" => {
                restore_and_focus_window(app, "main");
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

    Ok(())
}
