use serde::Serialize;

#[derive(Default, Clone, Serialize)]
pub struct ImageAsset {
    key: String,
    label: Option<String>,
}

#[derive(Default, Clone, Serialize)]
pub struct Button {
    label: String,
    url: String,
}

#[derive(Default, Clone, Serialize)]
pub struct Timestamp {
    start: i64,
    end: Option<i64>,
}

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordRichPresence {
    details: Option<String>,
    state: Option<String>,
    large_image: Option<ImageAsset>,
    small_image: Option<ImageAsset>,
    buttons: Vec<Button>,
    timestamp: Option<Timestamp>,
}

impl ImageAsset {
    pub fn new() -> ImageAsset {
        ImageAsset::default()
    }

    pub fn with_key(&mut self, key: String) -> &mut ImageAsset {
        self.key = key;
        self
    }

    pub fn with_label(&mut self, label: String) -> &mut ImageAsset {
        self.label = Some(label);
        self
    }
}

// impl Button {
//     pub fn new() -> Button {
//         Button::default()
//     }

//     pub fn with_label(&mut self, label: String) -> &mut Button {
//         self.label = label;
//         self
//     }
//     pub fn with_url(&mut self, url: String) -> &mut Button {
//         self.url = url;
//         self
//     }
// }

impl Timestamp {
    pub fn new() -> Timestamp {
        Timestamp::default()
    }

    pub fn with_start(&mut self, start: i64) -> &mut Timestamp {
        self.start = start;
        self
    }
    pub fn with_end(&mut self, end: i64) -> &mut Timestamp {
        self.end = Some(end);
        self
    }
}

impl DiscordRichPresence {
    pub fn new() -> DiscordRichPresence {
        DiscordRichPresence::default()
    }

    pub fn apply(
        &mut self,
        details: Option<String>,
        state: Option<String>,
        large_image_key: Option<String>,
        large_image_label: Option<String>,
        small_image_key: Option<String>,
        small_image_label: Option<String>,
        buttons: Vec<Button>,
        timestamp_start: i64,
        timestamp_end: Option<i64>,
    ) -> &mut DiscordRichPresence {
        self.details = details;
        self.state = state;
        self.buttons = buttons;

        let mut timestamp = Timestamp::new();
        timestamp.with_start(timestamp_start);

        if let Some(timestamp_end) = timestamp_end {
            timestamp.with_end(timestamp_end);
        }

        self.timestamp = Some(timestamp);

        if let Some(large_image_key) = large_image_key {
            let mut large_image = ImageAsset::new();
            large_image.with_key(large_image_key);

            if let Some(large_image_label) = large_image_label {
                large_image.with_label(large_image_label);
            }

            self.large_image = Some(large_image);
        } else {
            self.large_image = None;
        }

        if let Some(small_image_key) = small_image_key {
            let mut small_image = ImageAsset::new();
            small_image.with_key(small_image_key);

            if let Some(small_image_label) = small_image_label {
                small_image.with_label(small_image_label);
            }

            self.small_image = Some(small_image);
        } else {
            self.small_image = None;
        }

        self
    }

    pub fn with_details(&mut self, details: String) -> &mut DiscordRichPresence {
        self.details = Some(details);
        self
    }

    pub fn with_state(&mut self, state: String) -> &mut DiscordRichPresence {
        self.state = Some(state);
        self
    }

    // pub fn with_large_image(&mut self, large_image: ImageAsset) -> &mut DiscordRichPresence {
    //     self.large_image = Some(large_image);
    //     self
    // }

    // pub fn with_small_image(&mut self, small_image: ImageAsset) -> &mut DiscordRichPresence {
    //     self.small_image = Some(small_image);
    //     self
    // }

    // pub fn with_buttons(&mut self, buttons: Vec<Button>) -> &mut DiscordRichPresence {
    //     self.buttons = buttons;
    //     self
    // }

    // pub fn with_timestamp(&mut self, timestamp: Timestamp) -> &mut DiscordRichPresence {
    //     self.timestamp = Some(timestamp);
    //     self
    // }
}
