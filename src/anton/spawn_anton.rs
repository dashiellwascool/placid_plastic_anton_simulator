use bevy::{prelude::*, utils::HashSet};
use bevy_mod_billboard::prelude::*;

use crate::{anton::{anton_type::AntonType, movement::Wandering, Anton}, apartment::{Door, DoorAnimationNodeIndex}, GameAssets};

use super::anton_type::SPAWNABLE_ANTONS;

const PX_SIZE: Vec2 = Vec2::new(496., 662.); // :o)
const PIXELS_PER_METER: f32 = 1500.0;

const SPAWN_LOCATION: Vec3 = Vec3::new(0.663, 0., -0.350);

#[derive(Event)]
pub struct SpawnAnton(pub Option<AntonType>);
pub fn spawn_anton(
    event: Trigger<SpawnAnton>,
    antons: Query<&AntonType>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut door_query: Query<(&mut AnimationPlayer, &DoorAnimationNodeIndex), With<Door>>,
) {

    let anton_type = match event.0 {
        Some(t) => t,
        None => {
            // spawn unique antons until we run out of antons then just spawn anything
            let mut spawnables: HashSet<AntonType> = HashSet::from(SPAWNABLE_ANTONS);
            for anton in &antons {
                spawnables.remove(anton);
            }
            if spawnables.is_empty() {
                spawnables = HashSet::from(SPAWNABLE_ANTONS);
            }
            let spawnables: Vec<AntonType> = spawnables.into_iter().collect();

            spawnables[rand::random_range(0..spawnables.len())]
        }
    };


    // I still don't like this and think this should be generalized and not hard coded
    // Something like a pixels-per-meter thing instead. - Dashiell
    let size = Vec2::new(PX_SIZE.x / PIXELS_PER_METER, PX_SIZE.y / PIXELS_PER_METER);

    let anton = AntonType::spawn(anton_type, &mut commands, &assets);
    let mut anton = commands.entity(anton);
    
    anton.insert((
        Anton,
        Transform::from_xyz(SPAWN_LOCATION.x, size.y / 2.0, SPAWN_LOCATION.z),
        BillboardMesh(meshes.add(Rectangle::from_size(size))),
        BillboardLockAxis { y_axis: true, rotation: false },
        Wandering::default(),
    ));

    // Start door animation
    for (mut player, index) in &mut door_query {
        player.stop(index.0);
        player.play(index.0);
    }

}