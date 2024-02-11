use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::text::Text2dBounds;
use bevy::window::close_on_esc;

use othello::ai::{AI, MinimaxAI, RandomAI};
use othello::game::{Board, Colour, DefaultGame, Game, Move, Pos};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_pieces, update_score))
        .add_systems(Update, (collect_events, update_current_square, click_square))
        .add_systems(Update, (update_ai, update_chat))
        .add_systems(Update, close_on_esc)
        .init_resource::<Colours>()
        .init_resource::<CurrentGame>()
        .init_resource::<CurrentSquare>()
        .add_event::<GameEvent>()
        .run();
}

#[derive(Resource)]
struct CurrentGame {
    game: DefaultGame,
    over: bool
}

#[derive(Clone)]
enum AIType {
    RandomAI(RandomAI),
    MinimaxAI(MinimaxAI)
}

impl AI for AIType {
    fn choose_move<B: Board>(&self, game: &Game<B>) -> Option<Move> {
        match self {
            AIType::RandomAI(ai) => ai.choose_move(game),
            AIType::MinimaxAI(ai) => ai.choose_move(game),
        }
    }
}

#[derive(Component)]
struct Player {
    colour: Colour,
    name: String,
    ai: Option<AIType>,
    sender: Sender<String>
}

impl Default for CurrentGame {
    fn default() -> Self {
        CurrentGame {
            game: DefaultGame::new(),
            over: false
        }
    }
}

#[derive(Default, Resource)]
struct Colours {
    green: Handle<ColorMaterial>,
    gold: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,
    white: Handle<ColorMaterial>,
}

#[derive(Component)]
struct BoardSquare {
    row: Pos,
    col: Pos
}

#[derive(Component)]
struct PlacedDisc {
    row: Pos,
    col: Pos
}

#[derive(Default, Resource)]
struct CurrentSquare {
    row: Pos,
    col: Pos
}

#[derive(Component)]
struct ScoreLabel(Colour);

#[derive(Component)]
struct Chat {
    receiver: Arc<Mutex<Receiver<String>>>,
    messages: Vec<String>,
}

#[derive(Event)]
enum GameEvent {
    ClickSquare { row: Pos, col: Pos }
}

fn setup(
    mut commands: Commands,
    mut colours: ResMut<Colours>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>)
{
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let (sender, receiver) = channel();

    commands.spawn(Player {
        colour: Colour::Black,
        name: "Computer".to_string(),
        ai: Some(AIType::MinimaxAI(MinimaxAI { max_depth: 3 })),
        sender: sender.clone()
    });

    commands.spawn(Player {
        colour: Colour::White,
        name: "Human".to_string(),
        ai: None,
        sender: sender.clone()
    });

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(1000.0);
    commands.spawn(camera);

    const TILE_SIZE: f32 = 80.0;
    const TILE_GAP: f32 = 2.0;
    const TILE_SPACING: f32 = TILE_SIZE + TILE_GAP;

    const DISC_RADIUS: f32 = 30.0;

    let x_offset = -(TILE_SPACING * 8.0 - TILE_GAP)/2.0 + TILE_SIZE/2.0;
    let y_offset = -(TILE_SPACING * 8.0 - TILE_GAP)/2.0 + TILE_SIZE/2.0;

    colours.green = materials.add(ColorMaterial::from(Color::LIME_GREEN));
    colours.gold = materials.add(ColorMaterial::from(Color::GOLD));
    colours.black = materials.add(ColorMaterial::from(Color::BLACK));
    colours.white = materials.add(ColorMaterial::from(Color::WHITE));

    for row in 0..8 {
        for col in 0..8 {

            let xp = col as f32 * TILE_SPACING + x_offset;
            let yp = row as f32 * TILE_SPACING + y_offset;

            commands.spawn(BoardSquare {
                row,
                col,
            }).insert(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(TILE_SIZE, TILE_SIZE)).into())
                    .into(),
                material: colours.green.clone(),
                transform: Transform::from_translation(Vec3::new(xp, yp, 0.)),
                ..default()
            });

            commands.spawn(PlacedDisc {
                row,
                col,
            }).insert(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(DISC_RADIUS).into())
                    .into(),
                material: colours.black.clone(),
                transform: Transform::from_translation(Vec3::new(xp, yp, 1.)),
                ..default()
            });

        }
    }

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::GOLD,
    };

    commands.spawn(Text2dBundle {
        text: Text::from_section("OTHELLO", text_style)
            .with_alignment(TextAlignment::Left),
        transform: Transform::from_xyz(-500.0, 0.0, 0.0),
        ..default()
    });

    let score_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::BLACK,
    };

    commands.spawn(ScoreLabel(Colour::Black))
        .insert(Text2dBundle {
            text: Text::from_section("black", score_text_style),
            transform: Transform::from_xyz(-500.0, 50.0, 0.0),
            ..default()
        });

    let score_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::WHITE,
    };

    commands.spawn(ScoreLabel(Colour::White))
        .insert(Text2dBundle {
            text: Text::from_section("white", score_text_style),
            transform: Transform::from_xyz(-500.0, -50.0, 0.0),
            ..default()
        });

    const CHAT_WIDTH: f32 = 400.0;
    const CHAT_HEIGHT: f32 = 400.0;
    const CHAT_TOP: f32 = 0.0;
    const CHAT_LEFT: f32 = 400.0;

    let chat_text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::GRAY,
    };

    let chat_text_style2 = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::BLACK,
    };

    let chat = Chat {
        receiver: Arc::new(Mutex::new(receiver)),
        messages: Vec::new()
    };
    commands.spawn(chat)
        .insert(Text2dBundle {
            text: Text::from_sections(vec![
                TextSection::new("chat1", chat_text_style),
                TextSection::new("chat2", chat_text_style2),
            ]),
            text_anchor: Anchor::TopLeft,
            text_2d_bounds: Text2dBounds { size: Vec2::new(CHAT_WIDTH, CHAT_HEIGHT) },
            transform: Transform::from_xyz(CHAT_LEFT, CHAT_TOP, 0.0),
            ..default()
        });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(CHAT_WIDTH, CHAT_HEIGHT)).into())
            .into(),
        material: colours.black.clone(),
        transform: Transform::from_translation(Vec3::new(CHAT_LEFT + CHAT_WIDTH/2.0, CHAT_TOP - CHAT_HEIGHT/2.0, -1.)),
        ..default()
    });
}

fn update_pieces(
    colours: Res<Colours>,
    mut discs: Query<(&PlacedDisc, &mut Visibility, &mut Handle<ColorMaterial>)>,
    current_game: Res<CurrentGame>,
) {
    for (disc, mut vis, mut material) in discs.iter_mut() {
        match current_game.game.board.get(disc.row, disc.col) {
            Some(Colour::Black) => {
                *material = colours.black.clone();
                *vis = Visibility::Inherited;
            },
            Some(Colour::White) => {
                *material = colours.white.clone();
                *vis = Visibility::Inherited;
            },
            None => {
                *vis = Visibility::Hidden;
            }
        }
    }
}

fn update_score(
    mut labels: Query<(&ScoreLabel, &mut Text)>,
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
        text.sections[0].value = format!("{name}: {score}");
    }
}

fn collect_events(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut touch_events: EventReader<TouchInput>,
    mouse_input: Res<Input<MouseButton>>,
    mut squares: Query<(&BoardSquare, &Transform)>,
    mut game_events: EventWriter<GameEvent>
) {
    let (camera, camera_transform) = camera_query.single();

    let mut point = touch_events.read()
        .filter(|e| e.phase == TouchPhase::Ended)
        .flat_map(|e| camera.viewport_to_world_2d(camera_transform, e.position))
        .next();

    if point.is_none() && mouse_input.just_pressed(MouseButton::Left) {
        point = windows.single().cursor_position().iter()
            .flat_map(|pos| camera.viewport_to_world_2d(camera_transform, *pos))
            .next();
    }

    let Some(point) = point else { return };

    for (square, transform) in squares.iter() {
        let centre = transform.translation.truncate();
        let rect = Rect::from_center_half_size(centre, Vec2::new(40.0, 40.0));
        if rect.contains(point) {
            game_events.send(GameEvent::ClickSquare { row: square.row, col: square.col })
        }
    }
}

fn update_current_square(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    colours: Res<Colours>,
    mut squares: Query<(&BoardSquare, &Transform, &mut Handle<ColorMaterial>)>,
    mut current_square: ResMut<CurrentSquare>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    current_square.row = -1;
    current_square.col = -1;
    for (square, transform, mut material) in squares.iter_mut() {
        let centre = transform.translation.truncate();
        let rect = Rect::from_center_half_size(centre, Vec2::new(40.0, 40.0));
        if rect.contains(point) {
            *material = colours.gold.clone();
            current_square.row = square.row;
            current_square.col = square.col;
        } else {
            *material = colours.green.clone();
        }
    }
}

fn click_square(
    mut click_events: EventReader<GameEvent>,
    mut current_game: ResMut<CurrentGame>,
    players: Query<&Player>,
) {
    if current_game.over {
        return
    }

    for event in click_events.read() {
        let GameEvent::ClickSquare { row, col } = event
        else { continue };

        for player in players.iter() {
            if player.colour != current_game.game.next_turn {
                continue;
            }

            let mov = Move {
                player: player.colour,
                row: *row,
                col: *col,
            };
            if !current_game.game.board.is_valid_move(mov) {
                return;
            }
            let new_game = current_game.game.apply(mov);
            current_game.game = new_game;

            player.sender.send(format!("{} moved: {}", player.name, mov)).unwrap();
        }
    }

    /* Check if other player now can't go */
    for player in players.iter() {
        if player.colour != current_game.game.next_turn {
            continue;
        }

        if current_game.game.valid_moves(player.colour).is_empty() {
            player.sender.send(format!("{} can't go", player.name)).unwrap();
            current_game.over = true;
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
        if player.colour != current_game.game.next_turn {
            continue;
        }

        let Some(ref ai) = player.ai else { return };

        let Some(mov) = ai.choose_move(&current_game.game)
        else { continue };

        game_events.send(GameEvent::ClickSquare { row: mov.row, col: mov.col });
    }
}

fn update_chat(
    mut query: Query<(&mut Chat, &mut Text)>,
) {
    let (mut chat, mut text) = query.single_mut();
    let mut new_msgs = Vec::new();
    let Ok(receiver_guard) = chat.receiver.lock() else { return };
    for msg in receiver_guard.try_iter() {
        new_msgs.push(msg);
    }
    drop(receiver_guard);

    if new_msgs.is_empty() { return }

    chat.messages.append(&mut new_msgs);

    let current_style = text.sections[0].style.clone();

    while chat.messages.len() > 10 {
        chat.messages.remove(0);
    }
    let mut sections = Vec::new();
    for msg in &chat.messages {
        let mut msg = msg.clone();
        msg.push('\n');
        sections.push(TextSection::new(msg, current_style.clone()));
    }

    text.sections = sections;
}
