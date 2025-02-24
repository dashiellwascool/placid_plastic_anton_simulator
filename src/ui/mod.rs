use bevy::prelude::*;

use crate::GameState;

pub mod jointext;
mod timer;

pub struct TheUiPlugin;
impl Plugin for TheUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (timer::setup, jointext::setup));
        app.add_systems(
            Update,
            (jointext::update).run_if(in_state(GameState::Playing)),
        );
        app.add_observer(jointext::spawn_join_text);
    }
}
