use bevy::{
    animation::{animated_field, AnimationTargetId},
    prelude::*,
};
use rand::prelude::*;

use crate::GameAssets;

#[derive(Event)]
pub struct SpawnJoinText(pub Vec<String>);

pub fn spawn_join_text(
    trigger: Trigger<SpawnJoinText>,
    mut query_parent: Query<(&mut Transform, &Children), With<JoinTextParent>>,
    mut query_child: Query<&mut Text>,
) {
    let vec = &trigger.event().0;
    if let Some(str) = vec.choose(&mut rand::rng()) {
        if let Ok((mut transform, children)) = query_parent.get_single_mut() {
            for &child in children.iter() {
                if let Ok(mut text) = query_child.get_mut(child) {
                    **text = str.to_string();
                }
            }
        }
    }
}

#[derive(Component)]
pub struct JoinTextParent;

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
            JoinTextParent,
            //Visibility::Hidden,
        ))
        .with_child((
            Text::new("Anton joined the party!"),
            TextFont {
                font: assets.font_fuzzybubbles.clone(),
                font_size: 32.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
        ));
}

pub fn update() {
    /* for (mut transform, children) in query_parent.iter_mut() {
        for text in query_child.iter_mut() {
            **text = "";
        }
    } */
}
