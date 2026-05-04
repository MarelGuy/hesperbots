use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption,
};

use std::fmt::Write;

use crate::{
    collections::{ChannelPurpose, Channels, Purpose, RolePurpose, Roles},
    error::HesperError,
    functions::{MessageTarget, reply},
    handler::Handler,
};

pub async fn list(
    handler: &Handler,
    command: CommandInteraction,
    ctx: Context,
    guild_id: String,
) -> Result<(), HesperError> {
    let CommandDataOptionValue::String(purpose) = &command.data.options[1].value else {
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

    reply(&ctx, MessageTarget::CommandInteraction(&command), &list, 10).await?;

    Ok(())
}

pub fn define_list() -> CreateCommand {
    CreateCommand::new("list")
        .description("Command to check all associated and available purposes")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "purpose",
                "Purpose to list: RolePurpose, ChannelPurpose",
            )
            .add_string_choice("Role Purpose", "RolePurpose")
            .add_string_choice("Channel Purpose", "ChannelPurpose")
            .required(true),
        )
}
