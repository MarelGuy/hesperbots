use bonsaidb::core::schema::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "users", primary_key = String)]
pub struct User {
    #[natural_id]
    pub userid: String,
    pub rank: u16,
    pub xp: u16,
    pub next_rank_xp: u16,
    pub zod_sign: String,
    pub colour: String,
}
