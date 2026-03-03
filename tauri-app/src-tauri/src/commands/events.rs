use snapr::events::KeyboardListener;
use tauri::{command, Manager};

#[command]
pub fn start_listening_keyboard(app: tauri::AppHandle) -> Result<(), String> {
    let app_state = app.state::<crate::AppState>();

    let new_listener = KeyboardListener::start_keyboard_listener(
        app_state.command_storage.clone(),
        app_state.keyboard_event_sender.clone(),
    );

    let mut listener = app_state
        .keyboard_listener
        .lock()
        .expect("Keyboard listener mutex poisoned");
    *listener = new_listener;

    Ok(())
}

#[command]
pub fn stop_listening_keyboard(app: tauri::AppHandle) -> Result<(), String> {
    let app_state = app.state::<crate::AppState>();
    let listener = app_state
        .keyboard_listener
        .lock()
        .expect("Keyboard listener mutex poisoned");
    listener.stop_keyboard_listener();
    Ok(())
}
