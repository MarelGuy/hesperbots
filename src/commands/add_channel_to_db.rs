use std::str::FromStr;

use serenity::all::{CommandDataOptionValue, CommandInteraction, Context};

use crate::{
    collections::{ChannelPurpose, Channels},
    error::HesperError,
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

    let discord_channel = ctx.http.get_channel(*channel_id).await?;

    let channel = Channels {
        guild_id,
        channel_id: channel_id.to_string(),
        channel_name: discord_channel.to_string(),
        channel_purpose: ChannelPurpose::from_str(purpose.as_str())? as i32,
    };

    channel.insert(&handler.db).await?;

    Ok(())
}
