use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.35, 0.48, 0.66)));
    commands.spawn(
        PanOrbitCamera {
            pitch: Some(0.75),
            radius: Some(7.0),
            enabled: false,
            ..default()
        }
    );
}
