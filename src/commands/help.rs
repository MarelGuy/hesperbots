use serenity::all::{CommandInteraction, Context};
use tracing::error;

use crate::functions::reply;

pub async fn help(command: CommandInteraction, ctx: Context) {
    if let Err(why) = reply(
        &ctx,
        crate::functions::MessageTarget::Interaction(&command),
        "/help: Questo comando.\n/aggiungi_ruolo: Cambia o aggiunge un ruolo associato a RankPurpose\n/list <RankPurpose o ChannelPurpose>: Ritorna una lista di tutti i Purpose associabili (e associati) ad un rank o canale",
        10,
    )
    .await
    {
        error!("Cannot respond to slash command: {why}");
    }
}
