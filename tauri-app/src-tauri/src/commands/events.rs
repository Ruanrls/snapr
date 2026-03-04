use snapr::events::KeyboardListener;
use tauri::{command, Manager};

#[command]
pub fn start_listening_keyboard(app: tauri::AppHandle) -> Result<(), String> {
    let app_state = app.state::<crate::AppState>();

    KeyboardListener::start_keyboard_listener(
        app_state.command_storage.clone(),
        app_state.keyboard_event_sender.clone(),
    );

    Ok(())
}

#[command]
pub fn stop_listening_keyboard() -> Result<(), String> {
    KeyboardListener::stop_keyboard_listener();
    Ok(())
}
