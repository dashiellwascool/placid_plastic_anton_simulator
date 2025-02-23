use std::{f32::consts::PI, time::Duration};

use bevy::{gizmos, math::VectorSpace, prelude::*};
use bevy_mod_billboard::prelude::*;
use movement::{MovementPlugin, Velocity};
use parry2d::{math::Point, na::{Vector2, Vector3}, query::{Ray, RayCast}};

use crate::{apartment, apartment_mapping::ApartmentMesh, GameAssets, GameState};

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
    moving: bool,
    goal: Vec2
}

#[derive(Component)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(0.005)
    }
}

fn move_anton(
    mut query: Query<(&mut Wandering, &Transform, &Speed, &mut Velocity)>, 
    collidable: Res<ApartmentMesh>, 
    time: Res<Time>,
    mut gizmos: Gizmos
) {
    for (mut moving, transform, speed, mut velocity) in &mut query {
        if !moving.moving {
            moving.timer.tick(time.delta());
            if !moving.timer.finished() {
                continue;
            }

            let mut point: Option<Vec2> = None;
            for _ in 0..10 {
                let angle = Vec2::from_angle(rand::random_range(0.0..(2. * PI)));
                let dist = rand::random_range(0.0..=1.0);
                let test_point = Vec2::new(transform.translation.x, transform.translation.z) + 
                    (angle * dist);
                
                if collidable.point_inside(test_point) {
                    continue;
                }

                // setup raycast
                let ray = Ray::new(Point::new(transform.translation.x, transform.translation.z), Vector2::new(angle.x,angle.y));
                let cast = collidable.cast_local_ray(&ray, dist + 0.03, true);

                if cast.is_some() {
                    continue;
                }

                point = Some(test_point);
                break;
            }

            let Some(point) = point else { continue; };

            moving.goal = point;
            moving.moving = true;
        }
        
        // move them in the direction of the goal
        let translation2d = Vec2::new(transform.translation.x, transform.translation.z);
                
        if translation2d.distance(moving.goal) <= speed.0 {
            moving.moving = false;
            moving.timer.reset();
            moving.timer.set_duration(Duration::from_secs_f32(rand::random_range(0.0..=10.0)));
            velocity.0 = Vec2::ZERO;
        } else {
            let dir = moving.goal - translation2d;
            velocity.0 = dir.normalize() * speed.0;
        }
    }
}
