use bevy::prelude::*;

use crate::anton::anton_type::AntonType;
use crate::ui::timer::TimerText;
use crate::GameState;
use crate::anton::spawn_anton::SpawnAnton;

pub struct GameHandlerPlugin;
impl Plugin for GameHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
        app.add_systems(Update, (
            update_timer
        ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct SpawnTimer(Timer);

fn setup(mut commands: Commands) {
    commands.trigger(SpawnAnton(Some(AntonType::AntonParty))); // TODO: replace with regular anton

    // spawn
    commands.spawn(SpawnTimer(Timer::from_seconds(60., TimerMode::Repeating)));

    for _ in 0..100 {
        commands.trigger(SpawnAnton(None)); // TODO: replace with regular anton
    }
}

fn update_timer(mut commands: Commands, mut query: Query<&mut SpawnTimer>, time: Res<Time>, mut timer_ui: Query<&mut TimerText>) {
    let mut timer = query.single_mut();

    timer.0.tick(time.delta());

    timer_ui.single_mut().0 = format!("{}", timer.0.remaining().as_secs());

    if timer.0.just_finished() {
        commands.trigger(SpawnAnton(None));
    }
}