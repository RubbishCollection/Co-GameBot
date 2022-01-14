use serenity::builder::{CreateActionRow, CreateButton};
use serenity::model::id::ChannelId;
use serenity::model::interactions::message_component::{ButtonStyle, MessageComponentInteraction};

use crate::Game;

use dummy_game::dummy::{Coord, DummyGame, Direction};

pub struct Dummy {
    game: DummyGame,
    action_rows: Vec<CreateActionRow>,
}

impl Dummy {
    pub fn new() -> Self {
        let mut action_rows = Vec::new();

        let mut first_row = CreateActionRow::default();
        first_row.add_button(Self::none_button("none1"));
        first_row.add_button(Self::button_with("up", "↑", ButtonStyle::Primary));
        first_row.add_button(Self::none_button("none2"));

        let mut second_row = CreateActionRow::default();
        second_row.add_button(Self::button_with("left", "←", ButtonStyle::Primary));
        second_row.add_button(Self::button_with("down", "↓", ButtonStyle::Primary));
        second_row.add_button(Self::button_with("right", "→", ButtonStyle::Primary));

        action_rows.push(first_row);
        action_rows.push(second_row);

        Self {
            game: DummyGame::init(8, 8, Coord { x: 0, y: 0 }),
            action_rows,
        }
    }
    fn none_button(id: &str) -> CreateButton {
        Self::button_with(id, " ", ButtonStyle::Secondary)
    }

    fn button_with(id: &str, label: &str, style: ButtonStyle) -> CreateButton {
        let mut button = CreateButton::default();
        button.custom_id(id);
        button.label(label);
        button.style(style);

        button
    }
}

impl Game for Dummy {
    fn handle_interaction(
        &mut self,
        interaction: &MessageComponentInteraction,
    ) -> Option<String> {
        let dir = match &interaction.data.custom_id[..] {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => Direction::Stop,
        };

        self.game.move_to(dir);

        Some(self.get_current_state())
    }

    fn get_current_state(&self) -> String {
        self.game.as_string()
    }

    fn is_participant(&self, _user: &String) -> bool {
        true
    }

    fn action_rows(&self) -> Vec<CreateActionRow> {
        self.action_rows.clone()
    }
}
