use anton::AntonPlugin;
use apartment::ApartmentPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use camera::GameCameraPlugin;

mod camera;
mod anton;
mod apartment;

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
            AntonPlugin
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

    #[asset(path = "antons/furryton.png")]
    furryton: Handle<Image>
}
