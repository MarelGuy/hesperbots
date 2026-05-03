use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use strum::{Display, EnumString, FromRepr};

use crate::error::HesperError;

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
    pub async fn get(
        db: &PgPool,
        purpose: i32,
        guild_id: &str,
    ) -> Result<Option<Self>, HesperError> {
        Ok(
            sqlx::query_file_as!(Channels, "src/queries/get_channel.sql", purpose, guild_id)
                .fetch_optional(db)
                .await?,
        )
    }

    pub async fn get_by_guild(db: &PgPool, guild_id: &str) -> Result<Vec<Self>, HesperError> {
        Ok(
            sqlx::query_file_as!(Channels, "src/queries/get_channels_by_guild.sql", guild_id)
                .fetch_all(db)
                .await?,
        )
    }

    pub async fn get_by_id(db: &PgPool, channel_id: &str) -> Result<Option<Self>, HesperError> {
        Ok(
            sqlx::query_file_as!(Channels, "src/queries/get_channel_by_id.sql", channel_id)
                .fetch_optional(db)
                .await?,
        )
    }

    pub async fn insert(&self, db: &PgPool) -> Result<(), HesperError> {
        sqlx::query_file!(
            "src/queries/insert_channel.sql",
            self.channel_purpose,
            self.channel_id,
            self.channel_name,
            self.guild_id
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn remove(db: &PgPool, channel_id: &str) -> Result<(), HesperError> {
        sqlx::query_file!("src/queries/remove_channel.sql", channel_id)
            .execute(db)
            .await?;
        Ok(())
    }
}
