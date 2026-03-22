use bonsaidb::{core::schema::SerializedCollection, local::AsyncDatabase};
use serenity::{
    all::{Context, EventHandler, Message, Ready},
    async_trait,
};
use tracing::{error, info};

use crate::{
    BoxError,
    collections::Users,
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

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    async fn handle_message(&self, ctx: Context, new_message: Message) -> Result<(), BoxError> {
        let user_id = new_message.author.id.to_string();

        let user_option = Users::get_async(&user_id, &self.db).await?;

        if let Some(mut user) = user_option {
            user.contents.xp += 1;

            if user.contents.xp == user.contents.next_rank_xp {
                reply(
                    &ctx,
                    MessageTarget::Channel(new_message.channel_id),
                    format!(
                        "You leveled up! You are now level {}",
                        user.contents.rank + 1
                    )
                    .as_str(),
                    10,
                )
                .await?;
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
}
