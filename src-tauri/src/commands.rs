use std::sync::Mutex;

use tauri::State;

use crate::libs::discord::{DiscordRichPresenceManager, Presence};

#[tauri::command]
pub fn connect_presence(state: State<'_, Mutex<DiscordRichPresenceManager>>) -> Result<(), String> {
    let state = state.lock().unwrap();

    state.enable();

    Ok(())
}

#[tauri::command]
pub fn disconnect_presence(
    state: State<'_, Mutex<DiscordRichPresenceManager>>,
) -> Result<(), String> {
    let state = state.lock().unwrap();

    state.disable();

    Ok(())
}

#[tauri::command]
pub fn set_presence(
    state: State<'_, Mutex<DiscordRichPresenceManager>>,
    details: Option<String>,
    presence_state: Option<String>,
    large_image_key: Option<String>,
    large_image_label: Option<String>,
    small_image_key: Option<String>,
    small_image_label: Option<String>,
    buttons_labels: Option<Vec<String>>,
    buttons_urls: Option<Vec<String>>,
    timestamp_start: i64,
    timestamp_end: Option<i64>,
) -> Result<(), String> {
    let state = state.lock().unwrap();

    let _ = state
        .set_activity(Presence {
            details,
            state: presence_state,
            large_image_key,
            large_image_label,
            small_image_key,
            small_image_label,
            buttons_labels,
            buttons_urls,
            timestamp_start,
            timestamp_end,
        })
        .unwrap();

    Ok(())
}

#[tauri::command]
pub fn clear_presence(state: State<'_, Mutex<DiscordRichPresenceManager>>) -> Result<(), String> {
    let state = state.lock().unwrap();

    let _ = state.clear_presence().unwrap();

    Ok(())
}
