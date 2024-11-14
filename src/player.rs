use std::process::Command;
use bevy::prelude::*;
use crate::{GameTextures, WindowSize, PLAYER_SPRITE_WH, SPRITE_SCALE};
use crate::component::Player;
use crate::movement::{Movement, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands, game_textures: Res<GameTextures>, window_size: Res<WindowSize>) {
    let bottom = -window_size.half_height();
    let sprite_scaled_h = PLAYER_SPRITE_WH.1 * SPRITE_SCALE;

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

fn player_movement(mut query: Query<&mut Velocity, With<Player>>, button_input: Res<ButtonInput<KeyCode>>) {
    // if player still alive
    if let Ok(mut velocity) = query.get_single_mut() {
        let velocity_x = if button_input.pressed(KeyCode::ArrowLeft) || button_input.pressed(KeyCode::KeyA) {
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
        }*/;

        velocity.value.x = velocity_x;
    }
}

fn player_fire(mut command: Command, game_textures: Res<GameTextures>, window_size: Res<WindowSize>) {

}