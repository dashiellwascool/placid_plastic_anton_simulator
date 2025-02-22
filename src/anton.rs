use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

use crate::{GameAssets, GameState};

pub struct AntonPlugin;
impl Plugin for AntonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BillboardPlugin);
        app.add_systems(OnEnter(GameState::Playing), spawn_anton);
        //app.add_systems(Update, (
        //    move_antons
        //).run_if(in_state(GameState::Playing)));
    }
}

const PIXELS_PER_METER: f32 = 1500.0;

fn spawn_anton(mut commands: Commands, assets: Res<GameAssets>, images: Res<Assets<Image>>, mut meshes: ResMut<Assets<Mesh>>) {
    let px_size = images.get(&assets.furryton).unwrap().size().as_vec2();
    let size = Vec2::new(px_size.x / PIXELS_PER_METER, px_size.y / PIXELS_PER_METER);

    commands.spawn((
        Anton,
        Transform::from_xyz(0.0, size.y / 2.0, 0.0),
        BillboardTexture(assets.furryton.clone()),
        BillboardMesh(meshes.add(Rectangle::from_size(size)))
    ));
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct Anton;