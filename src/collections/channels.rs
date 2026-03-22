use bonsaidb::core::schema::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChannelPurpose {
    RankChannel = 0,
}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "channels", primary_key = String)]
pub struct Channels {
    #[natural_id]
    pub channel_id: String,
    channel_purpose: ChannelPurpose,
}
