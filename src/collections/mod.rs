mod channels;
mod roles;
mod users;

pub use channels::{ChannelPurpose, Channels};
pub use roles::{RolePurpose, Roles};
pub use users::Users;

use crate::BoxError;

pub enum Purpose {
    RolePurpose,
    ChannelPurpose,
}

impl TryFrom<&String> for Purpose {
    type Error = BoxError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "RolePurpose" => Ok(Self::RolePurpose),
            "ChannelPurpose" => Ok(Self::ChannelPurpose),
            _ => Err("Unkown Purpose".into()),
        }
    }
}
