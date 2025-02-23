use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use movement::{MovementPlugin, Velocity};
use rand::{distr::{Distribution, StandardUniform}, Rng};

use crate::{GameAssets, GameState};

mod movement;

pub struct AntonPlugin;
impl Plugin for AntonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BillboardPlugin, MovementPlugin));
        app.add_systems(OnEnter(GameState::Playing), spawn_anton);
    }
}

const PIXELS_PER_METER: f32 = 1500.0;

fn spawn_anton(mut commands: Commands, assets: Res<GameAssets>, images: Res<Assets<Image>>, mut meshes: ResMut<Assets<Mesh>>) {
    
    let px_size = images.get(&assets.furryton).unwrap().size().as_vec2();
    let size = Vec2::new(px_size.x / PIXELS_PER_METER, px_size.y / PIXELS_PER_METER);

    for _ in 0..1000 {
        Anton::spawn_random(&mut commands, &assets, &images, &mut meshes);
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Anton(pub AntonType);

#[derive(Component, Default)]
#[require(Speed, Transform, Velocity)]
pub struct Wandering {
    timer: Timer,
    moving: bool,
    goal: Vec2
}

#[derive(Component)]
pub struct Speed(f32);

#[derive(Clone, Copy)]
pub enum AntonType {
    Furryton,
    BusDriver,
    CampingTon,
    PetRocketRacer,
    SmokingCatboyAntonFlippingYouOff,
    CaffeineKing,
    CtrlFU,
    Mug,
    ScarecrowArt,
    Rubton,
    HonkNetworker,
    HappyZergling,
    SnailTrail,
    XtremeXplosiveFisher,
    Pentacat,
    Greger,
    Rover,
    Warewolf,
    Pernilla,
    VanillaIcecream,
    Molerat,
    FridayFrogger,
    DailyAnton16,
}

impl Default for Speed {
    fn default() -> Self {
        Self(0.005)
    }
}

impl Anton {
    // there are so many things here what if we just made an observer that spawns antons instead???
    pub fn spawn(anton: AntonType, commands: &mut Commands, assets: &Res<GameAssets>, images: &Res<Assets<Image>>, meshes: &mut ResMut<Assets<Mesh>>) -> Entity {
        let image = AntonType::get_image(anton, assets);
        let px_size = images.get(&image).unwrap().size().as_vec2();
        let size = Vec2::new(px_size.x / PIXELS_PER_METER, px_size.y / PIXELS_PER_METER);    
        
        commands.spawn((
            Anton(anton.clone()),
            Transform::from_xyz(0.0, size.y / 2.0, 0.0),
            BillboardTexture(image),
            BillboardMesh(meshes.add(Rectangle::from_size(size))),
            BillboardLockAxis { y_axis: true, rotation: false },
            Wandering::default()
        )).insert(match anton {
            _ => (),
        }).id()
    }

    pub fn spawn_random(commands: &mut Commands, assets: &Res<GameAssets>, images: &Res<Assets<Image>>, meshes: &mut ResMut<Assets<Mesh>>) -> Entity {
        let anton: AntonType = rand::random();
        Anton::spawn(anton, commands, assets, images, meshes)
    }
}

impl AntonType {
    pub fn get_image(anton: AntonType, assets: &Res<GameAssets>) -> Handle<Image> {
        match anton {
            // sorry
            AntonType::Furryton => assets.furryton.clone(),
            AntonType::BusDriver => assets.bus_driver.clone(),
            AntonType::CampingTon => assets.camping_ton.clone(),
            AntonType::PetRocketRacer => assets.pet_rocket_racer.clone(),
            AntonType::SmokingCatboyAntonFlippingYouOff => assets.smoking_catboy_anton_flipping_you_off.clone(),
            AntonType::CaffeineKing => assets.caffeine_king.clone(),
            AntonType::CtrlFU => assets.ctrl_f_u.clone(),
            AntonType::Mug => assets.mug.clone(),
            AntonType::ScarecrowArt => assets.scarecrow_art.clone(),
            AntonType::Rubton => assets.rubton.clone(),
            AntonType::HonkNetworker => assets.honk_networker.clone(),
            AntonType::HappyZergling => assets.happy_zergling.clone(),
            AntonType::SnailTrail => assets.snail_trail.clone(),
            AntonType::XtremeXplosiveFisher => assets.xtreme_xplosive_fisher.clone(),
            AntonType::Pentacat => assets.pentacat.clone(),
            AntonType::Greger => assets.greger.clone(),
            AntonType::Rover => assets.rover.clone(),
            AntonType::Warewolf => assets.warewolf.clone(),
            AntonType::Pernilla => assets.pernilla.clone(),
            AntonType::VanillaIcecream => assets.vanilla_icecream.clone(),
            AntonType::Molerat => assets.molerat.clone(),
            AntonType::FridayFrogger => assets.friday_frogger.clone(),
            AntonType::DailyAnton16 => assets.daily_anton_16.clone(),
        }
    }
}

impl Distribution<AntonType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AntonType {
        match rng.random_range(0..23) { // Has to be updated for every anton...
            0 => AntonType::Furryton,
            1 => AntonType::BusDriver,
            2 => AntonType::CampingTon,
            3 => AntonType::PetRocketRacer,
            4 => AntonType::SmokingCatboyAntonFlippingYouOff,
            5 => AntonType::CaffeineKing,
            6 => AntonType::CtrlFU,
            7 => AntonType::Mug,
            8 => AntonType::ScarecrowArt,
            9 => AntonType::Rubton,
            10 => AntonType::HonkNetworker,
            11 => AntonType::HappyZergling,
            12 => AntonType::SnailTrail,
            13 => AntonType::XtremeXplosiveFisher,
            14 => AntonType::Pentacat,
            15 => AntonType::Greger,
            16 => AntonType::Rover,
            17 => AntonType::Warewolf,
            18 => AntonType::Pernilla,
            19 => AntonType::VanillaIcecream,
            20 => AntonType::Molerat,
            21 => AntonType::FridayFrogger,
            22 => AntonType::DailyAnton16,
            
            i => panic!("Invalid random anton number {i}")
        }
    }
}