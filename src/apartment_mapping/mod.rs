use bevy::{color::palettes::css::BLUE, gltf::GltfMesh, prelude::*};
use parry2d::{math::Point, shape::TriMesh};

use crate::{GameAssets, GameState};

mod setup;

#[derive(Resource, Deref, DerefMut)]
pub struct ApartmentMesh(TriMesh);

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
}
