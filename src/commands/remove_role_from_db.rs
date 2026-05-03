use serenity::all::{CommandDataOptionValue, CommandInteraction};

use crate::{collections::Roles, error::HesperError, handler::Handler};

pub async fn remove_role_from_db(
    handler: &Handler,
    command: CommandInteraction,
) -> Result<(), HesperError> {
    let CommandDataOptionValue::Role(role_id) = &command.data.options[0].value else {
        return Err("Role not found".into());
    };

    Roles::remove(&handler.db, &role_id.to_string()).await?;

    Ok(())
}
