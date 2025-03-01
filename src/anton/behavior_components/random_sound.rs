use std::time::Duration;

use bevy::prelude::*;

use crate::misc::play_sound_event::PlaySoundEvent;

#[derive(Component)]
pub struct RandomSound {
    pub sound: Handle<AudioSource>,
    timer: Timer,
    /// in seconds
    pub min_time: f32,
    /// in seconds
    pub max_time: f32,
}

impl RandomSound {
    pub fn new(sound: Handle<AudioSource>, min_time: f32, max_time: f32) -> RandomSound {
        let mut sound = RandomSound {
            sound,
            min_time,
            max_time,
            timer: Timer::from_seconds(0.0, TimerMode::Once)
        };
        
        sound.reset_timer();

        sound
    }

    fn reset_timer(&mut self) {
        self.timer.reset();
        self.timer.set_duration(Duration::from_secs_f32(rand::random_range(self.min_time..self.max_time)));
    }
}

pub fn play_sound(mut query: Query<&mut RandomSound>, time: Res<Time>, mut commands: Commands) {
    for mut random_sound in &mut query {
        random_sound.timer.tick(time.delta());

        if random_sound.timer.just_finished() {
            commands.trigger(PlaySoundEvent(random_sound.sound.clone()));
            random_sound.reset_timer();
        }
    }
}
