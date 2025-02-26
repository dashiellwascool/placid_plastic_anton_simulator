use bevy::prelude::*;

pub mod play_sound_event;

pub struct MiscPlugin;
impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(play_sound_event::play_sound_event);
    }
}