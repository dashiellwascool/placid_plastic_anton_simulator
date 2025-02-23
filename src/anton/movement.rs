use bevy::prelude::*;
use parry2d::{math::Point, na::Vector2, query::{Ray, RayCast}};

use crate::{apartment_mapping::ApartmentMesh, GameState};

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            do_movement
        ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct Velocity(pub Vec2);

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
