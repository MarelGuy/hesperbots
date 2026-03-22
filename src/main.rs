use std::env;

use bonsaidb::{
    core::schema::SerializedCollection,
    local::{
        AsyncDatabase,
        config::{Builder, StorageConfiguration},
    },
};

use serenity::{Client, all::GatewayIntents};

use tracing::error;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    collections::{HesperSchema, Users},
    handler::Handler,
};

mod collections;
mod functions;
mod handler;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    tracing_subscriber::registry()
        .with(
            env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".into())
                .parse::<EnvFilter>()?,
        )
        .with(fmt::layer().with_file(true).with_line_number(true))
        .init();

    let db =
        AsyncDatabase::open::<HesperSchema>(StorageConfiguration::new(env::var("BONSAIDB_PATH")?))
            .await?;

    let mut client = Client::builder(env::var("DISCORD_TOKEN")?, GatewayIntents::empty())
        .event_handler(Handler { db: db.clone() })
        .await?;

    if let Err(why) = client.start().await {
        error!("Client error: {why}");
    }

    if let Err(e) = tokio::signal::ctrl_c().await {
        error!("Could not register ctrl+c handler: {}", e);
    }

    Ok(())
}
