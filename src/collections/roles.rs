use bonsaidb::core::{ key::Key, schema::Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Key, PartialOrd, Ord)]
#[repr(u8)]
pub enum RolePurpose {
    Rank0 = 0,
    Rank5 = 5,
    Rank10 = 10,
    Rank15 = 15,
    Rank20 = 20,
    Rank25 = 25,
    Rank30 = 30,
    Rank35 = 35,
    Rank40 = 40,
    Rank45 = 45,
    Rank50 = 50,
}

impl RolePurpose {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(Self::Rank0),
            5 => Some(Self::Rank5),
            10 => Some(Self::Rank10),
            15 => Some(Self::Rank15),
            20 => Some(Self::Rank20),
            25 => Some(Self::Rank25),
            30 => Some(Self::Rank30),
            35 => Some(Self::Rank35),
            40 => Some(Self::Rank40),
            45 => Some(Self::Rank45),
            50 => Some(Self::Rank50),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "roles", primary_key = RolePurpose)]
pub struct Roles {
    #[natural_id]
    pub role_purpose: RolePurpose,
    pub role_id: String,
}
