use snapr::configuration::UserConfiguration;
use tauri::{command, AppHandle, Manager};

use crate::AppState;

#[command]
pub fn save_config(app: AppHandle, config: UserConfiguration) -> Result<(), String> {
    let path = app
        .path()
        .app_data_dir()
        .expect("App data dir should be available");

    let config: UserConfiguration = snapr::configuration::save_config(
        config,
        path.to_str().expect("Path should always be a valid path"),
    )?;

    let app_state = app.state::<AppState>();
    let mut writable_commands = app_state.command_storage.commands.write().unwrap();

    writable_commands.clear();
    config.commands.iter().for_each(|(_, command)| {
        writable_commands.insert(command.key_binding, command.clone());
    });

    Ok(())
}

#[command]
pub fn load_config(app: AppHandle) -> Option<UserConfiguration> {
    let path = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");

    snapr::configuration::load_config(path.as_path().to_str()?)
}
