use std::{collections::HashMap, sync::LazyLock};

use crate::commands::{Command, CommandHash, KeyBinding, ScreenPositions};

/// Virtual key codes (mirroring Win32 VK_* constants for cross-platform compilation)
mod vk {
    pub const RETURN: u32 = 13;
    pub const LEFT: u32 = 37;
    pub const UP: u32 = 38;
    pub const RIGHT: u32 = 39;
    pub const DOWN: u32 = 40;
    pub const C: u32 = 67;
    pub const I: u32 = 73;
    pub const K: u32 = 75;
    pub const L: u32 = 76;
    pub const O: u32 = 79;
}

/// Modifier bitmask: Win = 4
const MOD_WIN: u8 = 4;

pub(crate) static DEFAULT_COMMANDS: LazyLock<CommandHash> = LazyLock::new(|| {
    let defaults = [
        (MOD_WIN, vk::RIGHT, ScreenPositions::Right),
        (MOD_WIN, vk::UP, ScreenPositions::Top),
        (MOD_WIN, vk::DOWN, ScreenPositions::Bottom),
        (MOD_WIN, vk::LEFT, ScreenPositions::Left),
        (MOD_WIN, vk::I, ScreenPositions::TopLeft),
        (MOD_WIN, vk::O, ScreenPositions::TopRight),
        (MOD_WIN, vk::K, ScreenPositions::BottomLeft),
        (MOD_WIN, vk::L, ScreenPositions::BottomRight),
        (MOD_WIN, vk::C, ScreenPositions::Center),
        (MOD_WIN, vk::RETURN, ScreenPositions::Maximize),
    ];

    defaults
        .into_iter()
        .map(|(modifiers, key, position)| {
            let key_binding = KeyBinding { modifiers, key };
            (
                key_binding,
                Command {
                    key_binding,
                    position,
                },
            )
        })
        .collect()
});
