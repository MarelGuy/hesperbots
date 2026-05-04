use serenity::{
    all::{ChannelId, Command, Context, EventHandler, Interaction, Message, Ready, RoleId},
    async_trait,
};
use sqlx::PgPool;
use tracing::{error, info};

use crate::{
    collections::{ChannelPurpose, Channels, RolePurpose, Roles, Users},
    commands::{
        add_channel_to_db, add_role_to_db, define_add_channel_to_db, define_add_role_to_db,
        define_help, define_list, define_remove_channel_from_db, define_remove_role_from_db,
        define_send_verification_message, help, list, remove_channel_from_db, remove_role_from_db,
        send_verification_message,
    },
    components::verbutton,
    error::HesperError,
    functions::{MessageTarget, calculate_xp_for_level, reply},
};

pub struct Handler {
    pub db: PgPool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if let Err(e) = self.handle_message(ctx, new_message).await {
            error!("Error handling message: {}", e);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(e) = Box::pin(self.handle_interaction_create(ctx, interaction)).await {
            error!("Error handling interaction: {}", e);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        if let Err(why) = Command::set_global_commands(
            &ctx.http,
            vec![
                define_help(),
                define_list(),
                define_add_role_to_db(),
                define_add_channel_to_db(),
                define_remove_role_from_db(),
                define_remove_channel_from_db(),
                define_send_verification_message(),
            ],
        )
        .await
        {
            error!("Failed to register commands: {why}");
        } else {
            info!("Global commands registered successfully.");
        }
    }
}

impl Handler {
    async fn handle_message(&self, ctx: Context, new_message: Message) -> Result<(), HesperError> {
        let user_id = new_message.author.id.to_string();

        let guild_id = if let Some(guild_id) = new_message.guild_id {
            guild_id.to_string()
        } else {
            return Err("No guild id, what happened?".into());
        };

        let user_option: Option<Users> = Users::get(&self.db, &user_id).await?;

        if let Some(mut user) = user_option {
            user.xp += 1;

            if user.xp >= user.next_rank_xp {
                user.rank += 1;
                user.xp = 1;
                user.next_rank_xp = calculate_xp_for_level(user.rank);

                reply(
                    &ctx,
                    MessageTarget::Channel(new_message.channel_id),
                    format!("You leveled up! You are now level {}", user.rank).as_str(),
                    10,
                )
                .await?;

                let channel_doc: Option<Channels> =
                    Channels::get(&self.db, ChannelPurpose::RankChannel as i32, &guild_id).await?;

                if let Some(channel_doc) = channel_doc {
                    let ranks_channel_id = ChannelId::new(channel_doc.channel_id.parse::<u64>()?);

                    if let Err(e) = ranks_channel_id
                        .say(
                            &ctx.http,
                            format!(
                                "<@{}> leveled up! They are now level {}",
                                new_message.author.id, user.rank
                            ),
                        )
                        .await
                    {
                        tracing::error!("Failed to send rank up announcement: {}", e);
                    }
                }

                if let Some(role_purpose) = RolePurpose::from_repr(user.rank) {
                    let role_doc: Option<Roles> =
                        Roles::get(&self.db, role_purpose as i32, &guild_id).await?;

                    if let Some(role_doc) = role_doc {
                        let role_id = RoleId::new(role_doc.role_id.parse::<u64>()?);

                        if let Err(e) = ctx
                            .http
                            .add_member_role(
                                new_message.guild_id.unwrap(),
                                new_message.author.id,
                                role_id,
                                Some("User leveled up"),
                            )
                            .await
                        {
                            tracing::error!("Failed to assign role: {}", e);
                        }
                    }
                }
            }

            user.update(&self.db).await?;
        } else {
            Users::insert(&self.db, Users::new(user_id, guild_id)).await?;
        }

        Ok(())
    }

    async fn handle_interaction_create(
        &self,
        ctx: Context,
        interaction: Interaction,
    ) -> Result<(), HesperError> {
        let Some(guild_id) = interaction.guild_id() else {
            return Err("No guild id, what happened?".into());
        };

        let guild_id_str: String = guild_id.to_string();

        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "help" => help(command, ctx).await,
                "list" => list(self, command, ctx, guild_id_str).await?,
                "add_role_to_db" => add_role_to_db(self, command, ctx, guild_id).await?,
                "add_channel_to_db" => add_channel_to_db(self, command, ctx, guild_id_str).await?,
                "remove_role_from_db" => remove_role_from_db(self, command, ctx).await?,
                "remove_channel_from_db" => remove_channel_from_db(self, command, ctx).await?,
                "send_verification_message" => send_verification_message(command, ctx).await?,
                _ => unreachable!(),
            }
        } else if let Interaction::Component(component) = interaction {
            match component.data.custom_id.as_str() {
                "verbutton" => verbutton(self, component, ctx, guild_id).await?,
                _ => todo!(),
            }
        }

        Ok(())
    }
}
