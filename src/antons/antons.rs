use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

use crate::anton::Anton;
use crate::GameAssets;

#[derive(Component, Debug)]
pub enum Antons {
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

impl Antons {
    pub fn spawn(anton: Antons, commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
        let mut entity = commands.spawn_empty();

        match anton {
            Antons::Furryton => {
                entity.insert((Anton, BillboardTexture(assets.furryton.clone())));
            }
            _ => todo!("Anton not added!"),
        }

        entity.id()
    }

    fn spawn_random(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
        let anton: Antons = rand::random();
        Antons::spawn(anton, commands, assets)

    }
}

impl Distribution<Antons> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Antons {
        match rng.random_range(0..22) { // Has to be updated for every anton...
            _ => Antons::Furryton
        }
    }
}