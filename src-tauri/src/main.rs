#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use tauri::Manager;
use tauri::api;

const SHOW_TEXT: &str = "Afficher / Show Tchap";
const QUIT_TEXT: &str = "Quitter / Quit";
const SHOW_ITEM_ID: &str = "show";
const FAQ_ITEM_ID: &str = "faq";

fn main() {
    let show = CustomMenuItem::new(SHOW_ITEM_ID.to_string(), SHOW_TEXT);
    let quit = CustomMenuItem::new("quit".to_string(), QUIT_TEXT);
    let faq = CustomMenuItem::new(FAQ_ITEM_ID.to_string(), "FAQ");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(faq)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let window = app.get_window("main").unwrap();
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    SHOW_ITEM_ID => {
                       if !window.is_visible() {
                            window.show().unwrap();
                       }
                       window.set_focus().unwrap();
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
