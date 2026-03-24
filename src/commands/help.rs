use serenity::all::{CommandInteraction, Context};
use tracing::error;

use crate::functions::reply;

pub async fn help(command: CommandInteraction, ctx: Context) {
    if let Err(why) = reply(
        &ctx,
        crate::functions::MessageTarget::Interaction(&command),
        "/help: Questo comando.",
        10,
    )
    .await
    {
        error!("Cannot respond to slash command: {why}");
    }
}
