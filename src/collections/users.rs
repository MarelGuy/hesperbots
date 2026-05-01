use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::BoxError;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Users {
    pub userid: String,
    pub rank: i32,
    pub xp: i32,
    pub next_rank_xp: i32,
    pub zod_sign: String,
    pub colour: String,
    pub guildid: String,
}

impl Users {
    pub async fn get(db: &PgPool, user_id: &str) -> Result<Option<Self>, BoxError> {
        Ok(
            sqlx::query_file_as!(Users, "src/queries/get_user.sql", user_id)
                .fetch_optional(db)
                .await?,
        )
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), BoxError> {
        sqlx::query_file!(
            "src/queries/update_user.sql",
            self.rank,
            self.xp,
            self.next_rank_xp,
            self.zod_sign,
            self.colour,
            self.guildid,
            self.userid
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn insert(
        db: &PgPool,
        userid: &str,
        rank: i32,
        xp: i32,
        next_rank_xp: i32,
        zod_sign: &str,
        colour: &str,
        guildid: &str,
    ) -> Result<(), BoxError> {
        sqlx::query_file!(
            "src/queries/insert_user.sql",
            userid,
            rank,
            xp,
            next_rank_xp,
            zod_sign,
            colour,
            guildid
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
