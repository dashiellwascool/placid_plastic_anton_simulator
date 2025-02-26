use std::time::Duration;

use bevy::prelude::*;

use crate::{misc::play_sound_event::PlaySoundEvent, GameAssets};

#[derive(Event)]
pub struct SpawnFactorioJoke;

#[derive(Resource, Default)]
pub struct FactorioJokeTimer {
    me: Option<Entity>,
    timer: Timer,
}

#[derive(Component)]
pub struct FactorioJokeMarker;

pub fn spawn_factorio_joke(trigger: Trigger<SpawnFactorioJoke>, mut commands: Commands, mut tell: ResMut<FactorioJokeTimer>, assets: Res<GameAssets>) {
    commands.trigger(PlaySoundEvent(assets.sfx_factorio_alert.clone()));

    tell.timer = Timer::new(Duration::from_secs(13), TimerMode::Once);

    match tell.me {
        Some(_) => (),
        None => {
            let joke = commands.spawn((
                FactorioJokeMarker,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(3.),
                    left: Val::Px(3.),
                    ..default()
                },
                ImageNode::new(assets.ui_danger_icon.clone()).with_color(Color::srgba(1., 1., 1., 1.))
            )).id();

            tell.me = Some(joke);
        },
    }
}

pub fn update_factorio_joke(mut res: ResMut<FactorioJokeTimer>, mut query: Query<(Entity, &mut ImageNode), With<FactorioJokeMarker>>, time: Res<Time>, mut commands: Commands) {
    res.timer.tick(time.delta());
    for (entity, mut image) in query.iter_mut() {

        let transparency = if (res.timer.elapsed_secs() as u32) % 2 == 0 { 1. } else {0.5};

        image.color = Color::srgba(1., 1., 1., transparency);

        if let Some(_) = res.me {
            if res.timer.finished() {
                res.me = None;
                commands.entity(entity).despawn();
    
            }
        }
    }

    
}