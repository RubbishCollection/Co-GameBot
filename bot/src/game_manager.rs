use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use endorphin::policy::TTIPolicy;
use endorphin::HashMap;

use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::interactions::message_component::{
    InteractionMessage, MessageComponentInteraction,
};
use serenity::model::interactions::InteractionResponseType;

use crate::Error;
use crate::Game;

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
                self.games.insert((msg.channel_id, msg.id), game, tti);
                Ok(())
            }
            Err(err) => {
                println!("{:#?}", err);
                Err(Error::RegisterFailed)
            }
        }
    }

    pub async fn interaction(&mut self, ctx: Context, interaction: MessageComponentInteraction) {
        if let InteractionMessage::Regular(mut msg) = interaction.message.clone() {
            let result = if let Some(game) = self.games.get_mut(&(msg.channel_id, msg.id)) {
                //if pressed user is not a particiapant exit function.
                if !game.is_participant(&interaction.user.name) {
                    return;
                }

                game.handle_interaction(&interaction)
            } else {
                Some("Content is expired by Timeout".to_string())
            };

            match result {
                Some(result) => {
                    let mut embed = CreateEmbed::default();
                    embed.description(result);

                    interaction
                        .create_interaction_response(&ctx, |r| {
                            r.kind(InteractionResponseType::UpdateMessage);
                            r.interaction_response_data(|d| d.embeds(vec![embed]))
                        })
                        .await
                        .unwrap();
                }
                None => {
                    self.games.remove(&(msg.channel_id, msg.id));
                }
            }
        } else {
            unreachable!()
        }
    }
}
