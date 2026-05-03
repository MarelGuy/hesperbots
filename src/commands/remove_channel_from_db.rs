use serenity::all::{CommandDataOptionValue, CommandInteraction};

use crate::{collections::Channels, error::HesperError, handler::Handler};

pub async fn remove_channel_from_db(
    handler: &Handler,
    command: CommandInteraction,
) -> Result<(), HesperError> {
    let CommandDataOptionValue::Channel(dchannel_id) = &command.data.options[0].value else {
        return Err("Channel not found".into());
    };

    Channels::remove(&handler.db, &dchannel_id.to_string()).await?;

    Ok(())
}
