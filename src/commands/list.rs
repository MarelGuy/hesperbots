use serenity::all::{CommandDataOptionValue, CommandInteraction, Context};

use crate::{
    BoxError,
    functions::{MessageTarget, reply},
    handler::Handler,
};

pub async fn list(
    handler: &Handler,
    command: CommandInteraction,
    ctx: Context,
) -> Result<(), BoxError> {
    let purpose = if let Some(option) = command.data.options.first() {
        if let CommandDataOptionValue::String(val) = &option.value {
            val
        } else {
            unreachable!()
        }
    } else {
        return reply(
            &ctx,
            MessageTarget::Interaction(&command),
            "Metti un username nel comando coglione",
            3,
        )
        .await;
    };

    Ok(())
}
