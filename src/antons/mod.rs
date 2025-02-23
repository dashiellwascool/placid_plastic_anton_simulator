use bevy::prelude::*;

mod behaviors;
mod antons;

pub struct AntonsPlugin;
impl Plugin for AntonsPlugin {
    fn build(&self, app: &mut App) {
        behaviors::register_behaviors(app);
    }
}