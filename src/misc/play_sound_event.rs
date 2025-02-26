use bevy::prelude::*;

#[derive(Event)]
pub struct PlaySoundEvent(pub Handle<AudioSource>);

pub fn play_sound_event(trigger: Trigger<PlaySoundEvent>, mut commands: Commands) {
    // TODO: Audio setting?
    let source = trigger.event().0.clone();
    commands.spawn((AudioPlayer::new(source), PlaybackSettings::ONCE));

    // Keep in mind that it's also possible to play the sound directly in anton_type.rs
}
