//! Render graph settings.

use bevy::{
    asset::{Assets, HandleUntyped},
    prelude::World,
    reflect::TypeUuid,
    render::{
        pipeline::{
            BlendComponent, BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite,
            CompareFunction, DepthBiasState, DepthStencilState, Face, FrontFace,
            PipelineDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, StencilFaceState,
            StencilState,
        },
        shader::{Shader, ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
};

#[allow(missing_docs, clippy::unreadable_literal)]
pub const SHAPE_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x35ae6bb2984f7f14);

#[allow(missing_docs, clippy::unreadable_literal)]
pub const UI_SHAPE_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x158745fbe65cbe34);

#[allow(clippy::too_many_lines)]
fn build_shape_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Cw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
        },
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
        }),
        color_target_states: vec![ColorTargetState {
            format: TextureFormat::default(),
            blend: Some(BlendState {
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
            }),
            write_mask: ColorWrite::ALL,
        }],
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                if cfg!(target_arch = "wasm32") {
                    include_str!("shape.es.vert")
                } else {
                    include_str!("shape.vert")
                },
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                if cfg!(target_arch = "wasm32") {
                    include_str!("shape.es.frag")
                } else {
                    include_str!("shape.frag")
                },
            ))),
        })
    }
}

pub(crate) fn add_shape_pipeline(world: &mut World) {
    let world = world.cell();
    let mut pipelines = world
        .get_resource_mut::<Assets<PipelineDescriptor>>()
        .unwrap();
    let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
    pipelines.set_untracked(SHAPE_PIPELINE_HANDLE, build_shape_pipeline(&mut shaders));
}

#[allow(clippy::clippy::too_many_lines)]
fn build_ui_shape_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
        }),
        color_target_states: vec![ColorTargetState {
            format: TextureFormat::default(),
            blend: Some(BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
            }),
            write_mask: ColorWrite::ALL,
        }],
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Cw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
        },
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("ui_shape.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("ui_shape.frag"),
            ))),
        })
    }
}

pub(crate) fn add_ui_shape_pipeline(world: &mut World) {
    let world = world.cell();
    let mut pipelines = world
        .get_resource_mut::<Assets<PipelineDescriptor>>()
        .unwrap();
    let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
    pipelines.set_untracked(
        UI_SHAPE_PIPELINE_HANDLE,
        build_ui_shape_pipeline(&mut shaders),
    );
}
