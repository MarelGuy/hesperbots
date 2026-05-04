use serenity::all::{
    ButtonStyle, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateEmbedFooter, CreateMessage, Permissions,
};

use crate::{
    error::HesperError,
    functions::{MessageTarget, reply},
};

pub async fn send_verification_message(
    command: CommandInteraction,
    ctx: Context,
) -> Result<(), HesperError> {
    let channel_id = command.channel_id;

    let CommandDataOptionValue::String(message_content) = &command.data.options[0].value else {
        reply(
            &ctx,
            MessageTarget::CommandInteraction(&command),
            "Message could not be sent: content not set",
            10,
        )
        .await?;

        return Err("Message content not set".into());
    };

    let button = CreateButton::new("verbutton")
        .label(" ")
        .style(ButtonStyle::Secondary)
        .emoji('🆔');

    let row = CreateActionRow::Buttons(vec![button]);

    let embed = CreateEmbed::new()
        .image("https://cdn.discordapp.com/attachments/...")
        .footer(CreateEmbedFooter::new(message_content).icon_url("https://cdn.discordapp.com/..."));

    let builder = CreateMessage::new().add_embed(embed).components(vec![row]);

    if let Err(e) = channel_id.send_message(&ctx.http, builder).await {
        reply(
            &ctx,
            MessageTarget::CommandInteraction(&command),
            "Message could not be sent!",
            10,
        )
        .await?;

        return Err(format!("Verification message could not be sent: {e}").into());
    }

    reply(
        &ctx,
        MessageTarget::CommandInteraction(&command),
        "Sent!",
        10,
    )
    .await?;

    Ok(())
}

pub fn define_send_verification_message() -> CreateCommand {
    CreateCommand::new("send_verification_message")
        .description("Send verification message in the current channel")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "content",
                "Content to send as text message",
            )
            .required(true),
        )
}
