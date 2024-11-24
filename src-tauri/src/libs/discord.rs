use std::error::Error;

use declarative_discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DeclarativeDiscordIpcClient,
};
use serde::Serialize;

const CLIENT_ID: &str = "1307868749176504483";

#[derive(Serialize)]
pub struct Presence {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image_key: Option<String>,
    pub large_image_label: Option<String>,
    pub small_image_key: Option<String>,
    pub small_image_label: Option<String>,
    pub buttons_labels: Option<Vec<String>>,
    pub buttons_urls: Option<Vec<String>>,
    pub timestamp_start: i64,
    pub timestamp_end: Option<i64>,
}

pub struct DiscordRichPresenceManager {
    client: DeclarativeDiscordIpcClient,
}

impl DiscordRichPresenceManager {
    pub fn new() -> DiscordRichPresenceManager {
        DiscordRichPresenceManager {
            client: DeclarativeDiscordIpcClient::new(CLIENT_ID),
        }
    }

    pub fn enable(&self) {
        self.client.enable();
    }

    pub fn disable(&self) {
        self.client.disable();
    }

    pub fn set_activity(&self, presence: Presence) -> Result<Presence, Box<dyn Error>> {
        let mut activity = Activity::new();
        let mut assets = Assets::new();
        let mut buttons: Vec<Button> = vec![];
        let mut timestamps = Timestamps::new().start(presence.timestamp_start);

        if let Some(end) = presence.timestamp_end {
            timestamps = timestamps.end(end);
        }

        if let Some(ref details) = presence.details {
            activity = activity.details(details);
        }

        if let Some(ref state) = presence.state {
            activity = activity.state(state);
        }

        if let Some(ref large_image_key) = presence.large_image_key {
            assets = assets.large_image(large_image_key);
            if let Some(ref large_image_label) = presence.large_image_label {
                assets = assets.large_text(large_image_label);
            }
        }

        if let Some(ref small_image_key) = presence.small_image_key {
            assets = assets.small_image(small_image_key);
            if let Some(ref small_image_label) = presence.small_image_label {
                assets = assets.small_text(small_image_label);
            }
        }

        if let (Some(ref labels), Some(ref urls)) =
            (&presence.buttons_labels, &presence.buttons_urls)
        {
            for (label, url) in labels.iter().zip(urls.iter()) {
                buttons.push(Button::new(label.clone(), url.clone()));
            }
        }

        activity = activity
            .assets(assets)
            .buttons(buttons)
            .timestamps(timestamps);

        self.client
            .set_activity(activity)
            .map_err(|e| e.to_string())?;

        Ok(presence)
    }

    pub fn clear_presence(&self) -> Result<(), Box<dyn Error>> {
        self.client.clear_activity()
    }
}
