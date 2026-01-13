mod commands;
mod configuration;
mod events;
mod monitor;

use std::sync::Arc;

use crate::commands::{CommandHandler, CommandStorage};
use crate::configuration::load_config;
use crate::events::start_listening;

fn main() {
    let command_storage = CommandStorage::new();
    let commands_clone = Arc::clone(&command_storage.commands);

    std::thread::spawn(move || {
        if let Ok(user_configuration) = load_config("./config.json") {
            let mut storage = command_storage.commands.write().unwrap();
            storage.extend(user_configuration.commands);
        }
    });

    let event_handler = start_listening(commands_clone);
    let _ = event_handler.join();
}
