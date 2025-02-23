use anton::AntonPlugin;
use antons::AntonsPlugin;
use apartment::ApartmentPlugin;
use apartment_mapping::ApartmentMappingPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use camera::GameCameraPlugin;
use ui::TheUiPlugin;

mod camera;
mod anton;
mod apartment;
mod apartment_mapping;
mod ui;
mod antons;

pub struct PlacidPlasticAntonSimulatorPlugin;
impl Plugin for PlacidPlasticAntonSimulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Placid Plastic Anton Simulator".to_owned(),
                ..default()
            }),
            ..default()
        }));

        app.add_plugins((
            GameCameraPlugin,
            ApartmentPlugin,
            ApartmentMappingPlugin,
            AntonPlugin,
            TheUiPlugin,
            AntonsPlugin,
        ));

        app.init_state::<GameState>();

        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<GameAssets>()
        );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "apartment.glb")]
    apartment: Handle<Gltf>,

    #[asset(path = "apartment_floor.glb")]
    apartment_floor: Handle<Gltf>,

    #[asset(path = "antons/furryton.png")]
    furryton: Handle<Image>,

    #[asset(path = "fonts/FuzzyBubbles-Regular.ttf")]
    font_fuzzybubbles: Handle<Font>,

    #[asset(path = "fonts/FuzzyBubbles-Bold.ttf")]
    font_fuzzybubbles_bold: Handle<Font>,
}
