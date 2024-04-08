use log::error;
use serde_json::json;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{generate_context, generate_handler, App, AppHandle, Builder, Manager, Runtime, Wry};
use tauri_plugin_store::{self, StoreBuilder};

use crate::database::Database;
use crate::store::store_path;

fn init_menu<R: Runtime>(handle: &AppHandle) -> tauri::Result<Menu<Wry>> {
    Menu::with_items(
        handle,
        &[
            &Submenu::with_items(
                handle,
                env!("CARGO_PKG_NAME"),
                true,
                &[
                    &PredefinedMenuItem::close_window(handle, None)?,
                    #[cfg(target_os = "macos")]
                    &MenuItem::new(handle, "Hello", true, None::<&str>)?,
                    &PredefinedMenuItem::quit(handle, None)?,
                ],
            )?,
            &Submenu::with_items(
                handle,
                "View",
                true,
                &[&PredefinedMenuItem::fullscreen(handle, None)?],
            )?,
        ],
    )
}

pub fn init_app() -> App {
    Builder::default()
        .menu(|handle| init_menu::<Wry>(handle))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            let path = store_path();
            let mut store = StoreBuilder::new(path.as_str()).build(app.handle().clone());
            if !store.has("database") {
                let _ = store.insert("database".to_string(), json!(Database::default()));
                let _ = store.save();
            }
            store.load().unwrap_or_else(|why| {
                error!("Failed to load store from disk: {:?}", why);
                panic!("Unable to initialize app.");
            });
            Ok(())
        })
        .invoke_handler(generate_handler![])
        .build(generate_context!())
        .expect("error while running tauri application")
}
