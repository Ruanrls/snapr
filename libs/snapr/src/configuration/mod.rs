pub mod defaults;
pub mod error;
pub use error::ConfigurationError;

use crate::commands::Command;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, write},
    path::Path,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserConfiguration {
    pub commands: HashMap<String, Command>,
}

impl Default for UserConfiguration {
    fn default() -> Self {
        let mut commands = HashMap::new();
        defaults::DEFAULT_COMMANDS
            .iter()
            .for_each(|(key_binding, command)| {
                commands.insert(
                    format!("{};{}", key_binding.key, key_binding.modifiers),
                    command.clone(),
                );
            });

        UserConfiguration { commands }
    }
}

pub fn save_config(
    config: UserConfiguration,
    path: &Path,
) -> Result<UserConfiguration, ConfigurationError> {
    let config_json = serde_json::to_string(&config)?;

    let config_path = path.join("config.json");
    if let Some(parent_path) = config_path.parent() {
        fs::create_dir_all(parent_path)?;
    }

    write(&config_path, config_json)?;

    Ok(config)
}

pub fn load_config(path: &Path) -> Result<UserConfiguration, ConfigurationError> {
    let config_path = path.join("config.json");
    if !config_path.exists() {
        return Err(ConfigurationError::ConfigNotFound(
            config_path.display().to_string(),
        ));
    }

    let file_string = fs::read_to_string(&config_path)?;
    let user_configuration: UserConfiguration = serde_json::from_str(&file_string)?;

    Ok(user_configuration)
}
