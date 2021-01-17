//! Draw 2D shapes in Bevy.
//!
//! This crate provides a Bevy [plugin] to draw shapes with minimum boilerplate.
//! Some shapes are provided for convenience, however you can extend the
//! functionality of this crate by implementing the
//! [`ShapeSprite`](plugin::ShapeSprite) trait by your own.
//!
//! ## Usage
//! Check out the `README.md` on the [**GitHub repository**](https://github.com/Nilirad/bevy_prototype_lyon)
//! or run the [examples](https://github.com/Nilirad/bevy_prototype_lyon/tree/master/examples).

// rustc
#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, unused)]
#![allow(elided_lifetimes_in_paths)]
// clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use bevy_render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};
use lyon_tessellation::{
    FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
};

pub mod conversions;
pub mod path;
pub mod plugin;
pub mod shapes;

/// Import this module as `use bevy_prototype_lyon::prelude::*` to get
/// convenient imports.
pub mod prelude {
    pub use crate::{
        path::PathBuilder,
        plugin::{draw_path, Multishape, ShapePlugin, ShapeSprite, TessellationMode, Tessellator},
        shapes,
    };
    pub use lyon_tessellation::{
        path::path::Builder, FillOptions, FillRule, LineCap, LineJoin, Orientation, StrokeOptions,
    };
}

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

/// Zero-sized type used to implement various vertex construction traits from
/// Lyon.
struct VertexConstructor;

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0],
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0],
        }
    }
}

/// The index type of a Bevy [`Mesh`].
pub type IndexType = u32;

/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
pub type Buffers = VertexBuffers<Vertex, IndexType>;

/// Builds a Bevy [`Mesh`] from a [`Buffers`] type.
fn build_mesh(buffers: &Buffers) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(buffers.indices.clone())));
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|v| v.position)
            .collect::<Vec<[f32; 3]>>(),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        buffers
            .vertices
            .iter()
            .map(|v| v.normal)
            .collect::<Vec<[f32; 3]>>(),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        buffers
            .vertices
            .iter()
            .map(|v| v.uv)
            .collect::<Vec<[f32; 2]>>(),
    );

    mesh
}
