use crate::commands::KeyBinding;

#[cfg(windows)]
mod windows;

pub enum Events {
    KeyboardEvent(KeyBinding),
}

#[cfg(windows)]
pub type KeyboardListener = windows::WindowsKeyboardListener;
