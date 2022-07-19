use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::InteractionResponseType;
use serenity::{
    builder::CreateEmbed, model::interactions::message_component::MessageComponentInteraction,
};

use tracing::instrument;

#[async_trait]
pub trait UpdateMessage {
    async fn update_embed(&self, ctx: Context, description: &str);
}

#[async_trait]
impl UpdateMessage for MessageComponentInteraction {
    #[instrument(level = "info", skip(self, ctx))]
    async fn update_embed(&self, ctx: Context, description: &str) {
        let mut embed = CreateEmbed::default();
        embed.description(description);

        self.create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::UpdateMessage);
            r.interaction_response_data(|d| d.embeds(vec![embed]))
        })
        .await
        .unwrap();
    }
}
