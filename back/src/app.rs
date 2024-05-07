use log::error;
use serde_json::json;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, SubmenuBuilder};
use tauri::webview::WebviewWindowBuilder;
use tauri::{
    generate_context, generate_handler, App, AppHandle, Builder, Manager, WebviewUrl, Wry,
};
use tauri_plugin_store::{self, StoreBuilder};

use crate::database::Database;
use crate::store::store_path;

fn init_menu(handle: &AppHandle) -> Result<Menu<Wry>, tauri::Error> {
    let menu = Menu::new(handle)?;
    let main_menu = SubmenuBuilder::new(handle, env!("CARGO_PKG_NAME"))
        .items(&[
            &PredefinedMenuItem::close_window(handle, None)?,
            #[cfg(target_os = "macos")]
            &MenuItem::with_id(handle, "settings", "Settings", true, None::<&str>)?,
            &PredefinedMenuItem::quit(handle, None)?,
        ])
        .build()?;
    let view_menu = SubmenuBuilder::new(handle, "view")
        .items(&[&PredefinedMenuItem::fullscreen(handle, None)?])
        .build()?;

    menu.append(&main_menu)?;
    menu.append(&view_menu)?;
    Ok(menu)
}

pub fn init_app() -> App {
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let main_window = app.get_webview("main").unwrap();
                main_window.open_devtools();
                main_window.close_devtools();
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

            let handle = app.handle();
            let menu = init_menu(handle)?;
            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| match event.id().0.as_str() {
                "settings" => {
                    WebviewWindowBuilder::new(
                        app,
                        "settings",
                        WebviewUrl::App("#/settings".into()),
                    )
                    .title("settings")
                    .build()
                    .unwrap();
                    #[cfg(debug_assertions)]
                    {
                        let settings_window = app.get_webview("settings").unwrap();
                        settings_window.open_devtools();
                        settings_window.close_devtools();
                    }
                }
                _ => {}
            });

            Ok(())
        })
        .invoke_handler(generate_handler![])
        .build(generate_context!())
        .expect("error while running tauri application")
}
