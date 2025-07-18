use std::sync::atomic::Ordering;

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{EventReader, EventWriter, Query, ResMut, Single, Text2d, With};

use othello_ai::{MinimaxAI, RandomAI, AI};
use othello_game::{Game, Move};

use crate::game::{CurrentGame, GameEvent, Player};
use crate::rendering::AIInfoLabel;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_ai)
            .add_systems(Update, update_ai_info);
    }
}

#[derive(Clone)]
pub enum AIType {
    RandomAI(RandomAI),
    MinimaxAI(MinimaxAI)
}

impl AIType {
    fn choose_move(&self, game: &dyn Game) -> Option<Move> {
        match self {
            AIType::RandomAI(ai) => ai.choose_move(game),
            AIType::MinimaxAI(ai) => ai.choose_move(game),
        }
    }
}

fn update_ai(
    current_game: ResMut<CurrentGame>,
    players: Query<&Player>,
    mut game_events: EventWriter<GameEvent>,
) {
    if current_game.over {
        return
    }

    for player in players.iter() {
        if player.colour != current_game.game.next_turn() {
            continue;
        }

        let Some(ref ai) = player.ai else { return };

        let Some(mov) = ai.choose_move(&*current_game.game)
        else { continue };

        game_events.write(GameEvent::ClickSquare { row: mov.row, col: mov.col });
    }
}

fn update_ai_info(
    game_events: EventReader<GameEvent>,
    players: Query<&Player>,
    mut ai_text: Single<&mut Text2d, With<AIInfoLabel>>
) {
    if game_events.is_empty() { return }

    for player in players.iter() {
        let Some(AIType::MinimaxAI(ai)) = &player.ai
        else { continue };
        let Some(info) = ai.info()
        else { continue };
        ai_text.0 = format!("AI Info:\n\
        Nodes Searched: {}\n", info.nodes_searched.load(Ordering::Relaxed));
    }
}
