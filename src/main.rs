use std::env;

use bonsaidb::local::{
    AsyncDatabase,
    config::{Builder, StorageConfiguration},
};

use serenity::{
    Client,
    all::{EventHandler, GatewayIntents},
};

use tracing::error;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use handlers::{MainHandler, RanksHandler};

use crate::collections::HesperSchema;

mod collections;
mod functions;
mod handlers;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// # Errors
///
/// yes
pub async fn start_bot<T>(handler: T, token: String) -> Result<Client, BoxError>
where
    T: EventHandler + 'static,
{
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(handler)
        .await?;

    if let Err(why) = client.start().await {
        error!("Client error: {why}");
    }

    Ok(client)
}

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

    tokio::spawn(start_bot(
        RanksHandler { db: db.clone() },
        env::var("RANKS_DISCORD_TOKEN")?,
    ));
    tokio::spawn(start_bot(
        MainHandler { db: db.clone() },
        env::var("MAIN_DISCORD_TOKEN")?,
    ));

    if let Err(e) = tokio::signal::ctrl_c().await {
        error!("Could not register ctrl+c handler: {}", e);
    }

    Ok(())
}
