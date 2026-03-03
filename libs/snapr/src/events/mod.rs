mod windows;
#[cfg(windows)]
pub use windows::windows::*;

use crate::commands::KeyBinding;

pub enum Events {
    KeyboardEvent(KeyBinding),
}
