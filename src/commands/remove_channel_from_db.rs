use serenity::all::{CommandDataOptionValue, CommandInteraction, Context};

use crate::{
    collections::{ChannelPurpose, Channels},
    error::HesperError,
    functions::{MessageTarget, reply},
    handler::Handler,
};

pub async fn remove_channel_from_db(
    handler: &Handler,
    command: CommandInteraction,
    ctx: Context,
) -> Result<(), HesperError> {
    let CommandDataOptionValue::Channel(channel_id) = &command.data.options[0].value else {
        return Err("Channel not found".into());
    };

    let channel_id = channel_id.to_string();

    let Some(channel) = Channels::get_by_id(&handler.db, &channel_id).await? else {
        reply(
            &ctx,
            MessageTarget::CommandInteraction(&command),
            "Channel not found in the database",
            10,
        )
        .await?;

        return Err("Called remove_channel_from_db with 404".into());
    };

    let Some(channel_purpose) = ChannelPurpose::from_repr(channel.channel_purpose) else {
        reply(
            &ctx,
            MessageTarget::CommandInteraction(&command),
            "Internal error, check bot logs",
            10,
        )
        .await?;

        return Err("Internal error: Channel purpose conversion went wrong".into());
    };

    Channels::remove(&handler.db, &channel_id).await?;

    reply(
        &ctx,
        MessageTarget::CommandInteraction(&command),
        &format!("Removed {} as {}", channel.channel_name, channel_purpose),
        10,
    )
    .await?;

    Ok(())
}
