use std::sync::Mutex;

use chrono::Utc;
use declarative_discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DeclarativeDiscordIpcClient,
};
use lazy_static::lazy_static;
use tauri::{App, AppHandle, Emitter};

use crate::structs::discord::DiscordRichPresence;

lazy_static! {
    static ref DRPC_CLIENT: Mutex<Option<DeclarativeDiscordIpcClient>> = Mutex::new(Some(
        DeclarativeDiscordIpcClient::new("1307868749176504483")
    ));
}

pub fn connect_rich_presence(app: &App) -> Result<(), String> {
    let mut drpc = DRPC_CLIENT.lock().unwrap();

    if let Some(ref mut client) = *drpc {
        client.enable();

        client
            .set_activity(
                Activity::new()
                    .details("Iniciando")
                    .state("O launcher está iniciando..."),
            )
            .expect("DiscordRichPresenceError: Unable to set the activity");

        let mut discord_rich_presence = DiscordRichPresence::new();

        discord_rich_presence
            .with_details("Iniciando".to_string())
            .with_state("O launcher está iniciando...".to_string());

        let _ = app
            .emit::<Option<DiscordRichPresence>>("current_activity", Some(discord_rich_presence));

        Ok(())
    } else {
        Err("DiscordRichPresenceError: Discord client is uninitialized".to_string())
    }
}

#[tauri::command]
pub fn update_activity(
    app: AppHandle,
    details: Option<String>,
    state: Option<String>,
    large_image_key: Option<String>,
    large_image_label: Option<String>,
    small_image_key: Option<String>,
    small_image_label: Option<String>,
) -> Result<(), String> {
    let timestamp_start = Utc::now().timestamp_millis();

    let mut drpc = DRPC_CLIENT.lock().unwrap();

    if let Some(ref mut client) = *drpc {
        client
            .set_activity(
                Activity::new()
                    .details(details.clone().unwrap_or_default().as_ref())
                    .state(state.clone().unwrap_or_default().as_ref())
                    .assets(
                        Assets::new()
                            .large_image(large_image_key.clone().unwrap_or_default().as_ref())
                            .large_text(large_image_label.clone().unwrap_or_default().as_ref())
                            .small_image(small_image_key.clone().unwrap_or_default().as_ref())
                            .small_text(small_image_label.clone().unwrap_or_default().as_ref()),
                    )
                    .timestamps(Timestamps::new().start(timestamp_start.clone())),
            )
            .expect("DiscordRichPresenceError: Unable to set the activity");

        let mut discord_rich_presence = DiscordRichPresence::new();

        discord_rich_presence.apply(
            details,
            state,
            large_image_key,
            large_image_label,
            small_image_key,
            small_image_label,
            vec![],
            timestamp_start,
            None,
        );

        let _ = app
            .emit::<Option<DiscordRichPresence>>("current_activity", Some(discord_rich_presence));

        Ok(())
    } else {
        Err("DiscordRichPresenceError: Discord client is uninitialized".to_string())
    }
}

#[tauri::command]
pub fn clear_activity(app: AppHandle) -> Result<(), String> {
    let mut drpc = DRPC_CLIENT.lock().unwrap();

    if let Some(ref mut client) = *drpc {
        client
            .clear_activity()
            .expect("DiscordRichPresenceError: Unable to clear the activity");

        let _ = app.emit::<Option<DiscordRichPresence>>("current_activity", None);

        Ok(())
    } else {
        Err("DiscordRichPresenceError: Discord client is uninitialized".to_string())
    }
}
