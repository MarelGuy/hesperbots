use core::fmt;
use std::str::FromStr;

use bonsaidb::core::{key::Key, schema::Collection};
use serde::{Deserialize, Serialize};

use crate::BoxError;

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

impl fmt::Display for RolePurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rank0 => write!(f, "Rank 0"),
            Self::Rank5 => write!(f, "Rank 5"),
            Self::Rank10 => write!(f, "Rank 10"),
            Self::Rank15 => write!(f, "Rank 15"),
            Self::Rank20 => write!(f, "Rank 20"),
            Self::Rank25 => write!(f, "Rank 25"),
            Self::Rank30 => write!(f, "Rank 30"),
            Self::Rank35 => write!(f, "Rank 35"),
            Self::Rank40 => write!(f, "Rank 40"),
            Self::Rank45 => write!(f, "Rank 45"),
            Self::Rank50 => write!(f, "Rank 50"),
        }
    }
}

impl FromStr for RolePurpose {
    type Err = BoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s
            .strip_prefix("Rank")
            .ok_or("Missing 'Rank' prefix")?
            .parse::<u8>()?;

        match val {
            0 => Ok(Self::Rank0),
            5 => Ok(Self::Rank5),
            10 => Ok(Self::Rank10),
            15 => Ok(Self::Rank15),
            20 => Ok(Self::Rank20),
            25 => Ok(Self::Rank25),
            30 => Ok(Self::Rank30),
            35 => Ok(Self::Rank35),
            40 => Ok(Self::Rank40),
            45 => Ok(Self::Rank45),
            50 => Ok(Self::Rank50),
            _ => Err(format!("Unsupported rank value: {val}").into()),
        }
    }
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
