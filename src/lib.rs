use anton::AntonPlugin;
use apartment::ApartmentPlugin;
use apartment_mapping::ApartmentMappingPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_easings::EasingsPlugin;
use camera::GameCameraPlugin;
use game_handler::GameHandlerPlugin;
use ui::TheUiPlugin;
use bevy_mod_billboard::prelude::*;

mod anton;
mod apartment;
mod apartment_mapping;
mod camera;
mod ui;
mod game_handler;

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
            // Our plugins
            GameCameraPlugin,
            ApartmentPlugin,
            ApartmentMappingPlugin,
            AntonPlugin,
            TheUiPlugin,
            GameHandlerPlugin,
            // Library plugins
            BillboardPlugin,
            EasingsPlugin::default(),
        ));

        app.init_state::<GameState>();

        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<GameAssets>(),
        );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

// wow this is awful...
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // world assets
    #[asset(path = "apartment.glb")]
    apartment: Handle<Gltf>,
    #[asset(path = "apartment_floor.glb")]
    apartment_floor: Handle<Gltf>,

    // ui assets
    #[asset(path = "fonts/FuzzyBubbles-Regular.ttf")]
    font_fuzzybubbles: Handle<Font>,
    #[asset(path = "fonts/FuzzyBubbles-Bold.ttf")]
    font_fuzzybubbles_bold: Handle<Font>,

    // antons
    #[asset(path = "antons/bus_driver.png")]
    bus_driver: Handle<Image>,
    #[asset(path = "antons/caffeine_king.png")]
    caffeine_king: Handle<Image>,
    #[asset(path = "antons/camping_ton.png")]
    camping_ton: Handle<Image>,
    #[asset(path = "antons/ctrl_f_u.png")]
    ctrl_f_u: Handle<Image>,
    #[asset(path = "antons/daily_anton_16.png")]
    daily_anton_16: Handle<Image>,
    #[asset(path = "antons/friday_frogger.png")]
    friday_frogger: Handle<Image>,
    #[asset(path = "antons/furryton.png")]
    furryton: Handle<Image>,
    #[asset(path = "antons/greger.png")]
    greger: Handle<Image>,
    #[asset(path = "antons/happy_zergling.png")]
    happy_zergling: Handle<Image>,
    #[asset(path = "antons/honk_networker.png")]
    honk_networker: Handle<Image>,
    #[asset(path = "antons/molerat.png")]
    molerat: Handle<Image>,
    #[asset(path = "antons/mug.png")]
    mug: Handle<Image>,
    #[asset(path = "antons/pentacat.png")]
    pentacat: Handle<Image>,
    #[asset(path = "antons/pernilla.png")]
    pernilla: Handle<Image>,
    #[asset(path = "antons/pet_rocket_racer.png")]
    pet_rocket_racer: Handle<Image>, // it just keeps going!!!
    #[asset(path = "antons/rover.png")]
    rover: Handle<Image>,
    #[asset(path = "antons/rubton.png")]
    rubton: Handle<Image>,
    #[asset(path = "antons/scarecrow_art.png")]
    scarecrow_art: Handle<Image>,
    #[asset(path = "antons/smoking_catboy_anton_flipping_you_off.png")]
    smoking_catboy_anton_flipping_you_off: Handle<Image>,
    #[asset(path = "antons/snail_trail.png")]
    snail_trail: Handle<Image>,
    #[asset(path = "antons/vanilla_icecream.png")]
    vanilla_icecream: Handle<Image>,
    #[asset(path = "antons/warewolf.png")]
    warewolf: Handle<Image>,
    #[asset(path = "antons/xtreme_xplosive_fisher.png")]
    xtreme_xplosive_fisher: Handle<Image>,
    #[asset(path = "antons/fax.png")]
    fax: Handle<Image>,
    #[asset(path = "antons/gooby_hobo.png")]
    gooby_hobo: Handle<Image>,
    #[asset(path = "antons/baby_with_a_hammer.png")]
    baby_with_a_hammer: Handle<Image>,
    #[asset(path = "antons/biter.png")]
    biter: Handle<Image>,
    #[asset(path = "antons/one_more.png")]
    one_more: Handle<Image>,
    #[asset(path = "antons/cortana.png")]
    cortana: Handle<Image>,
    #[asset(path = "antons/anton_party.png")]
    anton_party: Handle<Image>,
}
