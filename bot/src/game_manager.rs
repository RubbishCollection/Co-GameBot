use std::time::Duration;

use endorphin::policy::TTIPolicy;
use endorphin::HashMap;

use serenity::client::Context;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::interactions::message_component::{
    InteractionMessage, MessageComponentInteraction,
};

use tracing::info;
use tracing::instrument;

use crate::Error;
use crate::Game;
use crate::UpdateMessage;

type BoxedGame = Box<dyn Game + Send + Sync>;

pub struct GameManager {
    games: HashMap<(ChannelId, MessageId), BoxedGame, TTIPolicy>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(TTIPolicy::new()),
        }
    }

    #[instrument(level = "info", skip(self, ctx, game))]
    pub async fn register(
        &mut self,
        ctx: Context,
        channel_id: ChannelId,
        game: BoxedGame,
        tti: Duration,
    ) -> Result<(), Error> {
        let content = game.get_current_state();
        let rows = game.action_rows();

        match channel_id
            .send_message(&ctx.http, |m| {
                m.add_embed(|e| e.description(content));
                m.components(|c| c.set_action_rows(rows))
            })
            .await
        {
            Ok(msg) => {
                info!(?msg.id);

                self.games.insert((msg.channel_id, msg.id), game, tti);
                Ok(())
            }
            Err(err) => {
                info!("Error: {}", err);

                Err(Error::RegisterFailed)
            }
        }
    }

    #[instrument(level = "debug", skip(self, ctx, interaction))]
    pub async fn handle_interaction(
        &mut self,
        ctx: Context,
        interaction: MessageComponentInteraction,
    ) {
        if let InteractionMessage::Regular(msg) = interaction.message.clone() {
            let result = if let Some(game) = self.games.get_mut(&(msg.channel_id, msg.id)) {
                //if pressed user is not a particiapant exit function.
                if !game.is_participant(&interaction.user.name) {
                    return;
                }

                game.handle_interaction(&interaction)
            } else {
                Some("Content is expired.".to_string())
            };

            match result {
                Some(result) => {
                    interaction.update_embed(ctx, &result).await;
                }
                None => {
                    info!(?msg.id, ?msg.channel_id, "Removed by explicit `quit` interaction");

                    interaction.update_embed(ctx, "Player quit the game").await;
                    self.games.remove(&(msg.channel_id, msg.id));
                }
            }
        } else {
            unreachable!()
        }
    }
}
