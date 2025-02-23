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

pub struct ApartmentMappingPlugin;
impl Plugin for ApartmentMappingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup::setup);
        app.add_systems(Update, debug.run_if(in_state(GameState::Playing)));
    }
}

fn debug(mut gizmos: Gizmos, mesh: Res<ApartmentMesh>) {
    gizmos.linestrip(mesh.0.vertices().iter().map(|p| Vec3::new(p.x, 0., p.y)), BLUE);
    for p in mesh.0.vertices().iter() {
        gizmos.sphere(Isometry3d::from_xyz(p.x, 0., p.y), 0.05, BLUE);
    }

    let test_points = vec![Vec2::new(0., 0.), Vec2::new(-5., 0.), Vec2::new(-1.17689, -0.004271), Vec2::new(-1.14602, -0.04187)];
    for point in test_points {
        let success = mesh.point_inside(point);
        let color = match success {
            true => LIME,
            false => RED
        };

        gizmos.sphere(Isometry3d::from_xyz(point.x, 0., point.y), 0.01, color);

    }
}