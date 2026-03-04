use std::sync::Arc;

use snapr::InitializeCommandsConfig;
use tauri::{
    generate_handler,
    menu::MenuBuilder,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};

mod commands;
use commands::{load_config, save_config, start_listening_keyboard, stop_listening_keyboard};

pub struct AppState {
    pub command_storage: Arc<snapr::commands::CommandStorage>,
    pub keyboard_event_sender: std::sync::mpsc::Sender<snapr::events::Events>,
}

fn setup_tray_menu(app: &App) -> tauri::Result<()> {
    let menu = MenuBuilder::new(app).text("quit", "Quit").build()?;

    app.on_tray_icon_event(|app, event| match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {}
    });

    app.on_menu_event(|app, event| match event.id().0.as_str() {
        "quit" => {
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    });

    let _ = TrayIconBuilder::new()
        .icon(
            app.default_window_icon()
                .expect("Default window icon must be set")
                .clone(),
        )
        .tooltip("Snapr")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .build(app)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .device_event_filter(tauri::DeviceEventFilter::Always)
        .setup(|app| {
            setup_tray_menu(app)?;

            let path = app
                .path()
                .app_data_dir()
                .expect("App data directory must be available at startup");

            let command_storage = snapr::initialize_commands(InitializeCommandsConfig { path })
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            if let Ok(commands) = command_storage.commands.read() {
                print!("Initialized command storage with {:?} commands", *commands);
            }

            let command_arc = Arc::new(command_storage);

            let (sender, receiver) = std::sync::mpsc::channel();
            snapr::commands::listen_commands(receiver, command_arc.clone());
            snapr::events::KeyboardListener::start_keyboard_listener(
                command_arc.clone(),
                sender.clone(),
            );

            app.manage(AppState {
                command_storage: command_arc.clone(),
                keyboard_event_sender: sender,
            });
            Ok(())
        })
        .invoke_handler(generate_handler![
            save_config,
            load_config,
            start_listening_keyboard,
            stop_listening_keyboard
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
