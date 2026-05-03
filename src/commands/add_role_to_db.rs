use std::str::FromStr;

use serenity::all::{CommandDataOptionValue, CommandInteraction, Context, GuildId};

use crate::{
    collections::{RolePurpose, Roles},
    error::HesperError,
    functions::{MessageTarget, reply},
    handler::Handler,
};

pub async fn add_role_to_db(
    handler: &Handler,
    command: CommandInteraction,
    ctx: Context,
    guild_id: GuildId,
) -> Result<(), HesperError> {
    let CommandDataOptionValue::Role(role_id) = &command.data.options[0].value else {
        return Err("Role not found".into());
    };

    let CommandDataOptionValue::String(purpose) = &command.data.options[1].value else {
        return Err("Error parsing purpose".into());
    };

    let discord_role = ctx.http.get_guild_role(guild_id, *role_id).await?.name;

    let role = Roles {
        guild_id: guild_id.to_string(),
        role_id: role_id.to_string(),
        role_name: discord_role.clone(),
        role_purpose: RolePurpose::from_str(purpose.as_str())? as i32,
    };

    role.insert(&handler.db).await?;

    reply(
        &ctx,
        MessageTarget::Interaction(&command),
        format!("Added {discord_role} as {purpose} to db").as_str(),
        10,
    )
    .await?;

    Ok(())
}
