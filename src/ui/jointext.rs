use std::time::Duration;

use bevy::
    prelude::*
;
use bevy_easings::{Ease, EasingComponent, EasingType};
use rand::prelude::*;

use crate::GameAssets;

#[derive(Component)]
pub struct ToDespawn;

#[derive(Event)]
pub struct SpawnJoinText(pub Vec<String>);

pub fn spawn_join_text(
    trigger: Trigger<SpawnJoinText>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let vec = &trigger.event().0;
    if let Some(str) = vec.choose(&mut rand::rng()) {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    bottom: Val::Percent(10.),
                    right: Val::Percent(50.),
                    overflow: Overflow::visible(),
                    max_width: Val::Px(0.),
                    ..default()
                }
                .ease_to(
                    Node {
                        position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    bottom: Val::Percent(20.),
                    right: Val::Percent(50.),
                    overflow: Overflow::visible(),
                    max_width: Val::Px(0.),
                        ..default()
                    },
                    bevy_easings::EaseFunction::CubicOut,
                    EasingType::Once {
                        duration: Duration::from_millis(3000),
                    },
                ).with_original_value(),
                ToDespawn,
            ))
            .with_child((
                Node { ..default() },
                Text::new(str),
                TextFont {
                    font: assets.font_fuzzybubbles.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
            ));
    }
}

pub fn update(
    mut commands: Commands,
    mut finished_easing: RemovedComponents<EasingComponent<Node>>,
) {
    for finished in finished_easing.read() {
        commands.entity(finished).despawn_recursive();
    }
}
