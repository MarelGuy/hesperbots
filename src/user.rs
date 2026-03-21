use bonsaidb::core::schema::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "users", primary_key = String)]
pub struct User {
    #[natural_id]
    pub userid: String,
    pub rank: usize,
    pub xp: usize,
    pub next_rank_xp: usize,
    pub zod_sign: String,
    pub colour: String,
}
