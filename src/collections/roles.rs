use bonsaidb::core::{key::Key, schema::Collection};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

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
    Key,
    Display,
    EnumString,
)]
#[repr(u8)]
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
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "roles", primary_key = RolePurpose)]
pub struct Roles {
    #[natural_id]
    pub role_purpose: RolePurpose,
    pub role_id: String,
}
