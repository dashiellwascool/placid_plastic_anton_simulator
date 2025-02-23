use bevy::prelude::*;

pub mod anton_type;
pub mod spawn_anton;
mod movement;

pub struct AntonPlugin;
impl Plugin for AntonPlugin {
    fn build(&self, app: &mut App) {
        // Observer event for spawning in an Anton
        app.add_observer(spawn_anton::spawn_anton);
        
        app.add_plugins(movement::MovementPlugin);

    }
}

/// Marker component
#[derive(Component)]
#[require(Transform)]
pub struct Anton;