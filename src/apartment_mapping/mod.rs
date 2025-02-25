use bevy::{color::palettes::css::{BLUE, LIME, RED}, prelude::*};
use parry2d::{math::Point, query::PointQuery, shape::TriMesh};

use crate::GameState;

mod setup;

#[derive(Resource, Deref, DerefMut)]
pub struct ApartmentMesh(TriMesh);

impl ApartmentMesh {
    pub fn point_inside(&self, point: Vec2) -> bool {
        self.contains_local_point(&Point::new(point.x, point.y))
    }
}

/// When true: Draws debug lines and funny things like that
const DEBUGGING: bool = false;

pub struct ApartmentMappingPlugin;
impl Plugin for ApartmentMappingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup::setup);
        app.add_systems(Update, debug.run_if(in_state(GameState::Playing).and(|| DEBUGGING)));
    }
}

fn debug(mut gizmos: Gizmos, mesh: Res<ApartmentMesh>) {

    let mut iter = mesh.triangles();
    loop {
        if let Some(triangle) = iter.next() {
          // line between a and b
          gizmos.line(Vec3::new(triangle.a.x, 0.0, triangle.a.y), Vec3::new(triangle.b.x, 0.0, triangle.b.y), Color::srgb(0.0, 0.0, 1.0));

          // line between b and c
          gizmos.line(Vec3::new(triangle.b.x, 0.0, triangle.b.y), Vec3::new(triangle.c.x, 0.0, triangle.c.y), Color::srgb(0.0, 0.0, 1.0));

          // line between c and a  
          gizmos.line(Vec3::new(triangle.c.x, 0.0, triangle.c.y), Vec3::new(triangle.a.x, 0.0, triangle.a.y), Color::srgb(0.0, 0.0, 1.0));
        } else {
            break;
        }
    }
}
