use bevy::prelude::*;

use crate::GameState;

pub mod random_sound;

pub struct BehaviorsPlugin;
impl Plugin for BehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (random_sound::play_sound).run_if(in_state(GameState::Playing)));
    }
}