use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use parry2d::{math::Point, na::Vector2, query::{Ray, RayCast}};

use crate::{apartment_mapping::ApartmentMesh, GameState};

#[derive(Component, Default)]
#[require(Speed, Transform, Velocity)]
pub struct Wandering {
    pub timer: Timer,
    pub moving: bool,
    pub goal: Vec2
}

#[derive(Component)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(0.005)
    }
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct Velocity(pub Vec2);

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_anton
        ).run_if(in_state(GameState::Playing)));

        app.add_systems(FixedUpdate, (
            do_movement
        ).run_if(in_state(GameState::Playing)));
    }
}

const OFFSET: f32 = 0.02;

fn do_movement(mut query: Query<(&mut Transform, &Velocity)>, map: Res<ApartmentMesh>, mut gizmos: Gizmos) {
    for (mut transform, velocity) in &mut query {
        let mut translation2d = Vec2::new(transform.translation.x, transform.translation.z);
        
        let dir = Vector2::new(velocity.0.x, velocity.0.y).normalize();
        let ray = Ray::new(Point::new(translation2d.x, translation2d.y), dir);
        let dist = map.cast_local_ray(&ray, velocity.0.length() + OFFSET, true);
        match dist {
            Some(d) => {
                if d > OFFSET {
                    translation2d += velocity.0.clamp_length_max(d - OFFSET);
                }
            },
            None => {
                translation2d += velocity.0;
            },
        }

        transform.translation = Vec3::new(translation2d.x, transform.translation.y, translation2d.y);
    }
}

fn move_anton(
    mut query: Query<(&mut Wandering, &Transform, &Speed, &mut Velocity)>, 
    collidable: Res<ApartmentMesh>, 
    time: Res<Time>
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
                let ray = Ray::new(
                    Point::new(transform.translation.x, transform.translation.z), 
                    Vector2::new(angle.x,angle.y)
                );
                
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
