use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
