use tauri::{command, Manager};

#[command]
pub fn stop_listening_keyboard(app: tauri::AppHandle) -> Result<(), String> {
    let app_state = app.state::<crate::AppState>();
    app_state.keyboard_listener.stop_keyboard_listener();
    Ok(())
}
