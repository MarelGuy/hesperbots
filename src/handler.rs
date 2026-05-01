use serenity::{
    all::{
        ChannelId, Command, CommandOptionType, Context, CreateCommand, CreateCommandOption,
        EventHandler, Interaction, Message, Ready, RoleId,
    },
    async_trait,
};
use sqlx::PgPool;
use tracing::{error, info};

use crate::{
    BoxError,
    collections::{ChannelPurpose, Channels, RolePurpose, Roles, Users},
    commands::{add_channel_to_db, add_role_to_db, help, list},
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
        if let Err(e) = self.handle_interaction_create(ctx, interaction).await {
            error!("Error handling interaction: {}", e);
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

        let add_channel_to_db = CreateCommand::new("add_channel_to_db")
            .description("Cambia o associa un canale ad un Purpose");

        if let Err(why) = Command::set_global_commands(
            &ctx.http,
            vec![help, list, add_role_to_db, add_channel_to_db],
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
    async fn handle_message(&self, ctx: Context, new_message: Message) -> Result<(), BoxError> {
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
    ) -> Result<(), BoxError> {
        if let Interaction::Command(command) = interaction {
            let guild_id: String = if let Some(guild_id) = command.guild_id {
                guild_id
            } else {
                return Err("No guild id, what happened?".into());
            }
            .to_string();

            match command.data.name.as_str() {
                "help" => help(command, ctx).await,
                "list" => list(self, command, ctx, guild_id).await?,
                "add_role_to_db" => add_role_to_db(self, command, ctx, guild_id).await,
                "add_channel_to_db" => add_channel_to_db(self, command, ctx, guild_id).await,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}
