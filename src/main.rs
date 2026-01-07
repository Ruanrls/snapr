mod commands;
mod events;
mod monitor;

use std::sync::Arc;

use crate::commands::{Command, CommandHandler, CommandStorage, KeyBinding, ScreenPositions};
use crate::events::start_listening;

fn main() {
    let command_storage = CommandStorage::new();
    command_storage.add(Command {
        key_binding: KeyBinding {
            modifiers: 4,
            key: 39,
        },
        position: ScreenPositions::Right,
    });
    command_storage.add(Command {
        key_binding: KeyBinding {
            modifiers: 4,
            key: 38,
        },
        position: ScreenPositions::Top,
    });
    command_storage.add(Command {
        key_binding: KeyBinding {
            modifiers: 4,
            key: 40,
        },
        position: ScreenPositions::Bottom,
    });
    command_storage.add(Command {
        key_binding: KeyBinding {
            modifiers: 4,
            key: 37,
        },
        position: ScreenPositions::Left,
    });
    command_storage.add(Command {
        key_binding: KeyBinding {
            modifiers: 4,
            key: 73,
        },
        position: ScreenPositions::TopLeft,
    });
    command_storage.add(Command {
        key_binding: KeyBinding {
            modifiers: 4,
            key: 79,
        },
        position: ScreenPositions::TopRight,
    });
    command_storage.add(Command {
        key_binding: KeyBinding {
            modifiers: 4,
            key: 75,
        },
        position: ScreenPositions::BottomLeft,
    });
    command_storage.add(Command {
        key_binding: KeyBinding {
            modifiers: 4,
            key: 76,
        },
        position: ScreenPositions::BottomRight,
    });

    let commands_clone = Arc::clone(&command_storage.commands);
    let event_handler = start_listening(commands_clone);

    let _ = event_handler.join();
}
