#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    symbol: String,
    #[serde(rename(deserialize = "lastPrice"))]
    price: String,
    #[serde(rename(deserialize = "priceChangePercent"))]
    change: String,
}

#[tauri::command]
fn get_binance_ticker() -> Vec<ApiResponse> {
    let url = "https://api.binance.com/api/v1/ticker/24hr?symbols=[%22BTCUSDT%22,%22BNBUSDT%22,%22ETHUSDT%22,%22XRPUSDT%22,%22DOGEUSDT%22,%22SHIBUSDT%22]";

    let url = Url::parse(&*url).unwrap();
    let res = reqwest::blocking::get(url).unwrap();
    let items: Vec<ApiResponse> = res.json().unwrap();

    println!("{:?}", items);
    items
}

#[tauri::command]
fn set_title(app_handle: tauri::AppHandle, value: &str) {
    #[cfg(target_os = "macos")]
    app_handle.tray_handle().set_title(&value).unwrap();
}

fn create_tray(app: &tauri::App) -> tauri::Result<()> {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("clear_title", "Clear Title"))
        .add_item(CustomMenuItem::new("set_title", "Set Title"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

    let handle = app.handle();
    let tray_id = "cryptobar-tray".to_string();
    SystemTray::new()
        .with_id(&tray_id)
        .with_menu(tray_menu)
        .on_event(move |event| {
            let tray_handle = handle.tray_handle_by_id(&tray_id).unwrap();
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("left click")
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    // let item_handle = tray_handle.get_item(&id);
                    println!("menu click");
                    match id.as_str() {
                        "quit" => {
                            // exit the app
                            handle.exit(0);
                        }
                        "clear_title" => {
                            #[cfg(target_os = "macos")]
                            tray_handle.set_title("").unwrap();
                        }
                        "set_title" => {
                            #[cfg(target_os = "macos")]
                            tray_handle.set_title("Tauri").unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .build(app)
        .map(|_| ())
}

fn main() {
    tauri::Builder::new()
        .setup(|app| {
            create_tray(app)?;
            Ok(())
        })
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::LeftClick { position, size, .. } => {
                let w = app.get_window("main").unwrap();
                let visible = w.is_visible().unwrap();
                if visible {
                    w.hide().unwrap();
                } else {
                    w.show().unwrap();
                    w.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // don't kill the app when the user clicks close. this is important
                // event.window().hide().unwrap();
                // api.prevent_close();
            }
            tauri::WindowEvent::Focused(false) => {
                // hide the window automatically when the user
                // clicks out. this is for a matter of taste.
                // event.window().hide().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![set_title, get_binance_ticker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
