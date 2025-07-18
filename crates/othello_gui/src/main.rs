use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::sprite::Anchor;
use bevy::text::TextBounds;

use othello_game::Colour;

use crate::computer::AIPlugin;
use crate::game::{setup_players, BoardSquare, Chat, CurrentGame, GameEvent, GamePlugin, PlacedDisc};
use crate::rendering::{AIInfoLabel, CurrentSquare, RenderingPlugin, ScoreLabel, Theme, TimeLabel, setup_theme};

mod computer;
mod game;
mod utils;
mod rendering;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_board, setup_lefthand_info, setup_righthand_info).after(setup_theme).after(setup_players))
        .add_systems(Update, update_pieces)
        .add_systems(Update, (collect_game_inputs, collect_board_inputs))
        .add_systems(Update, update_current_square)
        .add_systems(Update, update_chat)
        .add_plugins(GamePlugin)
        .add_plugins(AIPlugin)
        .add_plugins(RenderingPlugin)
        .add_systems(Update, utils::close_on_esc)
        .run();
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
                Sprite::from_color(theme.green, Vec2::new(TILE_SIZE, TILE_SIZE)),
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
        TextColor::from(theme.gold),
        Transform::from_xyz(-500.0, 20.0, 0.0),
    ));

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let version_str = format!("Ver {VERSION}");

    commands.spawn((
        Text2d::new(version_str),
        TextFont::from_font(theme.font.clone()).with_font_size(20.0),
        TextColor::from(theme.gold),
        Transform::from_xyz(-500.0, -20.0, 0.0),
    ));

    commands.spawn((
        ScoreLabel(Colour::Black),
        Text2d::new("black"),
        TextFont::from_font(theme.font.clone()).with_font_size(40.0),
        TextColor::from(theme.black),
        Transform::from_xyz(-500.0, 100.0, 0.0),
    ));

    commands.spawn((
        ScoreLabel(Colour::White),
        Text2d::new("white"),
        TextFont::from_font(theme.font.clone()).with_font_size(40.0),
        TextColor::from(theme.white),
        Transform::from_xyz(-500.0, -100.0, 0.0),
    ));

    commands.spawn((
        TimeLabel(Colour::Black),
        Text2d::new("black time"),
        TextFont::from_font(theme.font.clone()).with_font_size(30.0),
        TextColor::from(theme.black),
        Transform::from_xyz(-500.0, 150.0, 0.0),
    ));

    commands.spawn((
        TimeLabel(Colour::White),
        Text2d::new("white time"),
        TextFont::from_font(theme.font.clone()).with_font_size(30.0),
        TextColor::from(theme.white),
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
        TextColor::from(theme.white),
        Anchor::TopLeft,
        TextBounds::new(CHAT_WIDTH, CHAT_HEIGHT),
        Transform::from_xyz(CHAT_LEFT, CHAT_TOP, 0.0),
        NoFrustumCulling,
    )).with_children(|parent| {
        parent.spawn((
            TextSpan::new("chat1"),
            TextFont::from_font(theme.font.clone()).with_font_size(30.0),
            TextColor::from(theme.grey)
        ));
        parent.spawn((
            TextSpan::new("chat2"),
            TextFont::from_font(theme.font.clone()).with_font_size(30.0),
            TextColor::from(theme.white)
        ));
    });

    commands.spawn((
        Sprite::from_color(theme.black, Vec2::new(CHAT_WIDTH, CHAT_HEIGHT)),
        Transform::from_translation(Vec3::new(CHAT_LEFT + CHAT_WIDTH/2.0, CHAT_TOP - CHAT_HEIGHT/2.0, -1.)),
    ));

    const AI_INFO_TOP: f32 = 400.0;

    commands.spawn((
        Sprite::from_color(theme.green, Vec2::new(CHAT_WIDTH, CHAT_HEIGHT)),
        Transform::from_xyz(CHAT_LEFT + CHAT_WIDTH/2.0, AI_INFO_TOP - CHAT_HEIGHT/2.0, 0.0),
    )).with_child((
        AIInfoLabel,
        Text2d::new("ai"),
        TextFont::from_font(theme.font.clone()).with_font_size(30.0),
        TextColor::from(theme.white),
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
            TextColor::from(theme.grey)
        ));
    }
}
