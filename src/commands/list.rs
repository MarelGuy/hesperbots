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
            let channels: Vec<Channels> = Channels::get_by_guild(&handler.db, &guild_id).await?;

            let mut list = String::new();

            for purpose in ChannelPurpose::all() {
                let channel = channels
                    .iter()
                    .find(|c| c.channel_purpose == *purpose as i32);

                if let Some(channel) = channel {
                    writeln!(list, "{}: {}", *purpose, channel.channel_name.clone())?;
                } else {
                    writeln!(list, "{}: None", *purpose)?;
                }
            }

            list
        }
        Purpose::RolePurpose => {
            let purposes = RolePurpose::all();
            let roles: Vec<Roles> = Roles::get_by_guild(&handler.db, &guild_id).await?;

            let mut list = String::new();

            for purpose in purposes {
                let role = roles.iter().find(|r| r.role_purpose == *purpose as i32);

                if let Some(role) = role {
                    writeln!(list, "{}: {}", *purpose, role.role_name.clone())?;
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
