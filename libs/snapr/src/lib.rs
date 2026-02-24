use crate::{
    commands::{CommandHandler, CommandHash, CommandStorage},
    configuration::{DEFAULT_COMMANDS, UserConfiguration, save_config},
};

pub mod commands;
pub mod configuration;
pub mod events;
mod monitor;

pub struct InitializeCommandsConfig {
    pub path: String,
}

pub fn initialize_commands(params: InitializeCommandsConfig) -> CommandStorage {
    let command_storage = CommandStorage::new();
    let load_user_config = configuration::load_config(&params.path);

    // Add user configured commands
    if let Some(user_configuration) = load_user_config {
        user_configuration.commands.iter().for_each(|(_, command)| {
            command_storage.add(command.clone());
        });
    } else {
        // Fallback to default commands
        DEFAULT_COMMANDS.iter().for_each(|(_, command)| {
            command_storage.add(command.clone());
        });
    }

    command_storage
}

pub fn update_keybinding(
    command_storage: &CommandStorage,
    new_command: commands::Command,
    path: &str,
) -> Result<(), String> {
    let mut writable_commands = command_storage.commands.write().unwrap();
    writable_commands.insert(new_command.key_binding, new_command);

    save_user_config(&writable_commands, path)
}

pub fn remove_keybinding(
    command_storage: &CommandStorage,
    keybinding: commands::KeyBinding,
    path: &str,
) -> Result<(), String> {
    command_storage.remove(keybinding);

    save_user_config(&command_storage.commands.read().unwrap(), path)
}

pub fn save_user_config(command_storage: &CommandHash, path: &str) -> Result<(), String> {
    let user_configuration: UserConfiguration = UserConfiguration {
        commands: command_storage
            .iter()
            .map(|(key_binding, command)| {
                (
                    format!("{0};{1}", key_binding.key, key_binding.modifiers),
                    command.clone(),
                )
            })
            .collect(),
    };

    save_config(user_configuration, path)?;
    Ok(())
}
