use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::app::{App, Plugin, Startup, Update};
use bevy::log::{error, info};
use bevy::prelude::{Commands, Component, Entity, Event, EventReader, Query, Real, Res, ResMut, Resource, Text2d, Time, With, Without};
use bevy::time::Stopwatch;

use othello_ai::MinimaxAI;
use othello_game::{Colour, DefaultGame, Game, Move, Pos};

use crate::computer::{AIType, Computer};
use crate::rendering::{ScoreLabel, TimeLabel};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentGame>()
            .add_event::<GameEvent>()
            .add_systems(Startup, setup_players)
            .add_systems(Update, handle_game_events)
            .add_systems(Update, update_time)
            .add_systems(Update, update_score);
    }
}

#[derive(Resource)]
pub struct CurrentGame {
    pub game: Box<dyn Game + Send + Sync>,
    pub over: bool,
}

impl Default for CurrentGame {
    fn default() -> Self {
        CurrentGame {
            game: Box::new(DefaultGame::new()),
            over: false,
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub colour: Colour,
    pub name: String,
    pub sender: Sender<String>,
    pub player_time: Stopwatch,
}

#[derive(Component)]
pub struct BoardSquare {
    pub row: Pos,
    pub col: Pos
}

#[derive(Component)]
pub struct PlacedDisc {
    pub row: Pos,
    pub col: Pos
}

#[derive(Component)]
pub struct Chat {
    pub receiver: Arc<Mutex<Receiver<String>>>,
    pub messages: Vec<String>,
}

#[derive(Event)]
pub enum GameEvent {
    NewGame,
    MakeMove { mov: Move },
    ClickSquare { row: Pos, col: Pos },
}

pub fn setup_players(
    mut commands: Commands,
) {
    let (sender, receiver) = channel();

    commands.spawn((
        Player {
            colour: Colour::Black,
            name: "Computer".to_string(),
            sender: sender.clone(),
            player_time: Stopwatch::new(),
        },
        Computer {
            ai: AIType::MinimaxAI(MinimaxAI::new(6)),
            task: None,
        }
    ));

    commands.spawn(Player {
        colour: Colour::White,
        name: "Human".to_string(),
        sender: sender.clone(),
        player_time: Stopwatch::new(),
    });

    let chat = Chat {
        receiver: Arc::new(Mutex::new(receiver)),
        messages: Vec::new()
    };
    commands.spawn(chat);
}

fn handle_game_events(
    mut click_events: EventReader<GameEvent>,
    mut current_game: ResMut<CurrentGame>,
    mut players: Query<&mut Player>,
    humans: Query<Entity, (With<Player>, Without<Computer>)>,
    mut commands: Commands,
) {
    for event in click_events.read() {
        match event {
            GameEvent::NewGame => {
                current_game.game = Box::new(DefaultGame::new());
                current_game.over = false;
                players.iter_mut().for_each(|mut p| p.player_time.reset());
            }
            GameEvent::MakeMove { mov } => {
                if current_game.over {
                    continue
                }

                for player in players.iter() {
                    if player.colour != current_game.game.next_turn() {
                        continue;
                    }

                    if !current_game.game.is_valid_move(*mov) {
                        return;
                    }
                    current_game.game.apply_in_place(*mov);

                    player.sender.send(format!("{} moved: {}", player.name, mov))
                        .unwrap_or_else(|e| error!("Failed to send message: {}", e));
                }
            }
            GameEvent::ClickSquare { row, col } => {
                info!("Clicked {row}, {col}");

                for id in humans.iter() {
                    let Ok(player) = players.get(id)
                    else { continue; };
                    if player.colour != current_game.game.next_turn() {
                        continue;
                    }
                    
                    let mov = Move { player: player.colour, row: *row, col: *col };
                    commands.send_event(GameEvent::MakeMove { mov });
                }
            }
        }
    }

    /* Check if other player now can't go */
    if !current_game.over {
        for player in players.iter() {
            if player.colour != current_game.game.next_turn() {
                continue;
            }

            if current_game.game.valid_moves(player.colour).is_empty() {
                player.sender.send(format!("{} can't go", player.name)).unwrap();
                current_game.over = true;
            }
        }
    }

    /* Update players' stopwatches */
    for mut player in players.iter_mut() {
        if current_game.over || player.colour != current_game.game.next_turn() {
            player.player_time.pause();
        } else {
            player.player_time.unpause();
        }
    }
}

fn update_time(
    mut labels: Query<(&TimeLabel, &mut Text2d)>,
    mut players: Query<&mut Player>,
    now: Res<Time<Real>>,
) {
    for (label, mut text) in labels.iter_mut() {
        let Some(mut player) = players.iter_mut()
            .find(|p| p.colour == label.0)
        else { continue };

        player.player_time.tick(now.delta());
        let total_time = player.player_time.elapsed();

        let ms = total_time.as_millis();
        let ms = ms - total_time.as_secs() as u128 * 1000;
        let secs = total_time.as_secs();
        let mins = secs / 60;
        let secs = secs - mins * 60;
        let time_str = format!("{mins}:{secs:02}.{ms:03}");
        text.0 = time_str;
    }
}

fn update_score(
    mut labels: Query<(&ScoreLabel, &mut Text2d)>,
    current_game: Res<CurrentGame>,
    players: Query<&Player>,
) {
    let scores = current_game.game.scores();

    for (label, mut text) in labels.iter_mut() {
        let Some(player) = players.iter()
            .find(|p| p.colour == label.0)
        else { continue };
        let name = &player.name;
        let score = match label.0 {
            Colour::Black => scores.0,
            Colour::White => scores.1,
        };
        text.0 = format!("{name}: {score}");
    }
}
