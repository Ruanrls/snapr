use std::sync::Arc;

use snapr::{
    InitializeCommandsConfig,
    commands::{CommandStorage, commands},
    events::{self},
    initialize_commands,
};

fn main() {
    let command_storage = initialize_commands(InitializeCommandsConfig {
        path: String::from("config.json"),
    })
    .expect("Failed to initialize commands");

    let commands: Arc<CommandStorage> = Arc::new(command_storage);
    let commands_clone = commands.clone();

    let (sender, receiver) = std::sync::mpsc::channel();

    commands::listen_commands(receiver, commands);
    events::KeyboardListener::start_keyboard_listener(commands_clone, sender);
}
