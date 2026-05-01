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
pub enum RolePurpose {
    #[strum(serialize = "Rank0")]
    Rank0 = 0,
    #[strum(serialize = "Rank5")]
    Rank5 = 5,
    #[strum(serialize = "Rank10")]
    Rank10 = 10,
    #[strum(serialize = "Rank15")]
    Rank15 = 15,
    #[strum(serialize = "Rank20")]
    Rank20 = 20,
    #[strum(serialize = "Rank25")]
    Rank25 = 25,
    #[strum(serialize = "Rank30")]
    Rank30 = 30,
    #[strum(serialize = "Rank35")]
    Rank35 = 35,
    #[strum(serialize = "Rank40")]
    Rank40 = 40,
    #[strum(serialize = "Rank45")]
    Rank45 = 45,
    #[strum(serialize = "Rank50")]
    Rank50 = 50,
    #[strum(serialize = "Verified")]
    Verified,
}

impl RolePurpose {
    pub const fn all() -> &'static [Self] {
        &[
            Self::Rank0,
            Self::Rank5,
            Self::Rank10,
            Self::Rank15,
            Self::Rank20,
            Self::Rank25,
            Self::Rank30,
            Self::Rank35,
            Self::Rank40,
            Self::Rank45,
            Self::Rank50,
            Self::Verified,
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Roles {
    pub role_purpose: i32,
    pub role_id: String,
    pub role_name: String,
    pub guild_id: String,
}

impl Roles {
    pub async fn get(db: &PgPool, purpose: i32, guild_id: &str) -> Result<Option<Self>, BoxError> {
        Ok(
            sqlx::query_file_as!(Roles, "src/queries/get_role.sql", purpose, guild_id)
                .fetch_optional(db)
                .await?,
        )
    }

    pub async fn get_by_guild(db: &PgPool, guild_id: &str) -> Result<Vec<Self>, BoxError> {
        Ok(
            sqlx::query_file_as!(Roles, "src/queries/get_roles_by_guild.sql", guild_id)
                .fetch_all(db)
                .await?,
        )
    }
}
