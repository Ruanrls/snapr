use crate::events::Events;
use crate::monitor::{Bounds, Monitor, MonitorHandler};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use std::thread;

const SHADOW_BORDERS_SIZE: i32 = 7;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScreenPositions {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Center,
    Top,
    Right,
    Left,
    Bottom,
    Maximize,
}

impl ScreenPositions {
    pub fn get_bounds(&self, monitor_bounds: &Bounds, monitor_center: (i32, i32)) -> Bounds {
        match self {
            ScreenPositions::Top => Bounds {
                top: monitor_bounds.top,
                right: monitor_bounds.right + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                left: monitor_bounds.left - SHADOW_BORDERS_SIZE,
            },
            ScreenPositions::Right => Bounds {
                top: monitor_bounds.top,
                right: monitor_bounds.right - monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_bounds.bottom + SHADOW_BORDERS_SIZE,
                left: monitor_center.0 - SHADOW_BORDERS_SIZE,
            },
            ScreenPositions::Bottom => Bounds {
                top: monitor_center.1 - SHADOW_BORDERS_SIZE,
                right: monitor_bounds.right + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_center.1 + SHADOW_BORDERS_SIZE * 2,
                left: monitor_bounds.left - SHADOW_BORDERS_SIZE,
            },
            ScreenPositions::Left => Bounds {
                top: monitor_bounds.top,
                right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_bounds.bottom + SHADOW_BORDERS_SIZE * 2,
                left: monitor_bounds.left - SHADOW_BORDERS_SIZE,
            },
            ScreenPositions::TopLeft => Bounds {
                top: monitor_bounds.top,
                right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                left: monitor_bounds.left - SHADOW_BORDERS_SIZE,
            },
            ScreenPositions::TopRight => Bounds {
                top: monitor_bounds.top,
                right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                left: monitor_center.0 - SHADOW_BORDERS_SIZE,
            },
            ScreenPositions::BottomRight => Bounds {
                top: monitor_center.1,
                right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                left: monitor_center.0 - SHADOW_BORDERS_SIZE,
            },
            ScreenPositions::BottomLeft => Bounds {
                top: monitor_center.1,
                right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                left: monitor_bounds.left - SHADOW_BORDERS_SIZE,
            },
            ScreenPositions::Center => Bounds {
                top: monitor_center.1 / 2,
                right: monitor_bounds.right - ((monitor_center.0 / 2) * 2) + SHADOW_BORDERS_SIZE,
                bottom: (monitor_center.0 / 2) + SHADOW_BORDERS_SIZE,
                left: monitor_center.0 / 2,
            },
            ScreenPositions::Maximize => Bounds {
                top: monitor_bounds.top,
                right: monitor_bounds.right + SHADOW_BORDERS_SIZE * 2,
                bottom: monitor_bounds.bottom + SHADOW_BORDERS_SIZE * 2,
                left: monitor_bounds.left - SHADOW_BORDERS_SIZE,
            },
        }
    }
}

pub type CommandHash = HashMap<KeyBinding, Command>;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub modifiers: u8, // bitmask,
    pub key: u32,
}

impl fmt::Debug for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KeyBinding {{ modifiers: {}, key: {:?} }}",
            self.modifiers,
            char::from_u32(self.key).unwrap_or('?')
        )
    }
}

impl KeyBinding {
    pub fn to_storage_key(&self) -> String {
        format!("{};{}", self.key, self.modifiers)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    pub key_binding: KeyBinding,
    pub position: ScreenPositions,
}

impl Command {
    pub fn exec(&self) {
        let active_monitor = Monitor::get_active_monitor();
        let position = &self.position;

        let monitor_center = active_monitor.get_center();
        let position_bounds = position.get_bounds(&active_monitor.bounds, monitor_center);
        active_monitor.set_position(&position_bounds);
    }
}

pub struct CommandStorage {
    pub commands: RwLock<CommandHash>,
}

pub(crate) trait CommandHandler {
    fn new() -> CommandStorage;
    fn add(&self, command: Command);
    fn remove(&self, key_binding: KeyBinding);
    fn get(&self, key_binding: KeyBinding) -> Option<Command>;
}

impl CommandHandler for CommandStorage {
    fn new() -> CommandStorage {
        CommandStorage {
            commands: RwLock::from(HashMap::new()),
        }
    }

    fn add(&self, command: Command) {
        let mut commands = self
            .commands
            .write()
            .expect("Command storage lock poisoned");
        commands.insert(command.key_binding, command);
    }

    fn remove(&self, keybinding: KeyBinding) {
        let mut commands = self
            .commands
            .write()
            .expect("Command storage lock poisoned");
        commands.remove(&keybinding);
    }

    fn get(&self, key_binding: KeyBinding) -> Option<Command> {
        let commands = self.commands.read().expect("Command storage lock poisoned");
        commands.get(&key_binding).cloned()
    }
}

pub fn listen_commands(receiver: Receiver<Events>, command_storage: Arc<CommandStorage>) {
    thread::spawn(move || {
        for message in receiver {
            match message {
                Events::KeyboardEvent(key_binding) => {
                    println!("Received keyboard event: {:?}", key_binding);
                    if let Some(command) = command_storage.get(key_binding) {
                        command.exec();
                    }
                }
            }
        }
    });
}
