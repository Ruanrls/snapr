#[cfg(windows)]
pub mod windows {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};
    use std::thread;
    use std::{ptr::null_mut, thread::JoinHandle};

    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_RWIN,
        VK_SHIFT,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{KBDLLHOOKSTRUCT, WM_KEYDOWN, WM_SYSKEYDOWN};
    use windows_sys::Win32::{
        Foundation::{LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::{
            CallNextHookEx, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL,
        },
    };

    use crate::commands::{Command, KeyBinding};

    fn is_modifier(key_code: u32) -> bool {
        matches!(
            key_code as u16,
            VK_LCONTROL
                | VK_RCONTROL
                | VK_LSHIFT
                | VK_RSHIFT
                | VK_LWIN
                | VK_RWIN
                | VK_LMENU
                | VK_RMENU
        )
    }

    fn modifier_to_bitmap(modifier: u32) -> u8 {
        match modifier as u16 {
            VK_LCONTROL | VK_RCONTROL => 1,
            VK_LSHIFT | VK_SHIFT => 1 << 1,
            VK_LWIN | VK_RWIN => 1 << 2,
            VK_LMENU | VK_RMENU => 1 << 3,
            _ => 0,
        }
    }

    thread_local! {
        static KEY_STORAGE: RefCell<KeyBinding> = RefCell::new(KeyBinding {
            modifiers: 0,
            key: 0
        });
        static COMMANDS: RefCell<Option<Arc<RwLock<HashMap<KeyBinding, Command>>>>> = RefCell::new(None);
    }

    unsafe extern "system" fn hook_callback(
        code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        unsafe {
            if code < 0 {
                return CallNextHookEx(null_mut(), code, w_param, l_param);
            }

            let keyboard_event = *(l_param as *const KBDLLHOOKSTRUCT);
            let key_code = keyboard_event.vkCode;

            let is_keypress = w_param == WM_KEYDOWN as usize || w_param == WM_SYSKEYDOWN as usize;
            let is_modifier = is_modifier(key_code);

            let updated_key_binding = KEY_STORAGE.with_borrow_mut(|storage| {
                match (is_keypress, is_modifier) {
                    (true, true) => storage.modifiers |= modifier_to_bitmap(key_code),
                    (false, true) => storage.modifiers &= !modifier_to_bitmap(key_code),
                    (true, false) => {
                        println!("pressed key {}", key_code);
                        if storage.modifiers != 0 {
                            storage.key = key_code
                        }
                    }
                    (false, false) => storage.key = 0,
                }

                return *storage;
            });

            let is_command = COMMANDS.with_borrow(|commands_storage| {
                if let Some(commands_guard) = commands_storage {
                    if let Ok(commands_storage) = commands_guard.read() {
                        if commands_storage.contains_key(&updated_key_binding) {
                            let command = commands_storage.get(&updated_key_binding).unwrap();
                            println!("Executing command {:?}", command);
                            command.exec();
                            return true;
                        }
                    }
                }

                return false;
            });

            if is_command {
                return -1;
            }

            CallNextHookEx(null_mut(), code, w_param, l_param)
        }
    }

    pub fn start_listening(
        commands: Arc<RwLock<HashMap<KeyBinding, Command>>>,
    ) -> JoinHandle<Option<()>> {
        thread::spawn(move || {
            unsafe {
                COMMANDS.with(|f| {
                    *f.borrow_mut() = Some(commands);
                });

                let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_callback), null_mut(), 0);

                let mut msg = std::mem::zeroed();
                loop {
                    let message_response = GetMessageW(&mut msg, null_mut(), 0, 0);

                    if message_response == 0 {
                        UnhookWindowsHookEx(hook);
                        return Some(());
                    }

                    if message_response < 0 {
                        UnhookWindowsHookEx(hook);
                        panic!("Something went wrong while receiving the message");
                    }
                }
            };
        })
    }
}
