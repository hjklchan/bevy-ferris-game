use crate::component::{Laser, Player};
use crate::movement::{Movement, Velocity};
use crate::{GameTextures, WindowSize, PLAYER_SPRITE_SCALED_WH, SPRITE_SCALE};
use bevy::prelude::*;

#[derive(Default)]
pub struct PlayerPlugin {
    #[allow(dead_code)]
    debug: bool,
}

impl PlayerPlugin {
    pub fn with_debug() -> Self {
        Self { debug: true }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
        app.add_systems(Update, player_movement);
        app.add_systems(Update, player_fire);
        app.add_systems(Update, despawn_laser);
    }
}

fn spawn_player(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
) {
    let bottom = -window_size.half_height();
    let sprite_scaled_h = PLAYER_SPRITE_SCALED_WH.1;

    commands.spawn((
        Player,
        SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + sprite_scaled_h / 2.0, 0.0),
                scale: Vec3::splat(SPRITE_SCALE),
                ..Default::default()
            },
            ..Default::default()
        },
        Velocity::default(),
        Movement,
    ));
}

fn player_movement(
    mut query: Query<&mut Velocity, With<Player>>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    // if player still alive
    if let Ok(mut velocity) = query.get_single_mut() {
        let velocity_x = if button_input.pressed(KeyCode::ArrowLeft)
            || button_input.pressed(KeyCode::KeyA)
        {
            -1.0
        } else if button_input.pressed(KeyCode::ArrowRight) || button_input.pressed(KeyCode::KeyD) {
            1.0
        } else {
            0.
        };

        /*let velocity_y = if button_input.pressed(KeyCode::ArrowUp) || button_input.pressed(KeyCode::KeyW) {
            1.0
        } else if button_input.pressed(KeyCode::ArrowDown) || button_input.pressed(KeyCode::KeyS) {
            -1.0
        } else {
            0.
        };*/

        velocity.value.x = velocity_x;
    }
}

/// # player_fire
///
/// press `space` button to attach,
/// will spawn lasers on both hands.
fn player_fire(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    game_textures: Res<GameTextures>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    // if player still alive
    if let Ok(player_transform) = player_query.get_single() {
        if button_input.just_pressed(KeyCode::Space) {
            let player_translation = player_transform.translation;
            let sprite_scaled_w = PLAYER_SPRITE_SCALED_WH.0;
            let sprite_scaled_h = PLAYER_SPRITE_SCALED_WH.1;

            let left_hand_x = player_translation.x - sprite_scaled_w / 2.0;
            let right_hand_x = player_translation.x + sprite_scaled_w / 2.0;

            commands.spawn_batch([
                (
                    Laser,
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                right_hand_x,
                                player_translation.y + sprite_scaled_h,
                                0.0,
                            ),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                            ..Default::default()
                        },
                        texture: game_textures.laser.clone(),
                        ..Default::default()
                    },
                    Velocity {
                        value: Vec3::new(0.0, 1.0, 0.0),
                    },
                    Movement,
                ),
                (
                    Laser,
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                left_hand_x,
                                player_translation.y + sprite_scaled_h,
                                0.0,
                            ),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                            ..Default::default()
                        },
                        texture: game_textures.laser.clone(),
                        ..Default::default()
                    },
                    Velocity {
                        value: Vec3::new(0.0, 1.0, 0.0),
                    },
                    Movement,
                ),
            ]);
        }
    }
}

fn despawn_laser(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Laser>>,
    window_size: Res<WindowSize>,
) {
    let max_window_h = window_size.half_height() - 100.0;
    
    for (entity, transform) in query.iter() {
        if transform.translation.y > max_window_h {
            commands.entity(entity).despawn();
        }
    }
}
