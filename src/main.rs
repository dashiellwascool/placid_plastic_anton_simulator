use std::env;

use bevy::prelude::*;
use placid_plastic_anton_simulator::PlacidPlasticAntonSimulatorPlugin;

fn main() {
    println!("Placid Plastic Anton Simulator v{}", env!("CARGO_PKG_VERSION"));
    App::new()
        .add_plugins(PlacidPlasticAntonSimulatorPlugin)
        .run();
}
