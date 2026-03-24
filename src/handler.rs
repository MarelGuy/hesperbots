use bonsaidb::{core::schema::SerializedCollection, local::AsyncDatabase};
use serenity::{
    all::{
        ChannelId, Command, CommandOptionType, Context, CreateCommand, CreateCommandOption,
        EventHandler, Interaction, Message, Ready, RoleId,
    },
    async_trait,
};
use tracing::{error, info};

use crate::{
    BoxError,
    collections::{ChannelPurpose, Channels, RolePurpose, Roles, Users},
    commands::{add_role_to_db, help, list},
    functions::{MessageTarget, calculate_xp_for_level, reply},
};

pub struct Handler {
    pub db: AsyncDatabase,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if let Err(e) = self.handle_message(ctx, new_message).await {
            error!("Error handling message: {}", e);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(e) = self.handle_interaction_create(ctx, interaction).await {
            error!("Error handling message: {}", e);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let help = CreateCommand::new("help")
            .description("Comando di aiuto per controllare i comandi disponibili");

        let list = CreateCommand::new("list")
            .description("Comando per controllare tutti i purpose associati e disponibili")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "purpose",
                    "Purpose da listare: RolePurpose, ChannelPurpose",
                )
                .add_string_choice("Role Purpose", "RolePurpose")
                .add_string_choice("Channel Purpose", "ChannelPurpose")
                .required(true),
            );

        let add_role_to_db = CreateCommand::new("add_role_to_db")
            .description("Cambia o associa un ruolo ad un Purpose");

        if let Err(why) =
            Command::set_global_commands(&ctx.http, vec![help, list, add_role_to_db]).await
        {
            error!("Failed to register commands: {why}");
        } else {
            info!("Global commands registered successfully.");
        }
    }
}

impl Handler {
    async fn handle_message(&self, ctx: Context, new_message: Message) -> Result<(), BoxError> {
        let user_id = new_message.author.id.to_string();

        let user_option = Users::get_async(&user_id, &self.db).await?;

        if let Some(mut user) = user_option {
            user.contents.xp += 1;

            if user.contents.xp >= user.contents.next_rank_xp {
                user.contents.rank += 1;

                user.contents.xp = 1;
                user.contents.next_rank_xp = calculate_xp_for_level(user.contents.rank + 1);

                reply(
                    &ctx,
                    MessageTarget::Channel(new_message.channel_id),
                    format!("You leveled up! You are now level {}", user.contents.rank).as_str(),
                    10,
                )
                .await?;

                if let Ok(Some(channel_doc)) =
                    Channels::get_async(&ChannelPurpose::RankChannel, &self.db).await
                {
                    let ranks_channel_id =
                        ChannelId::new(channel_doc.contents.channel_id.parse::<u64>()?);

                    if let Err(e) = ranks_channel_id
                        .say(
                            &ctx.http,
                            format!(
                                "<@{}> leveled up! They are now level {}",
                                new_message.author.id, user.contents.rank
                            ),
                        )
                        .await
                    {
                        tracing::error!("Failed to send rank up announcement: {}", e);
                    }
                }

                if let Some(role_purpose) = RolePurpose::from_repr(user.contents.rank)
                    && let Ok(Some(role_doc)) = Roles::get_async(&role_purpose, &self.db).await
                    && let Some(guild_id) = new_message.guild_id
                {
                    let role_id = RoleId::new(role_doc.contents.role_id.parse::<u64>()?);

                    if let Err(e) = ctx
                        .http
                        .add_member_role(
                            guild_id,
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

            user.update_async(&self.db).await?;
        } else {
            Users::push_async(
                Users {
                    userid: user_id.clone(),
                    rank: 0,
                    xp: 0,
                    next_rank_xp: calculate_xp_for_level(1),
                    colour: String::new(),
                    zod_sign: String::new(),
                },
                &self.db,
            )
            .await?;
        }

        Ok(())
    }

    async fn handle_interaction_create(
        &self,
        ctx: Context,
        interaction: Interaction,
    ) -> Result<(), BoxError> {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "help" => help(command, ctx).await,
                "list" => list(self, command, ctx).await?,
                "add_role_to_db" => add_role_to_db(self, command, ctx).await,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}
