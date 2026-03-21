use bonsaidb::local::AsyncDatabase;
use serenity::{
    all::{Context, EventHandler, Interaction, Ready},
    async_trait,
};
use tracing::info;

pub struct RanksHandler {
    pub db: AsyncDatabase,
}

#[async_trait]
impl EventHandler for RanksHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {}

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
