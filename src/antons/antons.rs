use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

use crate::anton::Anton;
use crate::GameAssets;

#[derive(Component)]
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

    // TODO: fn get_random()
}
