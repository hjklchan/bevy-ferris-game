use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    component::Enemy,
    movement::{Movement, Velocity},
    GameTextures, WindowSize, ENEMY_SPRITE_SCALED_WH, SPRITE_SCALE,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
) {
    let mut rng = thread_rng();
    let spawn_range_x = rng.gen_range(-window_size.half_width()..window_size.half_width());
    let spawning_y = window_size.half_height() + ENEMY_SPRITE_SCALED_WH.1;

    commands.spawn((
        Enemy,
        SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                translation: Vec3::new(spawn_range_x, spawning_y, 0.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Movement,
        Velocity {
            value: Vec3::new(0.0, -1.0, 0.0),
        },
    ));
}
