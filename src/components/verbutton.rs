use serenity::all::{ComponentInteraction, Context, GuildId};

use crate::{
    BoxError,
    collections::{RolePurpose, Roles, Users},
    handler::Handler,
};

pub async fn verbutton(
    handler: &Handler,
    component: ComponentInteraction,
    ctx: Context,
    guild_id: GuildId,
) -> Result<(), BoxError> {
    let Some(verification_role) = Roles::get(
        &handler.db,
        RolePurpose::Verified as i32,
        &guild_id.to_string(),
    )
    .await?
    else {
        return Err("Verification role not added.".into());
    };

    let user_id = component.user.id;
    let user_id_str = user_id.to_string();

    let user = Users::get(&handler.db, &user_id_str).await?;

    if user.is_none() {
        let user = Users::new(user_id_str, guild_id.to_string());

        Users::insert(&handler.db, user).await?;
    }

    ctx.http
        .add_member_role(guild_id, user_id, verification_role.role_id.parse()?, None)
        .await?;

    Ok(())
}
