use bevy::prelude::*;

use crate::GameAssets;

#[derive(Component)]
pub struct TimerText(pub String);

pub fn setup(mut commands: Commands, assets: Res<GameAssets>) {

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            top: Val::Px(0.),
            right: Val::Percent(50.),
            overflow: Overflow::visible(),
            max_width: Val::Px(0.),
            ..default()
        },))
        .with_child((
            Text::new(""),
            TimerText(String::new()),
            TextFont {
                font: assets.font_fuzzybubbles.clone(),
                font_size: 24.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
        ));
}

pub fn update(mut query: Query<(&mut Text, &TimerText), Changed<TimerText>>) {
    for (mut text, timer) in &mut query {
        text.0 = timer.0.clone();
    }
}
