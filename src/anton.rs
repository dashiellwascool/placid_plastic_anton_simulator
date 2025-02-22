use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct AntonPlugin;
impl Plugin for AntonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_anton);
        app.add_systems(Update, (
            move_antons
        ).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_anton(mut commands: Commands, assets: Res<AntonAssets>, images: Res<Assets<Image>>) {
    let size = images.get(&assets.furryton).unwrap().size();
    commands.spawn((
        Anton::default(),
        Sprite {
            image: assets.furryton.clone(),
            custom_size: Some(size.as_vec2() * 0.25),
            ..default()
        },
    ));
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct Anton {
    movement: Vec2,
    timer: Timer
}

#[derive(AssetCollection, Resource)]
pub struct AntonAssets {
    #[asset(path = "antons/furryton.png")]
    furryton: Handle<Image>
}

fn move_antons(mut query: Query<(&mut Anton, &mut Transform)>, time: Res<Time>) {
    for (mut anton, mut transform) in &mut query {
        transform.translation += Vec3::new(anton.movement.x, anton.movement.y, 0.0) * time.delta_secs();

        anton.timer.tick(time.delta());
        if !anton.timer.finished() {
            continue;
        }

        anton.timer.reset();
        anton.timer.set_duration(Duration::from_secs_f32(rand::random_range(0.0..10.0)));

        if rand::random_range(0..=1) == 0 {
            let dir = rand::random_range(0.0..(PI * 2.0));
            anton.movement = Vec2::from_angle(dir) * 100.0;
        } else {
            anton.movement = Vec2::ZERO;
        }
    }
}
