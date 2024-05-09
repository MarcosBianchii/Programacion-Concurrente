use serde::Deserialize;

#[derive(Deserialize)]
pub struct Video {
    #[serde(rename = "channel_title")]
    pub channel: String,
    pub views: u128,
}
