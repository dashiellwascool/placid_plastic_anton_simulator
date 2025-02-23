use bevy::prelude::*;

use crate::GameState;

mod timer;

pub struct TheUiPlugin;
impl Plugin for TheUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), timer::setup);
    }
}