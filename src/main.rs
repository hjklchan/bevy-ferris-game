mod camera;
mod component;
mod movement;
mod player;

use crate::camera::CameraPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::from(Vec2::new(598.0, 676.0)),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, load_assets)
        .add_systems(Startup, setup)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin::with_debug())
        .run();
}

const PLAYER_SPRITE_PATH: &str = "player_a_01.png";
const LASER_SPRITE_PATH: &str = "laser_a_01.png";

const SPRITE_SCALE: f32 = 0.5;
const PLAYER_SPRITE_WH: (f32, f32) = (144.0, 75.0);
// calculate the width and height after scaling
const PLAYER_SPRITE_SCALED_WH: (f32, f32) = (
    PLAYER_SPRITE_WH.0 * SPRITE_SCALE,
    PLAYER_SPRITE_WH.1 * SPRITE_SCALE,
);
const LASER_SPRITE_WH: (f32, f32) = (9.0, 54.0);
// calculate the width and height after scaling
const LASER_SPRITE_SCALED_WH: (f32, f32) = (
    LASER_SPRITE_WH.0 * SPRITE_SCALE,
    LASER_SPRITE_WH.1 * SPRITE_SCALE,
);
const BASE_MOVEMENT_SPEED: f32 = 500.0;

#[derive(Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl WindowSize {
    fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    fn half_height(&self) -> f32 {
        self.height / 2.0
    }
}

#[derive(Resource)]
pub struct GameTextures {
    player: Handle<Image>,
    laser: Handle<Image>,
}

pub fn setup(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    // Insert "WindowSize" resource
    let window = window.single();
    let window_size_resource = WindowSize {
        width: window.width(),
        height: window.height(),
    };
    commands.insert_resource(window_size_resource);
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures_resource = GameTextures {
        player: asset_server.load(PLAYER_SPRITE_PATH),
        laser: asset_server.load(LASER_SPRITE_PATH),
    };

    commands.insert_resource(game_textures_resource);
}
