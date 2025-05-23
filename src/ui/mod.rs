use bevy::prelude::*;

use crate::GameState;

pub mod jointext;
pub mod factorio_joke;
pub mod timer;
pub mod main_menu;

pub struct TheUiPlugin;
impl Plugin for TheUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (
            timer::setup, 
            jointext::setup,
            main_menu::setup
        ));
        app.add_systems(Update, (
            jointext::update, 
            factorio_joke::update_factorio_joke, 
            timer::update,
            main_menu::do_main_menu
        ).run_if(in_state(GameState::Playing)));

        app.init_resource::<factorio_joke::FactorioJokeTimer>();

        app.add_observer(jointext::spawn_join_text);
        app.add_observer(factorio_joke::spawn_factorio_joke);
    }
}
