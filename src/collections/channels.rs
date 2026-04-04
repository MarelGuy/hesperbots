use bonsaidb::core::{key::Key, schema::Collection};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Key,
    Display,
    EnumString,
)]
#[repr(u8)]
pub enum ChannelPurpose {
    #[strum(serialize = "RankChannel")]
    RankChannel = 0,
    #[strum(serialize = "DeletedMessagesChannel")]
    DeletedMessagesChannel = 1,
}

impl ChannelPurpose {
    pub const fn all() -> &'static [Self] {
        &[Self::RankChannel, Self::DeletedMessagesChannel]
    }
}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "channels", primary_key = ChannelPurpose)]
pub struct Channels {
    #[natural_id]
    pub channel_purpose: ChannelPurpose,
    pub channel_id: String,
}
