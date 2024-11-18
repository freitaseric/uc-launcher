use commands::{set_activity, start_rpc};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    Manager,
};

mod commands;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .setup(|app| {
            #[cfg(desktop)]
            {
                let _ = app.handle().plugin(tauri_plugin_positioner::init());
                let show = MenuItemBuilder::new("Mostrar")
                    .id("show")
                    .build(app)
                    .unwrap();
                let hide = MenuItemBuilder::new("Fechar")
                    .id("hide")
                    .build(app)
                    .unwrap();
                let quit = MenuItemBuilder::new("Sair").id("quit").build(app).unwrap();

                let menu = MenuBuilder::new(app)
                    .items(&[&show, &hide, &quit])
                    .build()
                    .unwrap();

                let _ = tauri::tray::TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .on_menu_event(|app, event| {
                        let window = app.get_webview_window("main").unwrap();

                        match event.id().as_ref() {
                            "show" => {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                            "hide" => window.hide().unwrap(),
                            "quit" => app.exit(0),
                            _ => {}
                        }
                    })
                    .on_tray_icon_event(|tray_handle, event| {
                        tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
                    })
                    .build(app)?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_rpc, set_activity])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
