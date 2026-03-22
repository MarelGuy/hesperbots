use bonsaidb::core::schema::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum RolePurpose {
    Rank0 = 0,
    Rank5 = 1,
    Rank10 = 2,
    Rank15 = 3,
    Rank20 = 4,
    Rank25 = 5,
    Rank30 = 6,
    Rank35 = 7,
    Rank40 = 8,
    Rank45 = 9,
    Rank50 = 10,
}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "roles", primary_key = String)]
pub struct Roles {
    #[natural_id]
    pub role_id: String,
    role_purpose: RolePurpose,
}
