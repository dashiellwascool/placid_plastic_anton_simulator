use anton::AntonPlugin;
use apartment::ApartmentPlugin;
use apartment_mapping::ApartmentMappingPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use camera::GameCameraPlugin;
use game_handler::GameHandlerPlugin;
use misc::MiscPlugin;
use ui::TheUiPlugin;
use bevy_mod_billboard::prelude::*;

mod anton;
mod apartment;
mod apartment_mapping;
mod camera;
mod ui;
mod game_handler;
mod misc;

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
            MiscPlugin,
            // Library plugins
            BillboardPlugin,
            EmbeddedAssetPlugin::default()
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
    #[asset(path = "embedded://apartment.glb")]
    apartment: Handle<Gltf>,
    #[asset(path = "embedded://apartment_floor.glb")]
    apartment_floor: Handle<Gltf>,
    #[asset(path = "embedded://door.glb")]
    door: Handle<Gltf>,

    // ui assets
    #[asset(path = "embedded://fonts/FuzzyBubbles-Regular.ttf")]
    font_fuzzybubbles: Handle<Font>,
    /* #[asset(path = "fonts/FuzzyBubbles-Bold.ttf")]
    font_fuzzybubbles_bold: Handle<Font>, */

    #[asset(path = "embedded://ui/danger-icon.png")]
    ui_danger_icon: Handle<Image>,

    // Sounds
    #[asset(path = "embedded://sfx/honk.ogg")]
    sfx_honk: Handle<AudioSource>,
    #[asset(path = "embedded://sfx/alert-destroyed.ogg")]
    sfx_factorio_alert: Handle<AudioSource>,

    // antons
    #[asset(path = "embedded://antons/bus_driver.png")]
    bus_driver: Handle<Image>,
    #[asset(path = "embedded://antons/caffeine_king.png")]
    caffeine_king: Handle<Image>,
    #[asset(path = "embedded://antons/camping_ton.png")]
    camping_ton: Handle<Image>,
    #[asset(path = "embedded://antons/ctrl_f_u.png")]
    ctrl_f_u: Handle<Image>,
    #[asset(path = "embedded://antons/daily_anton_16.png")]
    daily_anton_16: Handle<Image>,
    #[asset(path = "embedded://antons/friday_frogger.png")]
    friday_frogger: Handle<Image>,
    #[asset(path = "embedded://antons/furryton.png")]
    furryton: Handle<Image>,
    #[asset(path = "embedded://antons/greger.png")]
    greger: Handle<Image>,
    #[asset(path = "embedded://antons/happy_zergling.png")]
    happy_zergling: Handle<Image>,
    #[asset(path = "embedded://antons/honk_networker.png")]
    honk_networker: Handle<Image>,
    #[asset(path = "embedded://antons/molerat.png")]
    molerat: Handle<Image>,
    #[asset(path = "embedded://antons/mug.png")]
    mug: Handle<Image>,
    #[asset(path = "embedded://antons/pentacat.png")]
    pentacat: Handle<Image>,
    #[asset(path = "embedded://antons/pernilla.png")]
    pernilla: Handle<Image>,
    #[asset(path = "embedded://antons/pet_rocket_racer.png")]
    pet_rocket_racer: Handle<Image>, // it just keeps going!!!
    #[asset(path = "embedded://antons/rover.png")]
    rover: Handle<Image>,
    #[asset(path = "embedded://antons/rubton.png")]
    rubton: Handle<Image>,
    #[asset(path = "embedded://antons/scarecrow_art.png")]
    scarecrow_art: Handle<Image>,
    #[asset(path = "embedded://antons/smoking_catboy_anton_flipping_you_off.png")]
    smoking_catboy_anton_flipping_you_off: Handle<Image>,
    #[asset(path = "embedded://antons/snail_trail.png")]
    snail_trail: Handle<Image>,
    #[asset(path = "embedded://antons/vanilla_icecream.png")]
    vanilla_icecream: Handle<Image>,
    #[asset(path = "embedded://antons/warewolf.png")]
    warewolf: Handle<Image>,
    #[asset(path = "embedded://antons/xtreme_xplosive_fisher.png")]
    xtreme_xplosive_fisher: Handle<Image>,
    #[asset(path = "embedded://antons/fax.png")]
    fax: Handle<Image>,
    #[asset(path = "embedded://antons/gooby_hobo.png")]
    gooby_hobo: Handle<Image>,
    #[asset(path = "embedded://antons/baby_with_a_hammer.png")]
    baby_with_a_hammer: Handle<Image>,
    #[asset(path = "embedded://antons/biter.png")]
    biter: Handle<Image>,
    #[asset(path = "embedded://antons/one_more.png")]
    one_more: Handle<Image>,
    #[asset(path = "embedded://antons/cortana.png")]
    cortana: Handle<Image>,
    #[asset(path = "embedded://antons/anton_party.png")]
    anton_party: Handle<Image>,
    #[asset(path = "embedded://antons/ciaociao.png")]
    ciaociao: Handle<Image>,
}
