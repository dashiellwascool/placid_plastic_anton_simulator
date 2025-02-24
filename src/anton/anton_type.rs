use crate::ui::jointext::SpawnJoinText;
use bevy::prelude::*;
use bevy_mod_billboard::BillboardTexture;
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};

use crate::GameAssets;

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
    Fax,
    GoobyTheHobo,
    BabyWithAHammer,
    Biter,
    OneMore,
    Cortana,
    AntonParty,
}

impl AntonType {
    /// Returns an Entity with components necessary for the unique Anton
    pub fn spawn(anton: Self, commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
        let entity = commands.spawn_empty().id();

        match anton {
            AntonType::Furryton => {
                commands.trigger(SpawnJoinText(vec![
                    "Furryton has joined the party!".to_string()
                ]));
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.furryton.clone()));
            }
            AntonType::BusDriver => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.bus_driver.clone()));
            }
            AntonType::CampingTon => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.camping_ton.clone()));
            }
            AntonType::PetRocketRacer => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.pet_rocket_racer.clone()));
            }
            AntonType::SmokingCatboyAntonFlippingYouOff => {
                commands.entity(entity).insert(BillboardTexture(
                    assets.smoking_catboy_anton_flipping_you_off.clone(),
                ));
            }
            AntonType::CaffeineKing => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.caffeine_king.clone()));
            }
            AntonType::CtrlFU => {
                commands.trigger(SpawnJoinText(vec!["Ctrl + F + U".to_string()]));
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.ctrl_f_u.clone()));
            }
            AntonType::Mug => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.mug.clone()));
            }
            AntonType::ScarecrowArt => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.scarecrow_art.clone()));
            }
            AntonType::Rubton => {
                commands.trigger(SpawnJoinText(vec!["No way new RWBY episode??".to_string()]));
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.rubton.clone()));
            }
            AntonType::HonkNetworker => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.honk_networker.clone()));
            }
            AntonType::HappyZergling => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.happy_zergling.clone()));
            }
            AntonType::SnailTrail => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.snail_trail.clone()));
            }
            AntonType::XtremeXplosiveFisher => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.xtreme_xplosive_fisher.clone()));
            }
            AntonType::Pentacat => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.pentacat.clone()));
            }
            AntonType::Greger => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.greger.clone()));
            }
            AntonType::Rover => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.rover.clone()));
            }
            AntonType::Warewolf => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.warewolf.clone()));
            }
            AntonType::Pernilla => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.pernilla.clone()));
            }
            AntonType::VanillaIcecream => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.vanilla_icecream.clone()));
            }
            AntonType::Molerat => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.molerat.clone()));
            }
            AntonType::FridayFrogger => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.friday_frogger.clone()));
            }
            AntonType::DailyAnton16 => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.daily_anton_16.clone()));
            }
            AntonType::Fax => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.fax.clone()));
            }
            AntonType::GoobyTheHobo => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.gooby_hobo.clone()));
            }
            AntonType::BabyWithAHammer => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.baby_with_a_hammer.clone()));
            }
            AntonType::Biter => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.biter.clone()));
            }
            AntonType::OneMore => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.one_more.clone()));
            }
            AntonType::Cortana => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.cortana.clone()));
            }
            AntonType::AntonParty => {
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.anton_party.clone()));
            }
        }

        entity
    }

    /// Picks a random AntonType and returns the entity spawned
    pub fn spawn_random(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
        let anton: Self = rand::random();
        Self::spawn(anton, commands, assets)
    }
}

impl Distribution<AntonType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AntonType {
        match rng.random_range(0..=29) {
            // Has to be updated for every anton...
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
            23 => AntonType::Fax,
            24 => AntonType::GoobyTheHobo,
            25 => AntonType::BabyWithAHammer,
            26 => AntonType::Biter,
            27 => AntonType::OneMore,
            28 => AntonType::Cortana,
            29 => AntonType::AntonParty,

            i => panic!("Invalid random anton number {i}"),
        }
    }
}
