use std::time::Duration;

use serenity::builder::{
    CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, EditInteractionResponse, EditMessage,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::sleep;

use crate::error::HesperError;

pub enum MessageTarget<'a> {
    CommandInteraction(&'a CommandInteraction),
    ComponentInteraction(&'a ComponentInteraction),
    Channel(ChannelId),
}

impl MessageTarget<'_> {
    async fn send_initial(
        &self,
        ctx: &Context,
        embed: CreateEmbed,
    ) -> Result<Option<Message>, HesperError> {
        match self {
            Self::CommandInteraction(i) => {
                let response = CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .ephemeral(true);
                i.create_response(&ctx.http, CreateInteractionResponse::Message(response))
                    .await?;

                Ok(None)
            }
            Self::ComponentInteraction(i) => {
                let response = CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .ephemeral(true);
                i.create_response(&ctx.http, CreateInteractionResponse::Message(response))
                    .await?;

                Ok(None)
            }
            Self::Channel(c) => {
                let msg = c
                    .send_message(&ctx.http, CreateMessage::new().embed(embed))
                    .await?;

                Ok(Some(msg))
            }
        }
    }

    async fn edit_current(
        &self,
        ctx: &Context,
        embed: CreateEmbed,
        channel_msg: &mut Option<Message>,
    ) -> Result<(), HesperError> {
        match self {
            Self::CommandInteraction(i) => {
                i.edit_response(&ctx.http, EditInteractionResponse::new().embed(embed))
                    .await?;
            }
            Self::ComponentInteraction(i) => {
                i.edit_response(&ctx.http, EditInteractionResponse::new().embed(embed))
                    .await?;
            }
            Self::Channel(_) => {
                if let Some(msg) = channel_msg {
                    msg.edit(&ctx.http, EditMessage::new().embed(embed)).await?;
                }
            }
        }
        Ok(())
    }

    async fn delete_final(
        &self,
        ctx: &Context,
        channel_msg: Option<Message>,
    ) -> Result<(), HesperError> {
        match self {
            Self::CommandInteraction(i) => {
                i.delete_response(&ctx.http).await?;
            }
            Self::ComponentInteraction(i) => {
                i.delete_response(&ctx.http).await?;
            }
            Self::Channel(_) => {
                if let Some(msg) = channel_msg {
                    msg.delete(&ctx.http).await?;
                }
            }
        }
        Ok(())
    }
}

pub async fn reply(
    ctx: &Context,
    target: MessageTarget<'_>,
    message_text: &str,
    timer: u64,
) -> Result<(), HesperError> {
    let author = CreateEmbedAuthor::new("Jagd");
    let mut current_timer = timer;

    let create_embed = |time: u64| {
        CreateEmbed::new()
            .author(author.clone())
            .title(message_text)
            .footer(CreateEmbedFooter::new(format!("{time}...")))
    };

    let mut channel_msg = target
        .send_initial(ctx, create_embed(current_timer))
        .await?;

    for _ in 1..timer {
        current_timer -= 1;
        sleep(Duration::from_secs(1)).await;

        target
            .edit_current(ctx, create_embed(current_timer), &mut channel_msg)
            .await?;
    }

    target.delete_final(ctx, channel_msg).await?;

    Ok(())
}
