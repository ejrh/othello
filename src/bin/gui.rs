use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle};
use bevy::window::close_on_esc;

use othello::ai::{AI, MinimaxAI};
use othello::game::{Board, Colour, DefaultGame, Move, Pos};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_pieces, update_current_square, update_score))
        .add_systems(Update, click_square)
        .add_systems(Update, update_ai)
        .add_systems(Update, close_on_esc)
        .init_resource::<Colours>()
        .init_resource::<CurrentGame>()
        .init_resource::<CurrentSquare>()
        .run();
}

#[derive(Resource)]
struct CurrentGame {
    game: DefaultGame,
    player1: Player,
    player2: Player,
}

struct Player {
    name: String,
    ai: Option<MinimaxAI>
}

impl Default for CurrentGame {
    fn default() -> Self {
        CurrentGame {
            game: DefaultGame::new(),
            player1: Player {
                name: String::from("Computer"),
                ai: Some(MinimaxAI { max_depth: 3 }),
            },
            player2: Player {
                name: String::from("Human"),
                ai: None,
            }
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

fn setup(
    mut commands: Commands,
    mut colours: ResMut<Colours>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>)
{
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

    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
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

fn update_score(
    mut labels: Query<(&ScoreLabel, &mut Text)>,
    current_game: Res<CurrentGame>,
) {
    let scores = current_game.game.scores();

    for (label, mut text) in labels.iter_mut() {
        let (name, score) = match label.0 {
            Colour::Black => (&current_game.player1.name, scores.0),
            Colour::White => (&current_game.player2.name, scores.1),
        };
        text.sections[0].value = format!("{name}: {score}");
    }
}

fn click_square(
    input: Res<Input<MouseButton>>,
    mut current_game: ResMut<CurrentGame>,
    current_square: Res<CurrentSquare>,
) {
    if current_square.row == -1 || current_square.col == -1 {
        return;
    }
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let player = current_game.game.next_turn;
    let mov = Move {
        player,
        row: current_square.row,
        col: current_square.col,
    };
    if !current_game.game.board.is_valid_move(mov) {
        return;
    }
    let new_game = current_game.game.apply(mov);
    current_game.game = new_game;
}

fn update_ai(
    mut current_game: ResMut<CurrentGame>,
) {
    let player = match current_game.game.next_turn {
        Colour::Black => &current_game.player1,
        Colour::White => &current_game.player2,
    };

    let Some(ref ai) = player.ai else { return };

    let Some(mov) = ai.choose_move(&current_game.game)
        else { return };

    if !current_game.game.board.is_valid_move(mov) {
        return;
    }
    let new_game = current_game.game.apply(mov);
    current_game.game = new_game;
}
