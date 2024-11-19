use bevy::prelude::*;

use crate::{GameData, HitEvent};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_score_text);
        app.add_systems(Update, update_score_text);
    }
}

#[derive(Component)]
pub struct Score;

fn spawn_score_text(mut commands: Commands) {
    let score_text = Text::from_section("Score: 0", TextStyle::default());

    commands.spawn((
        Score,
        TextBundle {
            style: Style {
                ..Default::default()
            },
            text: score_text,

            ..Default::default()
        },
    ));
}

fn update_score_text(
    mut query: Query<&mut Text, With<Score>>,
    mut hit_evt: EventReader<HitEvent>,
    game_data: Res<GameData>,
) {
    for _ in hit_evt.read() {
        if let Ok(mut text) = query.get_single_mut() {
            if let Some(ts) = text.sections.get_mut(0) {
                ts.value = format!("Score: {}", game_data.num_score);
            }
        }
    }
    
}
