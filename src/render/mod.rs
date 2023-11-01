//! Render plugin

use bevy::{
    asset::load_internal_asset,
    prelude::{App, Asset, AssetApp, Assets, Handle, Plugin, Shader},
    reflect::prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

/// Handle to the custom shader with a unique random ID
pub const SHAPE_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(3_191_283_017_262_752_456);

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

        app.add_plugins(Material2dPlugin::<ShapeMaterial>::default())
            .register_asset_reflect::<ShapeMaterial>();

        app.world
            .resource_mut::<Assets<ShapeMaterial>>()
            .insert(Handle::<ShapeMaterial>::default(), ShapeMaterial::default());
    }
}

impl Material2d for ShapeMaterial {
    fn fragment_shader() -> ShaderRef {
        SHAPE_MATERIAL_SHADER_HANDLE.into()
    }
}

/// A simple `Material2d` that renders with vertex colors.
#[derive(Asset, AsBindGroup, Reflect, Debug, Default, Clone)]
#[reflect(Default, Debug)]
pub struct ShapeMaterial {}
