use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;

use bevy::color::palettes::css::{BLACK, GOLD, GRAY, GREEN, LIMEGREEN, WHITE};
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::render::batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport};
use bevy::render::camera::ScalingMode;
use bevy::render::view::NoFrustumCulling;
use bevy::sprite::Anchor;
use bevy::text::TextBounds;
use bevy::time::Stopwatch;

use othello_ai::{AI, MinimaxAI, RandomAI};
use othello_game::{Colour, DefaultGame, Game, Move, Pos};

fn main() {
    App::new()
        .insert_resource(GpuPreprocessingSupport { max_supported_mode: GpuPreprocessingMode::None })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_theme, setup_camera, setup_players, setup_board, setup_lefthand_info, setup_righthand_info).chain())
        .add_systems(Update, (update_pieces, update_score, update_time))
        .add_systems(Update, (collect_game_inputs, collect_board_inputs))
        .add_systems(Update, (update_current_square, handle_game_events))
        .add_systems(Update, (update_ai, update_chat, update_ai_info))
        .add_systems(Update, close_on_esc)
        .init_resource::<Theme>()
        .init_resource::<CurrentGame>()
        .init_resource::<CurrentSquare>()
        .add_event::<GameEvent>()
        .run();
}

#[derive(Resource)]
struct CurrentGame {
    game: Box<dyn Game + Send + Sync>,
    over: bool,
}

#[derive(Clone)]
enum AIType {
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

#[derive(Component)]
struct Player {
    colour: Colour,
    name: String,
    ai: Option<AIType>,
    sender: Sender<String>,
    player_time: Stopwatch,
}

impl Default for CurrentGame {
    fn default() -> Self {
        CurrentGame {
            game: Box::new(DefaultGame::new()),
            over: false,
        }
    }
}

#[derive(Default, Resource)]
struct Theme {
    font: Handle<Font>,
    green: Color,
    gold: Color,
    black: Color,
    white: Color,
    black_material: Handle<ColorMaterial>,
    white_material: Handle<ColorMaterial>,
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
struct TimeLabel(Colour);

#[derive(Component)]
struct Chat {
    receiver: Arc<Mutex<Receiver<String>>>,
    messages: Vec<String>,
}

#[derive(Event)]
enum GameEvent {
    NewGame,
    ClickSquare { row: Pos, col: Pos }
}

#[derive(Component)]
struct AIInfoLabel;

fn setup_camera(
    mut commands: Commands,
) {
    let camera = Camera2d;
    let mut proj = OrthographicProjection::default_2d();
    proj.scaling_mode = ScalingMode::FixedVertical { viewport_height: 1000.0 };
    commands.spawn((camera, Projection::Orthographic(proj)));
}

fn setup_theme(
    mut theme: ResMut<Theme>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    theme.font = asset_server.load("fonts/FiraMono-Medium.ttf");

    theme.green = LIMEGREEN.into();
    theme.gold = GOLD.into();
    theme.black = BLACK.into();
    theme.white = WHITE.into();

    theme.black_material = materials.add(ColorMaterial::from(theme.black));
    theme.white_material = materials.add(ColorMaterial::from(theme.white));
}

fn setup_players(
    mut commands: Commands,
) {
    let (sender, receiver) = channel();

    commands.spawn(Player {
        colour: Colour::Black,
        name: "Computer".to_string(),
        ai: Some(AIType::MinimaxAI(MinimaxAI::new(6))),
        sender: sender.clone(),
        player_time: Stopwatch::new(),
    });

    commands.spawn(Player {
        colour: Colour::White,
        name: "Human".to_string(),
        ai: None,
        sender: sender.clone(),
        player_time: Stopwatch::new(),
    });

    let chat = Chat {
        receiver: Arc::new(Mutex::new(receiver)),
        messages: Vec::new()
    };
    commands.spawn(chat);
}

fn setup_board(
    mut commands: Commands,
    theme: ResMut<Theme>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    const TILE_SIZE: f32 = 80.0;
    const TILE_GAP: f32 = 2.0;
    const TILE_SPACING: f32 = TILE_SIZE + TILE_GAP;

    const DISC_RADIUS: f32 = 30.0;

    let disc_mesh = meshes.add(Circle::new(DISC_RADIUS));

    let x_offset = -(TILE_SPACING * 8.0 - TILE_GAP)/2.0 + TILE_SIZE/2.0;
    let y_offset = -(TILE_SPACING * 8.0 - TILE_GAP)/2.0 + TILE_SIZE/2.0;

    for row in 0..8 {
        for col in 0..8 {

            let xp = col as f32 * TILE_SPACING + x_offset;
            let yp = row as f32 * TILE_SPACING + y_offset;

            commands.spawn((
                BoardSquare { row, col },
                Sprite::from_color(LIMEGREEN, Vec2::new(TILE_SIZE, TILE_SIZE)),
                Transform::from_translation(Vec3::new(xp, yp, 0.)),
            ));

            commands.spawn((
                PlacedDisc { row, col },
                Mesh2d(disc_mesh.clone()),
                MeshMaterial2d(theme.black_material.clone()),
                Transform::from_translation(Vec3::new(xp, yp, 1.)),
            ));
        }
    }
}

fn setup_lefthand_info(
    mut commands: Commands,
    theme: Res<Theme>,
) {
    commands.spawn((
        Text2d::new("OTHELLO"),
        TextFont::from_font(theme.font.clone()).with_font_size(60.0),
        TextColor::from(GOLD),
        Transform::from_xyz(-500.0, 20.0, 0.0),
    ));

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let version_str = format!("Ver {VERSION}");

    commands.spawn((
        Text2d::new(version_str),
        TextFont::from_font(theme.font.clone()).with_font_size(20.0),
        TextColor::from(GOLD),
        Transform::from_xyz(-500.0, -20.0, 0.0),
    ));

    commands.spawn((
        ScoreLabel(Colour::Black),
        Text2d::new("black"),
        TextFont::from_font(theme.font.clone()).with_font_size(40.0),
        TextColor::from(BLACK),
        Transform::from_xyz(-500.0, 100.0, 0.0),
    ));

    commands.spawn((
        ScoreLabel(Colour::White),
        Text2d::new("white"),
        TextFont::from_font(theme.font.clone()).with_font_size(40.0),
        TextColor::from(WHITE),
        Transform::from_xyz(-500.0, -100.0, 0.0),
    ));

    commands.spawn((
        TimeLabel(Colour::Black),
        Text2d::new("black time"),
        TextFont::from_font(theme.font.clone()).with_font_size(30.0),
        TextColor::from(BLACK),
        Transform::from_xyz(-500.0, 150.0, 0.0),
    ));

    commands.spawn((
        TimeLabel(Colour::White),
        Text2d::new("white time"),
        TextFont::from_font(theme.font.clone()).with_font_size(30.0),
        TextColor::from(WHITE),
        Transform::from_xyz(-500.0, -150.0, 0.0),
    ));
}

fn setup_righthand_info(
    mut commands: Commands,
    theme: Res<Theme>,
    chat_id: Single<Entity, With<Chat>>,
) {
    const CHAT_WIDTH: f32 = 400.0;
    const CHAT_HEIGHT: f32 = 400.0;
    const CHAT_TOP: f32 = 0.0;
    const CHAT_LEFT: f32 = 400.0;

    commands.entity(*chat_id).insert((
        Text2d::default(),
        TextFont::from_font(theme.font.clone()).with_font_size(30.0),
        TextColor::from(WHITE),
        Anchor::TopLeft,
        TextBounds::new(CHAT_WIDTH, CHAT_HEIGHT),
        Transform::from_xyz(CHAT_LEFT, CHAT_TOP, 0.0),
        NoFrustumCulling,
    )).with_children(|parent| {
        parent.spawn((
            TextSpan::new("chat1"),
            TextFont::from_font(theme.font.clone()).with_font_size(30.0),
            TextColor::from(GRAY)
        ));
        parent.spawn((
            TextSpan::new("chat2"),
            TextFont::from_font(theme.font.clone()).with_font_size(30.0),
            TextColor::from(WHITE)
        ));
    });

    commands.spawn((
        Sprite::from_color(theme.black, Vec2::new(CHAT_WIDTH, CHAT_HEIGHT)),
        Transform::from_translation(Vec3::new(CHAT_LEFT + CHAT_WIDTH/2.0, CHAT_TOP - CHAT_HEIGHT/2.0, -1.)),
    ));

    const AI_INFO_TOP: f32 = 400.0;

    commands.spawn((
        Sprite::from_color(GREEN, Vec2::new(CHAT_WIDTH, CHAT_HEIGHT)),
        Transform::from_xyz(CHAT_LEFT + CHAT_WIDTH/2.0, AI_INFO_TOP - CHAT_HEIGHT/2.0, 0.0),
    )).with_child((
        AIInfoLabel,
        Text2d::new("ai"),
        TextFont::from_font(theme.font.clone()).with_font_size(30.0),
        TextColor::from(WHITE),
        Anchor::TopLeft,
        TextBounds::new(CHAT_WIDTH, CHAT_HEIGHT),
        Transform::from_xyz(-CHAT_WIDTH/2.0, CHAT_HEIGHT/2.0, 1.0),
        NoFrustumCulling,
    ));
}

fn update_pieces(
    theme: Res<Theme>,
    mut discs: Query<(&PlacedDisc, &mut Visibility, &mut MeshMaterial2d<ColorMaterial>)>,
    current_game: Res<CurrentGame>,
) {
    for (disc, mut vis, mut material) in discs.iter_mut() {
        match current_game.game.get_piece(disc.row, disc.col) {
            Some(Colour::Black) => {
                material.0 = theme.black_material.clone();
                *vis = Visibility::Inherited;
            },
            Some(Colour::White) => {
                material.0 = theme.white_material.clone();
                *vis = Visibility::Inherited;
            },
            None => {
                *vis = Visibility::Hidden;
            }
        }
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

fn collect_game_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_events: EventWriter<GameEvent>
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        info!("New othello_game");
        game_events.write(GameEvent::NewGame);
    }
}

fn collect_board_inputs(
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut touch_events: EventReader<TouchInput>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    squares: Query<(&BoardSquare, &Transform)>,
    mut game_events: EventWriter<GameEvent>
) {
    let (camera, camera_transform) = *camera;

    let mut point = touch_events.read()
        .filter(|e| e.phase == TouchPhase::Ended)
        .flat_map(|e| camera.viewport_to_world_2d(camera_transform, e.position))
        .next();

    if point.is_none() && mouse_input.just_pressed(MouseButton::Left) {
        point = window.cursor_position().iter()
            .flat_map(|pos| camera.viewport_to_world_2d(camera_transform, *pos))
            .next();
    }

    let Some(point) = point else { return };

    for (square, transform) in squares.iter() {
        let centre = transform.translation.truncate();
        let rect = Rect::from_center_half_size(centre, Vec2::new(40.0, 40.0));
        if rect.contains(point) {
            game_events.write(GameEvent::ClickSquare { row: square.row, col: square.col });
        }
    }
}

fn update_current_square(
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    theme: Res<Theme>,
    mut squares: Query<(&BoardSquare, &Transform, &mut Sprite)>,
    mut current_square: ResMut<CurrentSquare>,
) {
    let (camera, camera_transform) = *camera;

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    current_square.row = -1;
    current_square.col = -1;
    for (square, transform, mut sprite) in squares.iter_mut() {
        let centre = transform.translation.truncate();
        let rect = Rect::from_center_half_size(centre, Vec2::new(40.0, 40.0));
        if rect.contains(point) {
            sprite.color = theme.gold;
            current_square.row = square.row;
            current_square.col = square.col;
        } else {
            sprite.color = theme.green;
        }
    }
}

fn handle_game_events(
    mut click_events: EventReader<GameEvent>,
    mut current_game: ResMut<CurrentGame>,
    mut players: Query<&mut Player>,
) {
    for event in click_events.read() {
        match event {
            GameEvent::ClickSquare { row, col } => {
                info!("clicked {row}, {col}");
                if current_game.over {
                    continue
                }

                for player in players.iter() {
                    if player.colour != current_game.game.next_turn() {
                        continue;
                    }

                    let mov = Move {
                        player: player.colour,
                        row: *row,
                        col: *col,
                    };
                    if !current_game.game.is_valid_move(mov) {
                        return;
                    }
                    current_game.game.apply_in_place(mov);

                    player.sender.send(format!("{} moved: {}", player.name, mov))
                        .unwrap_or_else(|e| error!("Failed to send message: {}", e));
                }
            },
            GameEvent::NewGame => {
                current_game.game = Box::new(DefaultGame::new());
                current_game.over = false;
                players.iter_mut().for_each(|mut p| p.player_time.reset());
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

fn update_chat(
    mut chat: Single<(Entity, &mut Chat)>,
    theme: Res<Theme>,
    mut commands: Commands,
) {
    let (chat_id, chat) = &mut *chat;

    let mut new_msgs = Vec::new();
    let Ok(receiver_guard) = chat.receiver.lock() else { return };
    for msg in receiver_guard.try_iter() {
        new_msgs.push(msg);
    }
    drop(receiver_guard);

    if new_msgs.is_empty() { return }

    chat.messages.append(&mut new_msgs);

    while chat.messages.len() > 10 {
        chat.messages.remove(0);
    }

    /* Rerender chat text */
    commands.entity(*chat_id).despawn_related::<Children>();

    for msg in &chat.messages {
        let mut msg = msg.clone();
        msg.push('\n');
        commands.entity(*chat_id).with_child((
            TextSpan::new(msg),
            TextFont::from_font(theme.font.clone()).with_font_size(30.0),
            TextColor::from(GRAY)
        ));
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

pub fn close_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
