use serenity::all::{CommandDataOptionValue, CommandInteraction, Context};

use crate::{
    collections::{RolePurpose, Roles},
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

    let role_id = role_id.to_string();

    let Some(role) = Roles::get_by_id(&handler.db, &role_id).await? else {
        reply(
            &ctx,
            MessageTarget::Interaction(&command),
            "Role not found in the databse",
            10,
        )
        .await?;

        return Err("Called remove_role_from_db with 404".into());
    };

    let Some(role_purpose) = RolePurpose::from_repr(role.role_purpose) else {
        reply(
            &ctx,
            MessageTarget::Interaction(&command),
            "Internal error, check bot logs",
            10,
        )
        .await?;

        return Err("Internal error: Role purpose conversion went wrong".into());
    };

    Roles::remove(&handler.db, &role_id).await?;

    reply(
        &ctx,
        MessageTarget::Interaction(&command),
        &format!("Removed {} as {}", role.role_name, role_purpose),
        10,
    )
    .await?;

    Ok(())
}
