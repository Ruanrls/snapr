use std::{collections::HashMap, sync::LazyLock};

use crate::commands::{Command, CommandHash, KeyBinding, ScreenPositions};

pub(crate) static DEFAULT_COMMANDS: LazyLock<CommandHash> = LazyLock::new(|| {
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
