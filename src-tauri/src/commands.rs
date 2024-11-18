use std::sync::Mutex;

use chrono::Utc;
use lazy_static::lazy_static;

use discord_presence::Client;

lazy_static! {
    static ref DRPC_CLIENT: Mutex<Option<Client>> =
        Mutex::new(Some(Client::new(1307868749176504483)));
}

#[tauri::command]
pub fn start_rpc() -> Result<(), String> {
    let mut drpc = DRPC_CLIENT.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut client) = *drpc {
        client.start();
        Ok(())
    } else {
        Err("Discord IPC client not initialized".to_string())
    }
}

#[tauri::command]
pub fn set_activity(
    state: Option<String>,
    details: Option<String>,
    large_image_key: Option<String>,
    large_image_label: Option<String>,
    small_image_key: Option<String>,
    small_image_label: Option<String>,
) -> Result<(), String> {
    let timestamp_start: u64 = Utc::now().timestamp_millis() as u64;

    let mut drpc = DRPC_CLIENT.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut client) = *drpc {
        client
            .set_activity(|activity| {
                activity
                    .state(state.unwrap_or_default())
                    .details(details.unwrap_or_default())
                    .assets(|assets| {
                        assets
                            .large_image(large_image_key.unwrap_or_default())
                            .large_text(large_image_label.unwrap_or_default())
                            .small_image(small_image_key.unwrap_or_default())
                            .small_text(small_image_label.unwrap_or_default())
                    })
                    .timestamps(|timestamp| timestamp.start(timestamp_start))
            })
            .expect("Failed to set activity");
        Ok(())
    } else {
        Err("Discord IPC client not initialized".to_string())
    }
}
