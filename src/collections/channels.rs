use core::fmt;
use std::str::FromStr;

use bonsaidb::core::{key::Key, schema::Collection};
use serde::{Deserialize, Serialize};

use crate::BoxError;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Key, PartialOrd, Ord)]
#[repr(u8)]
pub enum ChannelPurpose {
    RankChannel = 0,
}

impl fmt::Display for ChannelPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelPurpose::RankChannel => write!(f, "RankChannel"),
        }
    }
}

impl FromStr for ChannelPurpose {
    type Err = BoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RankChannel" => Ok(ChannelPurpose::RankChannel),
            _ => Err(format!("Unknown ChannelPurpose: {s}").into()),
        }
    }
}

impl ChannelPurpose {
    pub const fn all() -> &'static [Self] {
        &[Self::RankChannel]
    }
}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "channels", primary_key = ChannelPurpose)]
pub struct Channels {
    #[natural_id]
    pub channel_purpose: ChannelPurpose,
    pub channel_id: String,
}
