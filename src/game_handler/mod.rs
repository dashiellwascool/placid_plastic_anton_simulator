use bevy::prelude::*;

use crate::GameState;
use crate::anton::spawn_anton::SpawnRandomAnton;

pub struct GameHandlerPlugin;
impl Plugin for GameHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

const TEST_ANTONS: u32 = 500;

fn setup(mut commands: Commands) {
    for _ in 0..TEST_ANTONS {
        commands.trigger(SpawnRandomAnton);
    }
    
}