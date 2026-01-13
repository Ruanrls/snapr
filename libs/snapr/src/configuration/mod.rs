use crate::commands::{Command, KeyBinding, ScreenPositions};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, write},
    io,
    sync::LazyLock,
};

pub static DEFAULT_COMMANDS: LazyLock<HashMap<KeyBinding, Command>> = LazyLock::new(|| {
    let mut command_storage: HashMap<KeyBinding, Command> = HashMap::new();

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

    command_storage
});

#[derive(Serialize, Deserialize)]
pub struct UserConfiguration {
    pub commands: HashMap<KeyBinding, Command>,
}

pub fn save_config(config: &UserConfiguration, path: &str) -> Result<(), io::Error> {
    let config_json = serde_json::to_string(config)?;
    write(path, config_json)?;

    Ok(())
}

pub fn load_config(path: &str) -> Result<UserConfiguration, io::Error> {
    let mut defaults = DEFAULT_COMMANDS.clone();
    if let Ok(file_string) = fs::read_to_string(path) {
        let commands: UserConfiguration = serde_json::from_str(file_string.as_str())?;
        defaults.extend(commands.commands);

        Ok(UserConfiguration { commands: defaults })
    } else {
        println!("defaults {:?}", defaults);
        Ok(UserConfiguration { commands: defaults })
    }
}
