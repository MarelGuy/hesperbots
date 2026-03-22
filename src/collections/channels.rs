use bonsaidb::core::{key::Key, schema::Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Key, PartialOrd, Ord)]
#[repr(u8)]
pub enum ChannelPurpose {
    RankChannel = 0,
}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "channels", primary_key = ChannelPurpose)]
pub struct Channels {
    #[natural_id]
    pub channel_purpose: ChannelPurpose,
    pub channel_id: String,
}
