use bevy::{
    animation::{animated_field, AnimationTarget, AnimationTargetId},
    prelude::*,
    render::render_resource::Face,
};

use crate::{GameAssets, GameState};

pub struct ApartmentPlugin;
impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup, setup_door));
    }
}

#[derive(Component)]
pub struct Door;

fn setup(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    // lights
    ambient_light.brightness = 500.;

    commands.spawn((PointLight { ..default() }, Transform::from_xyz(0., 3., 0.)));

    // apartment
    let asset = gltf_assets.get(&assets.apartment).unwrap();

    for material_handle in &asset.materials {
        let material = materials.get_mut(material_handle).unwrap();
        material.double_sided = true;
        material.cull_mode = Some(Face::Back);
    }

    let corner_mat = materials
        .get_mut(asset.named_materials.get("CornerLineMat").unwrap())
        .unwrap();
    corner_mat.alpha_mode = AlphaMode::Blend;
    corner_mat.unlit = true;

    let scene = asset.scenes[0].clone();

    commands.spawn(SceneRoot(scene));
}

#[derive(Component)]
pub struct DoorAnimationNodeIndex(pub AnimationNodeIndex);

fn setup_door(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let door_pos = Transform::from_xyz(0.484516, 0., -0.49771);
    let door_asset = gltf_assets.get(&assets.door).unwrap();
    for material_handle in &door_asset.materials {
        let material = materials.get_mut(material_handle).unwrap();
        material.unlit = true;
        material.double_sided = true;
        material.cull_mode = Some(Face::Back);
    }

    let door = Name::new("door");
    let mut animation = AnimationClip::default();
    let door_animation_target_id = AnimationTargetId::from_name(&door);
    animation.add_curve_to_target(
        door_animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            UnevenSampleAutoCurve::new([0.0, 0.2, 0.6, 0.8].into_iter().zip([
                Quat::from_rotation_y(0.),
                Quat::from_rotation_y(f32::to_radians(-20.)),
                Quat::from_rotation_y(f32::to_radians(-20.)),
                Quat::from_rotation_y(0.),
            ]))
            .expect("This should work, damn it!"),
        ),
    );

    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    let player = AnimationPlayer::default();

    let door_entity = commands
        .spawn((
            SceneRoot(door_asset.scenes[0].clone()),
            door_pos,
            Door,
            player,
            door,
            AnimationGraphHandle(graphs.add(graph)),
            DoorAnimationNodeIndex(animation_index)
        ))
        .id();

    commands.entity(door_entity).insert(AnimationTarget {
        id: door_animation_target_id,
        player: door_entity
    });
}
