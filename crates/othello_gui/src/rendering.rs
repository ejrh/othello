use bevy::app::{App, Plugin, Startup};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::color::Color;
use bevy::color::palettes::basic::{BLACK, WHITE};
use bevy::color::palettes::css::{GOLD, LIMEGREEN};
use bevy::prelude::{Camera2d, ColorMaterial, Commands, Component, Font, OrthographicProjection, Projection, Res, ResMut, Resource};
use bevy::render::camera::ScalingMode;

use othello_game::{Colour, Pos};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Theme>()
            .init_resource::<CurrentSquare>()
            .add_systems(Startup, setup_theme)
            .add_systems(Startup, setup_camera);
    }
}

#[derive(Default, Resource)]
pub struct Theme {
    pub font: Handle<Font>,
    pub green: Color,
    pub gold: Color,
    pub black: Color,
    pub white: Color,
    pub black_material: Handle<ColorMaterial>,
    pub white_material: Handle<ColorMaterial>,
}

#[derive(Default, Resource)]
pub struct CurrentSquare {
    pub row: Pos,
    pub col: Pos
}

#[derive(Component)]
pub struct ScoreLabel(pub Colour);

#[derive(Component)]
pub struct TimeLabel(pub Colour);

#[derive(Component)]
pub struct AIInfoLabel;

pub(crate) fn setup_theme(
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

fn setup_camera(
    mut commands: Commands,
) {
    let camera = Camera2d;
    let mut proj = OrthographicProjection::default_2d();
    proj.scaling_mode = ScalingMode::FixedVertical { viewport_height: 1000.0 };
    commands.spawn((camera, Projection::Orthographic(proj)));
}
