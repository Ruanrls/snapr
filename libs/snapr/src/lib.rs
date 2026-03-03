use crate::{
    commands::{CommandHandler, CommandHash, CommandStorage},
    configuration::{
        ConfigurationError, UserConfiguration, defaults::DEFAULT_COMMANDS, save_config,
    },
};

pub mod commands;
pub mod configuration;
pub mod events;
mod monitor;

pub struct InitializeCommandsConfig {
    pub path: String,
}

pub fn initialize_commands(
    params: InitializeCommandsConfig,
) -> Result<CommandStorage, ConfigurationError> {
    let command_storage = CommandStorage::new();

    match configuration::load_config(&params.path) {
        Ok(user_configuration) => {
            for (_, command) in &user_configuration.commands {
                command_storage.add(command.clone());
            }
        }
        Err(ConfigurationError::ConfigNotFound(_)) => {
            for (_, command) in DEFAULT_COMMANDS.iter() {
                command_storage.add(command.clone());
            }
        }
        Err(e) => return Err(e),
    }

    Ok(command_storage)
}

pub fn update_keybinding(
    command_storage: &CommandStorage,
    new_command: commands::Command,
    path: &str,
) -> Result<(), ConfigurationError> {
    let mut writable_commands = command_storage
        .commands
        .write()
        .expect("Command storage lock poisoned");
    writable_commands.insert(new_command.key_binding, new_command);

    save_user_config(&writable_commands, path)
}

pub fn remove_keybinding(
    command_storage: &CommandStorage,
    keybinding: commands::KeyBinding,
    path: &str,
) -> Result<(), ConfigurationError> {
    command_storage.remove(keybinding);

    let readable_commands = command_storage
        .commands
        .read()
        .expect("Command storage lock poisoned");
    save_user_config(&readable_commands, path)
}

fn save_user_config(command_storage: &CommandHash, path: &str) -> Result<(), ConfigurationError> {
    let user_configuration = UserConfiguration {
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
