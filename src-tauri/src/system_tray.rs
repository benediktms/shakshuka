use tauri::{AppHandle, Manager, SystemTrayEvent};

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
