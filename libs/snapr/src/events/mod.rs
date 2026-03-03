use crate::commands::{CommandStorage, KeyBinding};
use std::sync::{Arc, mpsc::Sender};

#[cfg(windows)]
mod windows;

pub enum Events {
    KeyboardEvent(KeyBinding),
}
// pub trait KeyboardListening {
//     fn start_keyboard_listener(
//         command_storage: Arc<CommandStorage>,
//         event_sender: Sender<Events>,
//     ) -> Self;
//     fn stop_keyboard_listener(&self);
// }

#[cfg(windows)]
pub type KeyboardListener = windows::WindowsKeyboardListener;
