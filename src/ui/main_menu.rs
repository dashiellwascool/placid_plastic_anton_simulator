use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::GameAssets;

#[derive(Component)]
pub struct MainMenu;

pub fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        MainMenu,
        Node {
            position_type: PositionType::Relative,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            overflow: Overflow::visible(),
            ..default()
        },
    )).with_child((
        Text::new("Placid Plastic Anton Simualtor\n\nPress Space to Play"),
        TextFont {
            font: assets.font_fuzzybubbles.clone(),
            font_size: 50.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
    ));
}

pub fn do_main_menu(main_menu_query: Query<Entity, With<MainMenu>>, mut camera_query: Query<&mut PanOrbitCamera>, mut commands: Commands, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    if !main_menu_query.is_empty() {
        let mut camera = camera_query.single_mut();
        camera.target_yaw = camera.target_yaw + 0.1 * time.delta_secs();
        if keys.pressed(KeyCode::Space) {
            commands.entity(main_menu_query.single()).despawn_recursive();
        }
    } else {
        camera_query.single_mut().enabled = true;
    }
}
