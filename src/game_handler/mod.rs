use bevy::prelude::*;

use crate::ui::timer::TimerText;
use crate::GameState;
use crate::anton::spawn_anton::SpawnRandomAnton;

pub struct GameHandlerPlugin;
impl Plugin for GameHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
        app.add_systems(Update, (
            update_timer
        ).run_if(in_state(GameState::Playing)));
    }
}

const TEST_ANTONS: u32 = 10;

#[derive(Component)]
pub struct SpawnTimer(Timer);

fn setup(mut commands: Commands) {
    for _ in 0..TEST_ANTONS {
        commands.trigger(SpawnRandomAnton);
    }

    // spawn
    commands.spawn(SpawnTimer(Timer::from_seconds(60., TimerMode::Repeating)));
}

fn update_timer(mut commands: Commands, mut query: Query<&mut SpawnTimer>, time: Res<Time>, mut timer_ui: Query<&mut TimerText>) {
    let mut timer = query.single_mut();

    timer.0.tick(time.delta());

    timer_ui.single_mut().0 = format!("{}", timer.0.remaining().as_secs());

    if timer.0.just_finished() {
        commands.trigger(SpawnRandomAnton);
    }
}