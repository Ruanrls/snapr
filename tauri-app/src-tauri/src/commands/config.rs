use snapr::configuration::{ConfigurationError, UserConfiguration};
use tauri::{command, AppHandle, Manager};

use crate::AppState;

#[command]
pub fn save_config(app: AppHandle, config: UserConfiguration) -> Result<(), String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {e}"))?;

    let path_str = path
        .to_str()
        .ok_or_else(|| "App data path contains invalid UTF-8".to_string())?;

    let config = snapr::configuration::save_config(config, path_str).map_err(|e| e.to_string())?;

    let app_state = app.state::<AppState>();
    let mut writable_commands = app_state
        .command_storage
        .commands
        .write()
        .expect("Command storage lock poisoned");

    writable_commands.clear();
    for (_, command) in &config.commands {
        writable_commands.insert(command.key_binding, command.clone());
    }

    Ok(())
}

#[command]
pub fn load_config(app: AppHandle) -> Result<UserConfiguration, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {e}"))?;

    let path_str = path
        .to_str()
        .ok_or_else(|| "App data path contains invalid UTF-8".to_string())?;

    match snapr::configuration::load_config(path_str) {
        Ok(config) => Ok(config),
        Err(ConfigurationError::ConfigNotFound(_)) => {
            let default_config = UserConfiguration::default();

            save_config(app, default_config.clone())?;
            Ok(default_config)
        }
        Err(e) => Err(e.to_string()),
    }
}
