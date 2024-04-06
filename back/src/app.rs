use tauri::{generate_context, generate_handler, App, Builder, Manager};

pub fn init_app() -> App {
    Builder::default()
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
