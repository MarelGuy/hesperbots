use serenity::all::{CommandDataOptionValue, CommandInteraction, Context};

use crate::{
    collections::Roles,
    error::HesperError,
    functions::{MessageTarget, reply},
    handler::Handler,
};

pub async fn remove_role_from_db(
    handler: &Handler,
    command: CommandInteraction,
    ctx: Context,
) -> Result<(), HesperError> {
    let CommandDataOptionValue::Role(role_id) = &command.data.options[0].value else {
        return Err("Role not found".into());
    };

    Roles::remove(&handler.db, &role_id.to_string()).await?;

    reply(&ctx, MessageTarget::Interaction(&command), "Removed", 10).await?;

    Ok(())
}
