mod commands {
    use crate::monitor::monitor::{Bounds, Monitor, MonitorHandler};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

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
    }

    #[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
    pub struct KeyBinding {
        pub modifiers: u8, // bitmask,
        pub key: u32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Command {
        pub key_binding: KeyBinding,
        pub position: ScreenPositions,
    }

    impl Command {
        pub fn exec(&self) {
            let active_monitor = Monitor::get_active_monitor();
            let position = &self.position;

            let monitor_center = active_monitor.get_center();
            println!("MONITOR CENTER {:?}", monitor_center);

            match position {
                ScreenPositions::Top => active_monitor.set_position(&Bounds {
                    top: active_monitor.bounds.top,
                    right: active_monitor.bounds.right + SHADOW_BORDERS_SIZE * 2,
                    bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                    left: active_monitor.bounds.left - SHADOW_BORDERS_SIZE,
                }),
                ScreenPositions::Right => active_monitor.set_position(&Bounds {
                    top: active_monitor.bounds.top,
                    right: active_monitor.bounds.right - monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                    bottom: active_monitor.bounds.bottom + SHADOW_BORDERS_SIZE,
                    left: monitor_center.0 - SHADOW_BORDERS_SIZE,
                }),
                ScreenPositions::Bottom => active_monitor.set_position(&Bounds {
                    top: monitor_center.1 - SHADOW_BORDERS_SIZE,
                    right: active_monitor.bounds.right + SHADOW_BORDERS_SIZE * 2,
                    bottom: monitor_center.1 + SHADOW_BORDERS_SIZE * 2,
                    left: active_monitor.bounds.left - SHADOW_BORDERS_SIZE,
                }),
                ScreenPositions::Left => active_monitor.set_position(&Bounds {
                    top: active_monitor.bounds.top,
                    right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                    bottom: active_monitor.bounds.bottom + SHADOW_BORDERS_SIZE * 2,
                    left: active_monitor.bounds.left - SHADOW_BORDERS_SIZE,
                }),
                ScreenPositions::TopLeft => active_monitor.set_position(&Bounds {
                    top: active_monitor.bounds.top,
                    right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                    bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                    left: active_monitor.bounds.left - SHADOW_BORDERS_SIZE,
                }),
                ScreenPositions::TopRight => active_monitor.set_position(&Bounds {
                    top: active_monitor.bounds.top,
                    right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                    bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                    left: monitor_center.0 - SHADOW_BORDERS_SIZE,
                }),
                ScreenPositions::BottomRight => active_monitor.set_position(&Bounds {
                    top: monitor_center.1,
                    right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                    bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                    left: monitor_center.0 - SHADOW_BORDERS_SIZE,
                }),
                ScreenPositions::BottomLeft => active_monitor.set_position(&Bounds {
                    top: monitor_center.1,
                    right: monitor_center.0 + SHADOW_BORDERS_SIZE * 2,
                    bottom: monitor_center.1 + SHADOW_BORDERS_SIZE,
                    left: active_monitor.bounds.left - SHADOW_BORDERS_SIZE,
                }),
                _ => (),
            }
        }
    }

    pub struct CommandStorage {
        pub commands: Arc<RwLock<HashMap<KeyBinding, Command>>>,
    }

    pub trait CommandHandler {
        fn new() -> CommandStorage;
        fn add(&self, command: Command) -> Option<()>;
        fn remove(self, command: Command) -> Option<()>;
        fn has(self, key_binding: KeyBinding) -> Option<()>;
    }

    impl CommandHandler for CommandStorage {
        fn new() -> CommandStorage {
            CommandStorage {
                commands: Arc::new(RwLock::from(HashMap::new())),
            }
        }

        fn add(&self, command: Command) -> Option<()> {
            let mut command_storage = self.commands.write().unwrap(); // TODO: avoid unwrap
            command_storage.insert(command.key_binding, command);

            Some(())
        }

        fn remove(self, command: Command) -> Option<()> {
            let mut command_storage = self.commands.write().unwrap();
            command_storage.remove(&command.key_binding);

            Some(())
        }

        fn has(self, key_binding: KeyBinding) -> Option<()> {
            let command_storage = self.commands.read().unwrap();
            command_storage.contains_key(&key_binding);

            Some(())
        }
    }
}

pub use commands::*;
