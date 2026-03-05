use std::{collections::HashMap, sync::LazyLock};

use crate::commands::{Command, CommandHash, KeyBinding, ScreenPositions};

pub(crate) static DEFAULT_COMMANDS: LazyLock<CommandHash> = LazyLock::new(|| {
    let defaults = [
        (4, 39, ScreenPositions::Right),
        (4, 38, ScreenPositions::Top),
        (4, 40, ScreenPositions::Bottom),
        (4, 37, ScreenPositions::Left),
        (4, 73, ScreenPositions::TopLeft),
        (4, 79, ScreenPositions::TopRight),
        (4, 75, ScreenPositions::BottomLeft),
        (4, 76, ScreenPositions::BottomRight),
        (4, 67, ScreenPositions::Center),
        (4, 13, ScreenPositions::Maximize),
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
