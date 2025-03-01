use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_billboard::BillboardText;

use crate::{GameAssets, GameState};

pub const MIN_TIME: f32 = 10.;
pub const MAX_TIME: f32 = 120.;
pub const HOLD_TIME: f32 = 5.;

pub struct TalkativePlugin;
impl Plugin for TalkativePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            talk_waiter,
            talking
        ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
#[require(BillboardText)]
pub struct Talkative {
    messages: Vec<String>,
    timer: Timer,
    min_time: f32,
    max_time: f32,
    hold_time: f32
}

#[derive(Component)]
struct Talking;

#[derive(Bundle)]
pub struct TalkativeBundle {
    pub transform: Transform,
    pub text_font: TextFont,
    pub talkative: Talkative
}

impl Talkative {
    pub fn new(messages: Vec<String>, min_time: f32, max_time: f32, hold_time: f32) -> Talkative {
        Talkative {
            messages,
            min_time,
            max_time,
            hold_time,
            timer: Timer::from_seconds(0.0, TimerMode::Once)
        }
    }

    pub fn new_str(messages: Vec<&str>, min_time: f32, max_time: f32, hold_time: f32) -> Talkative {
        Talkative::new(messages.iter().map(|s| s.to_string()).collect::<Vec<String>>(), min_time, max_time, hold_time)
    }

    pub fn default_strs(messages: Vec<&str>) -> Talkative {
        Talkative::new_str(messages, MIN_TIME, MAX_TIME, HOLD_TIME)
    }
}

impl TalkativeBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self { 
            transform: Transform::from_xyz(0.0, 0.25, 0.0).with_scale(Vec3::splat(0.002)), 
            text_font: TextFont::from_font(assets.font_fuzzybubbles.clone()), 
            talkative: Talkative::new(Vec::new(), 0.0, 0.0, 0.0) 
        }
    }
}

fn talk_waiter(mut query: Query<(&mut Talkative, Entity, &mut BillboardText), Without<Talking>>, time: Res<Time>, mut commands: Commands) {
    for (mut talkative, entity, mut text) in &mut query {
        talkative.timer.tick(time.delta());

        if talkative.timer.just_finished() {
            let message = talkative.messages[rand::random_range(0..talkative.messages.len())].clone();
            let time = talkative.hold_time;
            talkative.timer.reset();
            talkative.timer.set_duration(Duration::from_secs_f32(time));

            text.0 = message;
            commands.entity(entity).insert(Talking);
        }
    }
}

fn talking(mut query: Query<(&mut Talkative, Entity, &mut BillboardText), With<Talking>>, time: Res<Time>, mut commands: Commands) {
    for (mut talkative, entity, mut text) in &mut query {
        talkative.timer.tick(time.delta());

        if talkative.timer.just_finished() {
            commands.entity(entity).remove::<Talking>();
            text.0 = String::new();

            let time = rand::random_range(talkative.min_time..talkative.max_time);
            talkative.timer.reset();
            talkative.timer.set_duration(Duration::from_secs_f32(time));
        }
    }
}