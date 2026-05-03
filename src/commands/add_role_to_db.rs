use std::str::FromStr;

use serenity::all::{CommandDataOptionValue, CommandInteraction, Context, GuildId};

use crate::{
    collections::{RolePurpose, Roles},
    error::HesperError,
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

    let discord_role = ctx.http.get_guild_role(guild_id, *role_id).await?;

    let role = Roles {
        guild_id: guild_id.to_string(),
        role_id: role_id.to_string(),
        role_name: discord_role.name,
        role_purpose: RolePurpose::from_str(purpose.as_str())? as i32,
    };

    role.insert(&handler.db).await?;

    Ok(())
}
