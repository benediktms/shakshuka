// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod system_tray;

use serde::{Deserialize, Serialize};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, WindowEvent};
use ts_rs::TS;

use system_tray::system_tray_event_handler;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
struct TestRes {
    msg: String,
    ex: u32,
}

#[tauri::command]
fn test_command() -> TestRes {
    TestRes {
        msg: "This is a test message".to_string(),
        ex: 83839,
    }
}

fn main() {
    let quit = CustomMenuItem::new("quit", "Quit");
    let hide = CustomMenuItem::new("hide", "Hide");
    let show = CustomMenuItem::new("show", "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_item(hide)
        .add_item(show);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(system_tray_event_handler())
        .on_window_event(|e| {
            let event = e.event();
            if let WindowEvent::CloseRequested { api, .. } = event {
                e.window().hide().expect("could not hide window");
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![greet, test_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
