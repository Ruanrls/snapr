use std::{collections::HashMap, sync::Arc};

use snapr::{
    configuration::{UserConfiguration, DEFAULT_COMMANDS},
    InitializeCommandsConfig,
};
use tauri::{
    generate_handler,
    menu::MenuBuilder,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager,
};

mod commands;
use commands::{load_config, save_config};

pub struct AppState {
    pub command_storage: Arc<snapr::commands::CommandStorage>,
}

fn set_config(app_handle: AppHandle) {
    let configuration = load_config(app_handle.clone());

    if configuration.is_none() {
        let mut user_commands = UserConfiguration {
            commands: HashMap::new(),
        };
        DEFAULT_COMMANDS.iter().for_each(|(key_binding, command)| {
            user_commands.commands.insert(
                format!("{0};{1}", key_binding.key, key_binding.modifiers),
                command.clone(),
            );
        });

        save_config(app_handle, user_commands);
    }
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
        .icon(app.default_window_icon().unwrap().clone())
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
            let app_handle = app.app_handle();
            setup_tray_menu(app)?;

            let command_storage = snapr::initialize_commands(InitializeCommandsConfig {
                path: app
                    .path()
                    .app_data_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            });

            print!(
                "Initialized command storage with {:?} commands",
                command_storage.commands.read().unwrap()
            );

            let command_arc = Arc::new(command_storage);
            app.manage(AppState {
                command_storage: command_arc.clone(),
            });

            set_config(app_handle.to_owned());
            snapr::events::start_keyboard_listener(command_arc.clone());
            Ok(())
        })
        .invoke_handler(generate_handler![save_config, load_config])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
