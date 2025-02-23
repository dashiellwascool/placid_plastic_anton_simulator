use bevy::{prelude::*, render::render_resource::Face};

use crate::{GameAssets, GameState};

pub struct ApartmentPlugin;
impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
    mut ambient_light: ResMut<AmbientLight>
) {
    // lights
    ambient_light.brightness = 500.;

    // apartment
    let asset = gltf_assets.get(&assets.apartment).unwrap();

    for material_handle in &asset.materials {
        let material = materials.get_mut(material_handle).unwrap();
        material.double_sided = true;
        material.cull_mode = Some(Face::Back);
    }

    let corner_mat = materials.get_mut(asset.named_materials.get("CornerLineMat").unwrap()).unwrap();
    corner_mat.alpha_mode = AlphaMode::Blend;
    corner_mat.unlit = true;
    
    let scene = asset.scenes[0].clone();

    commands.spawn(
        SceneRoot(scene)
    );
}
