use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use rand::{thread_rng, Rng};

use crate::{
    component::{Enemy, FromEnemy, Laser, Player},
    movement::{Movement, Velocity},
    GameTextures, SpawnTimer, WindowSize, ENEMY_LASER_SPRITE_SCALED_WH, ENEMY_SPRITE_SCALED_WH,
    PLAYER_SPRITE_SCALED_WH, SPRITE_SCALE,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::default());
        app.add_systems(Update, spawn_enemy);
        app.add_systems(Update, enemy_fire);
        app.add_systems(Update, enemy_laser_hit_player);
        app.add_systems(Update, despawn_enemy);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.enemy.tick(time.delta());

    let mut rng = thread_rng();
    let spawn_range_x = rng.gen_range(-window_size.half_width()..window_size.half_width());
    let spawning_y = window_size.half_height() + ENEMY_SPRITE_SCALED_WH.1;

    if spawn_timer.enemy.finished() {
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
            Movement { per: 0.6 },
            Velocity {
                value: Vec3::new(0.0, -1.0, 0.0),
            },
        ));
    }
}

fn enemy_fire(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    game_textures: Res<GameTextures>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    for enemy_tf in enemy_query.iter() {
        spawn_timer.enemy_laser.tick(time.delta());

        let translation = &enemy_tf.translation;
        let attack_port_x = translation.x;
        let attack_port_y = translation.y - ENEMY_SPRITE_SCALED_WH.1;

        if spawn_timer.enemy_laser.finished() {
            // 生成敌人镭射
            commands.spawn((
                Laser,
                FromEnemy,
                Velocity {
                    value: Vec3::new(0.0, -1.0, 0.0),
                },
                Movement { per: 2.0 },
                SpriteBundle {
                    texture: game_textures.enemy_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(attack_port_x, attack_port_y, 0.0),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                        // rotation: todo
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        }
    }
}

#[allow(clippy::type_complexity)]
fn enemy_laser_hit_player(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    // determine if player still alive
    if let Ok((player_ent, player_tf)) = player_query.get_single() {
        for (laser_ent, laser_tf) in laser_query.iter() {
            let is_collide = Aabb2d::new(
                laser_tf.translation.truncate(),
                Vec2::new(
                    ENEMY_LASER_SPRITE_SCALED_WH.0 / 2.0,
                    ENEMY_LASER_SPRITE_SCALED_WH.1 / 2.0,
                ),
            )
            .intersects(&Aabb2d::new(
                player_tf.translation.truncate(),
                Vec2::new(
                    PLAYER_SPRITE_SCALED_WH.0 / 2.0,
                    PLAYER_SPRITE_SCALED_WH.1 / 2.0,
                ),
            ));

            if is_collide {
                // remove the specified laser entity
                commands.entity(laser_ent).despawn();
                // remove the player entity
                commands.entity(player_ent).despawn();
            }
        }
    }
}

fn despawn_enemy(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Enemy>>,
    window_size: Res<WindowSize>,
) {
    let bottom = -window_size.half_height();

    for (entity, transform) in query.iter() {
        if transform.translation.y < bottom {
            commands.entity(entity).despawn();
        }
    }
}
