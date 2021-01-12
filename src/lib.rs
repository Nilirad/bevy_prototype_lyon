//! Draw 2D shapes in Bevy.
//!
//! This crate provides a Bevy [plugin] to draw shapes with minimum boilerplate.
//! Some shapes are provided for convenience, however you can extend the
//! functionality of this crate by implementing the [`ShapeSprite`] trait by
//! your own.
//!
//! ## Usage
//! Check out the `README.md` on the [**GitHub repository**](https://github.com/Nilirad/bevy_prototype_lyon)
//! or run the [examples](https://github.com/Nilirad/bevy_prototype_lyon/tree/master/examples).

use bevy::{
    asset::Handle,
    render::{
        mesh::{Indices, Mesh},
        pipeline::PrimitiveTopology,
    },
    sprite::ColorMaterial,
    transform::components::Transform,
};
use lyon_tessellation::{
    path::path::Path, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
    StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
};
use plugin::ShapeDescriptor;

pub mod conversions;
pub mod plugin;
pub mod shapes;

/// Import this module as `use bevy_prototype_lyon::prelude::*` to get
/// convenient imports.
pub mod prelude {
    pub use crate::{plugin::ShapePlugin, shapes, ShapeSprite, TessellationMode, Tessellator};
    pub use lyon_tessellation::{
        FillOptions, FillRule, LineCap, LineJoin, Orientation, StrokeOptions,
    };
}

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// `Mesh`.
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

/// The index type of a Bevy `Mesh`
pub type IndexType = u32;

/// Lyon's `VertexBuffers` generic data type defined for [`Vertex`].
pub type Buffers = VertexBuffers<Vertex, IndexType>;

/// Builds a Bevy `Mesh` from a [`Buffers`] type.
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

// TODO: Move to plugin module
/// Determines if a shape must be filled or stroked.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TessellationMode {
    Fill(FillOptions),
    Stroke(StrokeOptions),
}

// TODO: Move to plugin module
/// A couple of `lyon` fill and stroke tessellators.
pub struct Tessellator {
    pub fill: FillTessellator,
    pub stroke: StrokeTessellator,
}

// TODO: Move to plugin module
impl Tessellator {
    /// Creates a new `Tessellator` data structure, containing the two types of
    /// Lyon tessellator.
    pub fn new() -> Self {
        Self {
            fill: FillTessellator::new(),
            stroke: StrokeTessellator::new(),
        }
    }
}

/// Shape structs that implement this trait can be transformed into a
/// [`SpriteBundle`](bevy::sprite::entity::SpriteBundle). See the [`shapes`]
/// module for some examples.
///
/// # Implementation example
///
/// ```
/// use bevy_prototype_lyon::ShapeSprite;
/// use lyon_tessellation::{
///     math::{Point, Rect, Size},
///     path::{path::Builder, traits::PathBuilder, Path, Winding},
/// };
///
/// // First, create a struct to hold the shape features:
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// pub struct Rectangle {
///     pub width: f32,
///     pub height: f32,
/// }
///
/// // Implementing the `Default` trait is not required, but it may facilitate the
/// // definition of the shape before spawning it.
/// impl Default for Rectangle {
///     fn default() -> Self {
///         Self {
///             width: 1.0,
///             height: 1.0,
///         }
///     }
/// }
///
/// // Finally, implement the `generate_path` method.
/// impl ShapeSprite for Rectangle {
///     fn generate_path(&self) -> Path {
///         let mut path_builder = Builder::new();
///         path_builder.add_rectangle(
///             &Rect::new(Point::zero(), Size::new(self.width, self.height)),
///             Winding::Positive,
///         );
///         path_builder.build()
///     }
/// }
/// ```
pub trait ShapeSprite {
    /// Generates a Lyon `Path` for the shape.
    fn generate_path(&self) -> Path;

    /// Returns a [`ShapeDescriptor`](plugin::ShapeDescriptor) entity for the
    /// shape. If spawned into the [`World`](bevy::ecs::World) during the
    /// [`UPDATE`](bevy::app::stage::UPDATE) stage, it will be replaced by a
    /// custom [`SpriteBundle`](bevy::sprite::entity::SpriteBundle)
    /// corresponding to the shape.
    fn draw(
        &self,
        material: Handle<ColorMaterial>,
        mode: TessellationMode,
        transform: Transform,
    ) -> (ShapeDescriptor,)
    where
        Self: Sync + Send + Sized + Clone + 'static,
    {
        let desc = ShapeDescriptor {
            shape: Box::new(self.clone()),
            material: material.clone(),
            mode,
            transform,
        };

        (desc,)
    }
}
