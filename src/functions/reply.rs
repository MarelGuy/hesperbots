use std::time::Duration;

use serenity::builder::{
    CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, EditInteractionResponse, EditMessage,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::sleep;

use crate::BoxError;

pub enum MessageTarget<'a> {
    Interaction(&'a CommandInteraction),
    Channel(ChannelId),
}

pub async fn reply(
    ctx: &Context,
    target: MessageTarget<'_>,
    message_text: &str,
    timer: u64,
) -> Result<(), BoxError> {
    let author = CreateEmbedAuthor::new("Jagd")
        .icon_url("https://cdn.discordapp.com/avatars/293310031905161216/a_93961cab2f619c0d9b55b087e0422139.gif?size=512");

    let mut current_timer = timer;

    let create_embed = |time: u64| {
        CreateEmbed::new()
            .author(author.clone())
            .title(message_text)
            .footer(CreateEmbedFooter::new(format!("{time}...")))
    };

    let mut channel_msg: Option<Message> = None;

    match target {
        MessageTarget::Interaction(interaction) => {
            let response = CreateInteractionResponseMessage::new()
                .embed(create_embed(current_timer))
                .ephemeral(true);

            interaction
                .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                .await?;
        }
        MessageTarget::Channel(channel_id) => {
            let msg = channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new().embed(create_embed(current_timer)),
                )
                .await?;
            channel_msg = Some(msg);
        }
    }

    for _ in 1..timer {
        current_timer -= 1;
        sleep(Duration::from_secs(1)).await;

        match target {
            MessageTarget::Interaction(interaction) => {
                interaction
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().embed(create_embed(current_timer)),
                    )
                    .await?;
            }
            MessageTarget::Channel(_) => {
                if let Some(msg) = &mut channel_msg {
                    msg.edit(
                        &ctx.http,
                        EditMessage::new().embed(create_embed(current_timer)),
                    )
                    .await?;
                }
            }
        }
    }

    sleep(Duration::from_secs(1)).await;

    match target {
        MessageTarget::Interaction(interaction) => {
            interaction.delete_response(&ctx.http).await?;
        }
        MessageTarget::Channel(_) => {
            if let Some(msg) = channel_msg {
                msg.delete(&ctx.http).await?;
            }
        }
    }

    Ok(())
}
