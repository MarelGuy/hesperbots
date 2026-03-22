mod channels;
mod roles;
mod users;

use bonsaidb::core::schema::Schema;

pub use channels::{ChannelPurpose, Channels};
pub use roles::{RolePurpose, Roles};
pub use users::Users;

#[derive(Schema)]
#[schema(name = "hesper-schema", collections = [Users, Channels, Roles])]
pub struct HesperSchema;
