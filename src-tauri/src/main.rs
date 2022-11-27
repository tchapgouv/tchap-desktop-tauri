#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use tauri::Manager;
use tauri::api;

const HIDE_TEXT: &str = "Masquer / Hide";
const SHOW_TEXT: &str = "Afficher / Show";
const QUIT_TEXT: &str = "Quitter / Quit";
const HIDE_SHOW_ITEM_ID: &str = "hide_show";
const FAQ_ITEM_ID: &str = "faq";

fn main() {
    let hide_show = CustomMenuItem::new(HIDE_SHOW_ITEM_ID.to_string(), HIDE_TEXT);
    let quit = CustomMenuItem::new("quit".to_string(), QUIT_TEXT);
    let faq = CustomMenuItem::new(FAQ_ITEM_ID.to_string(), "FAQ");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide_show)
        .add_item(faq)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                let window = app.get_window("main").unwrap();
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    HIDE_SHOW_ITEM_ID => {
                        match window.is_visible().unwrap() {
                            false => {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                                item_handle.set_title(HIDE_TEXT).unwrap();
                            }
                            _ => {
                                window.hide().unwrap();
                                item_handle.set_title(SHOW_TEXT).unwrap();
                            }
                        }
                    }
                    FAQ_ITEM_ID => {
                        api::shell::open(&window.shell_scope(), "https://tchap.gouv.fr/faq".to_string(), None).unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })  
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }   
            _ => {}
        });
}
