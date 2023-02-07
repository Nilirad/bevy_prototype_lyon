//! Render plugin

use bevy::{
    asset::load_internal_asset,
    prelude::{AddAsset, App, Assets, Handle, HandleUntyped, Plugin, Shader},
    reflect::{prelude::*, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

/// Handle to the custom shader with a unique random ID
pub const SHAPE_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3_191_283_017_262_752_456);

/// Plugin that provides a custom material for rendering [`Shape`]s
pub struct ShapeMaterialPlugin;

impl Plugin for ShapeMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SHAPE_MATERIAL_SHADER_HANDLE,
            "shape_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(Material2dPlugin::<ShapeMaterial>::default())
            .register_asset_reflect::<ShapeMaterial>();

        app.world
            .resource_mut::<Assets<ShapeMaterial>>()
            .set_untracked(Handle::<ShapeMaterial>::default(), ShapeMaterial::default());
    }
}

impl Material2d for ShapeMaterial {
    fn fragment_shader() -> ShaderRef {
        SHAPE_MATERIAL_SHADER_HANDLE.typed().into()
    }
}

/// A simple `Material2d` that renders with vertex colors.
#[derive(Default, AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[reflect(Default, Debug)]
#[uuid = "ab2e068e-0cca-4941-a114-524af2c431bb"]
pub struct ShapeMaterial {}
