mod camera;
mod component;
mod enemy;
mod movement;
mod player;

use crate::camera::CameraPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use component::Laser;
use enemy::EnemyPlugin;

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
        .add_plugins(EnemyPlugin)
        .add_systems(Update, despawn_laser)
        .run();
}

// 玩家精灵的路径
const PLAYER_SPRITE_PATH: &str = "player_a_01.png";
// 玩家镭射精灵的路径
const PLAYER_LASER_SPRITE_PATH: &str = "laser_a_01.png";
// 敌人精灵的路径
const ENEMY_SPRITE_PATH: &str = "enemy_a_01.png";
// 敌人镭射的精灵路径
const ENEMY_LASER_SPRITE_PATH: &str = "laser_b_01.png";

// 精灵缩放的比例
const SPRITE_SCALE: f32 = 0.5;
// 玩家精灵的宽高
const PLAYER_SPRITE_WH: (f32, f32) = (144.0, 75.0);
// Player - calculate the width and height after scaling
// 玩家精灵缩放后的宽高
const PLAYER_SPRITE_SCALED_WH: (f32, f32) = (
    PLAYER_SPRITE_WH.0 * SPRITE_SCALE,
    PLAYER_SPRITE_WH.1 * SPRITE_SCALE,
);

// 玩家镭射精灵的宽高
const PLAYER_LASER_SPRITE_WH: (f32, f32) = (9.0, 54.0);
// Player' laser - calculate the width and height after scaling
// 玩家镭射精灵缩放后的宽高
const PLAYER_LASER_SPRITE_SCALED_WH: (f32, f32) = (
    PLAYER_LASER_SPRITE_WH.0 * SPRITE_SCALE,
    PLAYER_LASER_SPRITE_WH.1 * SPRITE_SCALE,
);

// 敌人精灵的宽高
const ENEMY_SPRITE_WH: (f32, f32) = (93.0, 84.0);
// 敌人精灵缩放后的宽高
const ENEMY_SPRITE_SCALED_WH: (f32, f32) = (
    ENEMY_SPRITE_WH.0 * SPRITE_SCALE,
    ENEMY_SPRITE_WH.1 * SPRITE_SCALE,
);
// 敌人镭射精灵的宽高
const ENEMY_LASER_SPRITE_WH: (f32, f32) = (17.0, 55.0);
// 敌人镭射精灵缩放后的宽高
const ENEMY_LASER_SPRITE_SCALED_WH: (f32, f32) = (
    ENEMY_LASER_SPRITE_WH.0 * SPRITE_SCALE,
    ENEMY_LASER_SPRITE_WH.1 * SPRITE_SCALE,
);

// 基础移动速度
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
    enemy: Handle<Image>,
    player_laser: Handle<Image>,
    enemy_laser: Handle<Image>,
}

// 生成实体 Timer
#[derive(Resource)]
pub struct SpawnTimer {
    enemy: Timer,
    enemy_laser: Timer,
}

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            enemy: Timer::from_seconds(2.0, TimerMode::Repeating),
            enemy_laser: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
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
        enemy: asset_server.load(ENEMY_SPRITE_PATH),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE_PATH),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE_PATH),
    };

    commands.insert_resource(game_textures_resource);
}

fn despawn_laser(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Laser>>,
    window_size: Res<WindowSize>,
) {
    let max_window_h = window_size.half_height();
    let min_window_h = -window_size.half_height();

    for (entity, transform) in query.iter() {
        if transform.translation.y > max_window_h || transform.translation.y < min_window_h {
            commands.entity(entity).despawn();
        }
    }
}