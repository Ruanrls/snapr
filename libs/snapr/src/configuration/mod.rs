pub mod error;
pub use error::ConfigurationError;

use crate::commands::{Command, CommandHash, KeyBinding, ScreenPositions};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, write},
    path::Path,
    sync::LazyLock,
};

pub static DEFAULT_COMMANDS: LazyLock<CommandHash> = LazyLock::new(|| {
    let mut command_storage: CommandHash = HashMap::new();

    let key = KeyBinding {
        modifiers: 4,
        key: 39,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::Right,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 38,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::Top,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 40,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::Bottom,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 37,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::Left,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 73,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::TopLeft,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 79,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::TopRight,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 75,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::BottomLeft,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 76,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::BottomRight,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 67,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::Center,
        },
    );

    let key = KeyBinding {
        modifiers: 4,
        key: 13,
    };
    command_storage.insert(
        key,
        Command {
            key_binding: key,
            position: ScreenPositions::Maximize,
        },
    );

    command_storage
});

#[derive(Clone, Serialize, Deserialize)]
pub struct UserConfiguration {
    pub commands: HashMap<String, Command>,
}

pub fn save_config(config: UserConfiguration, path: &str) -> Result<UserConfiguration, ConfigurationError> {
    let config_json = serde_json::to_string(&config)?;

    let config_path = Path::new(path).join("config.json");

    if let Some(parent_path) = config_path.parent() {
        fs::create_dir_all(parent_path)?;
    }

    write(&config_path, config_json)?;

    Ok(config)
}

pub fn load_config(path: &str) -> Result<UserConfiguration, ConfigurationError> {
    let config_path = Path::new(path).join("config.json");

    if !config_path.exists() {
        return Err(ConfigurationError::ConfigNotFound(
            config_path.display().to_string(),
        ));
    }

    let file_string = fs::read_to_string(&config_path)?;
    let user_configuration: UserConfiguration = serde_json::from_str(&file_string)?;

    Ok(user_configuration)
}
