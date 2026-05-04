use std::str::FromStr;

use serenity::all::{CommandDataOptionValue, CommandInteraction, Context};

use crate::{
    collections::{ChannelPurpose, Channels},
    error::HesperError,
    functions::{MessageTarget, reply},
    handler::Handler,
};

pub async fn add_channel_to_db(
    handler: &Handler,
    command: CommandInteraction,
    ctx: Context,
    guild_id: String,
) -> Result<(), HesperError> {
    let CommandDataOptionValue::Channel(channel_id) = &command.data.options[0].value else {
        return Err("Channel not found".into());
    };

    let CommandDataOptionValue::String(purpose) = &command.data.options[1].value else {
        return Err("Error parsing purpose".into());
    };

    if let Some(channel) = Channels::get(
        &handler.db,
        ChannelPurpose::from_str(purpose)? as i32,
        &guild_id,
    )
    .await?
    {
        reply(
            &ctx,
            MessageTarget::CommandInteraction(&command),
            format!(
                "Channel {} already assigned to a purpose: {}",
                channel.channel_name, channel.channel_purpose
            )
            .as_str(),
            10,
        )
        .await?;

        return Err("Called add_channel_to_db with code 409".into());
    }

    let discord_channel = ctx.http.get_channel(*channel_id).await?.to_string();

    let channel = Channels {
        guild_id,
        channel_id: channel_id.to_string(),
        channel_name: discord_channel.clone(),
        channel_purpose: ChannelPurpose::from_str(purpose.as_str())? as i32,
    };

    channel.insert(&handler.db).await?;

    reply(
        &ctx,
        MessageTarget::CommandInteraction(&command),
        format!("Added {discord_channel} as {purpose} to db").as_str(),
        10,
    )
    .await?;

    Ok(())
}
