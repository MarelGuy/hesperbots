use serenity::all::{CommandInteraction, Context, CreateCommand};
use tracing::error;

use crate::functions::reply;

pub async fn help(command: CommandInteraction, ctx: Context) {
    if let Err(why) = reply(
        &ctx,
        crate::functions::MessageTarget::CommandInteraction(&command),
        "/help: This command.\n/add_role_to_db: Changes or adds a role associated with a RankPurpose\n/list <RankPurpose or ChannelPurpose>: Returns a list of all Purpose types that can be (and are) associated with a rank or channel",
        10,
    )
    .await
    {
        error!("Cannot respond to slash command: {why}");
    }
}

pub fn define_help() -> CreateCommand {
    CreateCommand::new("help").description("Help command to check available commands")
}
