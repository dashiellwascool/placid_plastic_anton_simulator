use std::time::Duration;

use bevy::
    prelude::*
;
use rand::prelude::*;

use crate::GameAssets;

#[derive(Event)]
pub struct SpawnJoinText(pub Vec<String>);

pub fn spawn_join_text(
    trigger: Trigger<SpawnJoinText>,
    mut query: Query<(&mut Text, &mut JoinText)>,
) {
    let vec = &trigger.event().0;
    if let Some(str) = vec.choose(&mut rand::rng()) {
        if let Ok((mut text, mut timer)) = query.get_single_mut() {
            **text = str.to_string();
            timer.0.set_duration(Duration::from_millis(3000));
            timer.0.reset();
        }
    }
}

#[derive(Component)]
pub struct JoinText(Timer);

pub fn setup(mut commands: Commands, assets: Res<GameAssets>) {
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
            },
        ))
        .with_child((
            Text::new("Anton joined the party!"),
            TextFont {
                font: assets.font_fuzzybubbles.clone(),
                font_size: 32.0,
                ..default()
            },
            JoinText(Timer::new(Duration::ZERO, TimerMode::Once)),
            Visibility::Hidden,
            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
        ));
}

pub fn update(mut query: Query<(&mut Visibility, &mut JoinText)>, time: Res<Time>) {
    if let Ok((mut visibility, mut timer)) = query.get_single_mut() {
        timer.0.tick(time.delta());
        *visibility = match timer.0.finished() {
            true => Visibility::Hidden,
            false => Visibility::Visible,
        };

    }
}
