use crate::commands::{Command, CommandHash, KeyBinding, ScreenPositions};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, write},
    io,
    ops::Deref,
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

#[derive(Serialize, Deserialize)]
pub struct UserConfiguration {
    pub commands: HashMap<String, Command>,
}

pub fn save_config(config: UserConfiguration, path: &str) -> Result<UserConfiguration, String> {
    let config_json = serde_json::to_string(&config).map_err(|e| "Invalid configuration")?;

    let config_path = Path::new(path);
    let config_path = config_path.join("config.json");
    let config_path = config_path.deref();

    if let Some(parent_path) = config_path.parent() {
        fs::File::create(parent_path).map_err(|err| err.to_string())?;
    }

    dbg!("Saving config to: {}", config_path);
    write(config_path, config_json).map_err(|err| err.to_string())?;
    dbg!("Config file saved successfully to: {}", config_path);

    Ok(config)
}

pub fn load_config(path: &str) -> Option<UserConfiguration> {
    let config_path = format!("{path}/config.json");
    let config_path = Path::new(config_path.as_str());

    if let Ok(file_string) = fs::read_to_string(config_path) {
        let user_configuration: UserConfiguration =
            serde_json::from_str(file_string.as_str()).unwrap();

        println!("Configuration initialized successfully!");
        return Some(user_configuration);
    }

    None
}
