use bonsaidb::core::schema::SerializedCollection;
use serenity::all::{CommandDataOptionValue, CommandInteraction, Context};

use std::fmt::Write;

use crate::{
    BoxError,
    collections::{ChannelPurpose, Channels, Purpose, RolePurpose, Roles},
    functions::{MessageTarget, reply},
    handler::Handler,
};

pub async fn list(
    handler: &Handler,
    command: CommandInteraction,
    ctx: Context,
    guild_id: String,
) -> Result<(), BoxError> {
    let CommandDataOptionValue::String(purpose) = &command.data.options.first().unwrap().value
    else {
        unreachable!()
    };

    let list = match Purpose::try_from(purpose)? {
        Purpose::ChannelPurpose => {
            let channels = Channels::all_async(&handler.db).await?;

            let mut list = String::new();

            for purpose in ChannelPurpose::all() {
                let channel = channels.iter().find(|c| {
                    c.contents.channel_purpose == *purpose && c.contents.guild_id == guild_id
                });

                if let Some(channel) = channel {
                    writeln!(
                        list,
                        "{}: {}",
                        *purpose,
                        channel.contents.channel_name.clone()
                    )?;
                } else {
                    writeln!(list, "{}: None", *purpose)?;
                }
            }

            list
        }
        Purpose::RolePurpose => {
            let purposes = RolePurpose::all();
            let roles = Roles::all_async(&handler.db).await?;

            let mut list = String::new();

            for purpose in purposes {
                let role = roles.iter().find(|r| {
                    r.contents.role_purpose == *purpose && r.contents.guild_id == guild_id
                });

                if let Some(role) = role {
                    writeln!(list, "{}: {}", *purpose, role.contents.role_name.clone())?;
                } else {
                    writeln!(list, "{}: None", *purpose)?;
                }
            }

            list
        }
    };

    reply(&ctx, MessageTarget::Interaction(&command), &list, 10).await?;

    Ok(())
}
