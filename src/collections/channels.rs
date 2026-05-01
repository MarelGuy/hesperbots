use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use strum::{Display, EnumString, FromRepr};

use crate::BoxError;

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

impl Channels {
    pub async fn get(db: &PgPool, purpose: i32, guild_id: &str) -> Result<Option<Self>, BoxError> {
        Ok(
            sqlx::query_file_as!(Channels, "src/queries/get_channel.sql", purpose, guild_id)
                .fetch_optional(db)
                .await?,
        )
    }

    pub async fn get_by_guild(db: &PgPool, guild_id: &str) -> Result<Vec<Self>, BoxError> {
        Ok(
            sqlx::query_file_as!(Channels, "src/queries/get_channels_by_guild.sql", guild_id)
                .fetch_all(db)
                .await?,
        )
    }
}
