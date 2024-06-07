use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn make_system_tray() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("quit", "Quit"))
            .add_item(CustomMenuItem::new("hide", "Hide"))
            .add_item(CustomMenuItem::new("show", "Show")),
    )
}

pub fn system_tray_event_handler() -> impl Fn(&AppHandle, SystemTrayEvent) {
    |app, event| {
        if let SystemTrayEvent::MenuItemClick { id, .. } = event {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            }
        }
    }
}
