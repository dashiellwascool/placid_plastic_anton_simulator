use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use movement::{MovementPlugin, Velocity};

use crate::{GameAssets, GameState};

mod movement;

pub struct AntonPlugin;
impl Plugin for AntonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BillboardPlugin, MovementPlugin));
        app.add_systems(OnEnter(GameState::Playing), spawn_anton);
        app.add_systems(FixedUpdate, (
            move_anton
        ).run_if(in_state(GameState::Playing)));
    }
}

const PIXELS_PER_METER: f32 = 1500.0;

fn spawn_anton(mut commands: Commands, assets: Res<GameAssets>, images: Res<Assets<Image>>, mut meshes: ResMut<Assets<Mesh>>) {
    
    let px_size = images.get(&assets.furryton).unwrap().size().as_vec2();
    let size = Vec2::new(px_size.x / PIXELS_PER_METER, px_size.y / PIXELS_PER_METER);

    for _ in 0..1000 {
        commands.spawn((
            Anton,
            Transform::from_xyz(0.0, size.y / 2.0, 0.0),
            BillboardTexture(assets.furryton.clone()),
            BillboardMesh(meshes.add(Rectangle::from_size(size))),
            BillboardLockAxis { y_axis: true, rotation: false },
            Wandering::default()
        ));
    }
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct Anton;

#[derive(Component, Default)]
#[require(Speed, Transform, Velocity)]
pub struct Wandering {
    timer: Timer,
    moving: bool
}

#[derive(Component)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(0.01)
    }
}

fn move_anton(mut query: Query<(&mut Transform, &mut Wandering, &Speed, &mut Velocity)>, mut commands: Commands, time: Res<Time>) {
    for (mut transform, mut moving, speed, mut velocity) in &mut query {
        moving.timer.tick(time.delta());
        if !moving.timer.just_finished() {
            continue;
        }

        moving.timer.reset();
        moving.timer.set_duration(Duration::from_secs_f32(rand::random_range(0.0..3.0)));
        
        moving.moving = !moving.moving;

        if moving.moving {
            let dir = Vec2::from_angle(rand::random_range(0.0..(2.0 * PI)));
            velocity.0 = dir * speed.0;
        } else {
            velocity.0 = Vec2::ZERO;
        }
    }
}
