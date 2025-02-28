use crate::ui::jointext::SpawnJoinText;
use bevy::prelude::*;
use bevy_mod_billboard::BillboardTexture;
use chrono::{Datelike, Local, Weekday};
use crate::misc::play_sound_event::PlaySoundEvent;
use crate::ui::factorio_joke::SpawnFactorioJoke;

use crate::GameAssets;

#[derive(Component, Hash, PartialEq, Eq, Debug, Clone, Copy)]
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

pub const SPAWNABLE_ANTONS: [AntonType; 30] = [
    AntonType::Furryton, AntonType::BusDriver, AntonType::CampingTon, AntonType::PetRocketRacer, 
    AntonType::SmokingCatboyAntonFlippingYouOff, AntonType::CaffeineKing, AntonType::CtrlFU, AntonType::Mug, 
    AntonType::ScarecrowArt, AntonType::Rubton, AntonType::HonkNetworker, AntonType::HappyZergling, AntonType::SnailTrail, 
    AntonType::XtremeXplosiveFisher, AntonType::Pentacat, AntonType::Greger, AntonType::Rover, AntonType::Warewolf, 
    AntonType::Pernilla, AntonType::VanillaIcecream, AntonType::Molerat, AntonType::FridayFrogger, AntonType::DailyAnton16, 
    AntonType::Fax, AntonType::GoobyTheHobo, AntonType::BabyWithAHammer, AntonType::Biter, AntonType::OneMore, AntonType::Cortana, AntonType::AntonParty
];

impl AntonType {
    /// Returns an Entity with components necessary for the unique Anton
    pub fn spawn(anton: Self, commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
        let entity = commands.spawn(anton).id();

        let mut message: Vec<String> = vec![];

        match anton {
            AntonType::Furryton => {
                message.push("Furryton has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.furryton.clone()));
            }
            AntonType::BusDriver => {
                message.push("Our designated anti-covid bus driver has arrived!".to_string());
                message.push("Atchoo!".to_string());
                message.push("The bus driver has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.bus_driver.clone()));
            }
            AntonType::CampingTon => {
                message.push("It's Nils!".to_string());
                message.push("It's Camping Carl no way".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.camping_ton.clone()));
            }
            AntonType::PetRocketRacer => {
                message.push("Ready to do some pet rocket racing?".to_string());
                message.push("A pet rocket racer has joined the party!".to_string());
                message.push("Pet Rocket Rock Rock Rocket Racing".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.pet_rocket_racer.clone()));
            }
            AntonType::SmokingCatboyAntonFlippingYouOff => {
                message.push("what the fuck".to_string());
                message.push("Smoking catboy Anton flipping you off has joined the party!".to_string());
                commands.entity(entity).insert(BillboardTexture(
                    assets.smoking_catboy_anton_flipping_you_off.clone(),
                ));
            }
            AntonType::CaffeineKing => {
                message.push("Caffeine king has decided to stay awake just one more second!".to_string());
                message.push("Caffeine king has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.caffeine_king.clone()));
            }
            AntonType::CtrlFU => {
                message.push("Ctrl + F + U".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.ctrl_f_u.clone()));
            }
            AntonType::Mug => {
                message.push("What a stupid bird!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.mug.clone()));
            }
            AntonType::ScarecrowArt => {
                message.push("Guys we're releasing a new game we're gonna call it anton".to_string());
                message.push("This truly is a Scarecrow Art".to_string());
                message.push("Scarecrow Arts has sponsored the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.scarecrow_art.clone()));
            }
            AntonType::Rubton => {
                message.push("No way new RWBY episode??".to_string());
                message.push("This is canon now.".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.rubton.clone()));
            }
            AntonType::HonkNetworker => {
                message.push("HONK!".to_string());
                commands.trigger(PlaySoundEvent(assets.sfx_honk.clone()));
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.honk_networker.clone()));
            }
            AntonType::HappyZergling => {
                message.push("The happy Zergling has happily joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.happy_zergling.clone()));
            }
            AntonType::SnailTrail => {
                message.push("The snail trail itself has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.snail_trail.clone()));
            }
            AntonType::XtremeXplosiveFisher => {
                message.push("The most sane fisher has joined the party!".to_string());
                message.push("Fish? Fish X-plosions? Erm, I think so.".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.xtreme_xplosive_fisher.clone()));
            }
            AntonType::Pentacat => {
                message.push("Pentacat has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.pentacat.clone()));
            }
            AntonType::Greger => {
                // I feel like there's a lot of potential to use greger as a madlib word in random quotes from things anton likes here
                message.push("When you first saw Greger, were you blinded by his majesty?".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.greger.clone()));
            }
            AntonType::Rover => {
                message.push("Rover has joined the party all the way from Mars!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.rover.clone()));
            }
            AntonType::Warewolf => {
                message.push("AWOOOOO!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.warewolf.clone()));
            }
            AntonType::Pernilla => {
                message.push("Pernilla has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.pernilla.clone()));
            }
            AntonType::VanillaIcecream => {
                message.push("Vanilla ice-cream has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.vanilla_icecream.clone()));
            }
            AntonType::Molerat => {
                message.push("The latest Daily Naked Mole-Rat has joined the party!".to_string());
                message.push("Certified Naked Mole-Rat Moment".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.molerat.clone()));
            }
            AntonType::FridayFrogger => {
                let today = Local::now().weekday();
                if today == Weekday::Fri {
                    message.push("Det är fredag mina bekanta".to_string());
                }else{
                    message.push("Kväk!".to_string());
                    message.push("Is it Friday yet?".to_string());
                }

                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.friday_frogger.clone()));
            }
            AntonType::DailyAnton16 => {
                message.push("Daily Anton #16 has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.daily_anton_16.clone()));
            }
            AntonType::Fax => {
                message.push("It's Faxmashine!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.fax.clone()));
            }
            AntonType::GoobyTheHobo => {
                message.push("pls".to_string());
                message.push("gooby pls".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.gooby_hobo.clone()));
            }
            AntonType::BabyWithAHammer => {
                message.push("Where is the key located?".to_string());
                message.push("I want to destroy a tower.".to_string());
                message.push("A literal baby has joined and he's destroying your towers!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.baby_with_a_hammer.clone()));
            }
            AntonType::Biter => {
                // TODO: Create "play sound" event and play Factorio warning sound
                message.push("The biter has infilitrated the party!".to_string());

                commands.trigger(SpawnFactorioJoke);
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.biter.clone()));
            }
            AntonType::OneMore => {
                message.push("One more.".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.one_more.clone()));
            }
            AntonType::Cortana => {
                message.push("When you first saw Halo, were you blinded by its majesty?".to_string());
                message.push("Blinded? Paralyzed? Dumbstruck?".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.cortana.clone()));
            }
            AntonType::AntonParty => {
                message.push("Anton has joined the party!".to_string());
                commands
                    .entity(entity)
                    .insert(BillboardTexture(assets.anton_party.clone()));
            }
        }

        commands.trigger(SpawnJoinText(message));

        entity
    }

    pub fn spawn_random_subset(antons: &Vec<AntonType>, commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
        Self::spawn(antons[rand::random_range(0..antons.len())], commands, assets)
    }
}