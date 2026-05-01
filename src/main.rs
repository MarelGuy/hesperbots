use clap::{
    Parser,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};

use sqlx::postgres::PgPoolOptions;

use serenity::{Client, all::GatewayIntents};

use tracing::error;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::handler::Handler;

mod collections;
mod commands;
mod components;
mod functions;
mod handler;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Parser, Debug)]
#[command(
    name = "Hesper",
    styles = cli_styles(),
    version,
    about = "Discord bot to manage roles and channels on your server",
    arg_required_else_help = false
)]
struct Config {
    #[arg(long, env)]
    database_url: String,

    #[arg(long, env)]
    discord_token: String,

    #[arg(long, env, default_value = "info")]
    rust_log: String,
}

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let config = Config::parse();

    tracing_subscriber::registry()
        .with(config.rust_log.parse::<EnvFilter>()?)
        .with(fmt::layer().with_file(true).with_line_number(true))
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let mut client = Client::builder(config.discord_token, GatewayIntents::empty())
        .event_handler(Handler { db: pool })
        .await?;

    if let Err(why) = client.start().await {
        error!("Client error: {why}");
    }

    if let Err(e) = tokio::signal::ctrl_c().await {
        error!("Could not register ctrl+c handler: {}", e);
    }

    Ok(())
}
