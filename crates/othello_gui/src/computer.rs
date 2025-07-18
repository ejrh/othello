use std::sync::atomic::Ordering;
use bevy::app::{App, Plugin, Update};
use bevy::log::info;
use bevy::prelude::{Component, EventReader, EventWriter, Query, ResMut, Single, Text2d, With};
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;

use othello_ai::{MinimaxAI, RandomAI, AI};
use othello_game::{convert, DefaultGame, Game, Move};

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

#[derive(Component)]
pub struct Computer {
    pub ai: AIType,
    pub task: Option<Task<(AIType, Option<Move>)>>,
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
    mut players: Query<(&Player, &mut Computer)>,
    mut game_events: EventWriter<GameEvent>,
) {
    if current_game.over {
        return
    }

    for (player, mut computer) in players.iter_mut() {
        if player.colour != current_game.game.next_turn() {
            continue;
        }

        if let Some(ref mut task) = computer.task {
            /* If the task is completion, apply the move chosen by the AI */
            if let Some((ai_back, maybe_mov)) = block_on(future::poll_once(task)) {
                info!("AI task completed with: {maybe_mov:?}");
                if let Some(mov) = maybe_mov {
                    game_events.write(GameEvent::ClickSquare { row: mov.row, col: mov.col });
                }
                computer.ai = ai_back;
                computer.task = None;
            }
        } else {
            /* Spawn a task for the computer to choose a move */
            let task_pool = AsyncComputeTaskPool::get();

            let ai_copy = computer.ai.clone();
            let game_copy: DefaultGame = convert(&*current_game.game);
            computer.task = Some(task_pool.spawn(async move {
                let mov = ai_copy.choose_move(&game_copy);
                (ai_copy, mov)
            }));
            info!("Spawned task for AI")
        }
    }
}

fn update_ai_info(
    game_events: EventReader<GameEvent>,
    computers: Query<&Computer>,
    mut ai_text: Single<&mut Text2d, With<AIInfoLabel>>
) {
    if game_events.is_empty() { return }

    for computer in computers.iter() {
        let AIType::MinimaxAI(ai) = &computer.ai
        else { continue };
        let Some(info) = ai.info()
        else { continue };
        ai_text.0 = format!("AI Info:\n\
        Nodes Searched: {}\n", info.nodes_searched.load(Ordering::Relaxed));
    }
}
