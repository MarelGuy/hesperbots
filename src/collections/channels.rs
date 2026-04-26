use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum::{Display, EnumString, FromRepr};

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
    Display,
    EnumString,
    FromRepr,
)]
#[repr(i32)]
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

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Channels {
    pub channel_purpose: i32,
    pub channel_id: String,
    pub channel_name: String,
    pub guild_id: String,
}
