use serenity::all::{CommandDataOptionValue, CommandInteraction, Context};

use crate::{
    collections::Channels,
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

    Channels::remove(&handler.db, &channel_id.to_string()).await?;

    reply(&ctx, MessageTarget::Interaction(&command), "Removed", 10).await?;

    Ok(())
}
