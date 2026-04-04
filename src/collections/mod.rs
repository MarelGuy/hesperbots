mod channels;
mod roles;
mod users;

use bonsaidb::core::schema::Schema;

pub use channels::{ChannelPurpose, Channels};
pub use roles::{RolePurpose, Roles};
pub use users::Users;

use crate::BoxError;

#[derive(Schema)]
#[schema(name = "hesper-schema", collections = [Users, Channels, Roles])]
pub struct HesperSchema;

pub enum Purpose {
    RolePurpose,
    ChannelPurpose,
}

impl TryFrom<&String> for Purpose {
    type Error = BoxError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "RolePurpose" => Ok(Purpose::RolePurpose),
            "ChannelPurpose" => Ok(Purpose::ChannelPurpose),
            _ => Err("Unkown Purpose".into()),
        }
    }
}
