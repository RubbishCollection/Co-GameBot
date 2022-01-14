pub mod dummy;

use serenity::builder::CreateActionRow;
use serenity::model::id::{ChannelId, UserId};
use serenity::model::interactions::message_component::MessageComponentInteraction;

pub trait Game: Send + Sync {
    //if user interaction is `Exit` returns `None`
    fn handle_interaction(&mut self, interaction: &MessageComponentInteraction) -> Option<String>;
    fn get_current_state(&self) -> String;
    fn is_participant(&self, user: &String) -> bool;

    fn action_rows(&self) -> Vec<CreateActionRow>;
}
