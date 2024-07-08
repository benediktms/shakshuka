// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod system_tray;

use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, Manager, Size, WindowEvent};
use ts_rs::TS;

use system_tray::{make_system_tray, system_tray_event_handler};

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
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            match window.set_size(Size::Logical(LogicalSize {
                width: 1300.0,
                height: 800.0,
            })) {
                Err(err) => println!("{err}"),
                Ok(res) => res,
            }

            Ok(())
        })
        .system_tray(make_system_tray())
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
