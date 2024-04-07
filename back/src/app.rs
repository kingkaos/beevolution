use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{generate_context, generate_handler, App, AppHandle, Builder, Manager, Runtime, Wry};

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
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            Ok(())
        })
        .invoke_handler(generate_handler![])
        .build(generate_context!())
        .expect("error while running tauri application")
}
