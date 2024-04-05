use tauri::{
    generate_context, generate_handler, AboutMetadata, App, Builder, CustomMenuItem, Manager, Menu,
    MenuItem, Submenu, WindowBuilder, WindowUrl,
};

fn init_menu() -> Menu {
    let settings = CustomMenuItem::new("settings".to_string(), "Settings...");
    let submenu = Submenu::new(
        "beevolution",
        Menu::new()
            .add_native_item(MenuItem::About(
                "beevolution".to_string(),
                AboutMetadata::new(),
            ))
            .add_item(settings)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    );
    Menu::new().add_submenu(submenu)
}

pub fn init_app() -> App {
    Builder::default()
        .menu(init_menu())
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            let main_window = app.get_window("main").unwrap();
            let handle = app.handle();

            main_window.on_menu_event(move |event| match event.menu_item_id() {
                "settings" => {
                    WindowBuilder::new(&handle, "settings", WindowUrl::App("#/settings".into()))
                        .title("settings")
                        .build()
                        .unwrap();
                }
                _ => {}
            });
            Ok(())
        })
        .invoke_handler(generate_handler![])
        .build(generate_context!())
        .expect("error while running tauri application")
}
