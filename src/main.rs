use bevy::prelude::*;
use placid_plastic_anton_simulator::PlacidPlasticAntonSimulatorPlugin;

fn main() {
    App::new()
        .add_plugins(PlacidPlasticAntonSimulatorPlugin)
        .run();
}
