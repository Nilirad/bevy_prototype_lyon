//! Types for defining and using geometries.

use bevy::transform::components::Transform;
use lyon_tessellation::{self as tess, path::path::Builder};

use crate::{
    draw::DrawMode,
    entity::{Path, ShapeBundle},
};

/// Structs that implement this trait can be drawn as a shape. See the
/// [`shapes`](crate::shapes) module for some examples.
///
/// # Implementation example
///
/// ```
/// use bevy_prototype_lyon::geometry::Geometry;
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
/// // Finally, implement the `add_geometry` method.
/// impl Geometry for Rectangle {
///     fn add_geometry(&self, b: &mut Builder) {
///         b.add_rectangle(
///             &Rect::new(Point::zero(), Size::new(self.width, self.height)),
///             Winding::Positive,
///         );
///     }
/// }
/// ```
pub trait Geometry {
    /// Adds the geometry of the shape to the given Lyon path `Builder`.
    fn add_geometry(&self, b: &mut Builder);
}

/// This implementation permits to use a Lyon [`Path`] as a [`Geometry`].
impl Geometry for tess::path::Path {
    fn add_geometry(&self, b: &mut Builder) {
        b.concatenate(&[self.as_slice()]);
    }
}

/// Allows the creation of shapes using geometries added to a path builder.
pub struct GeometryBuilder(Builder);

impl GeometryBuilder {
    /// Creates a new, empty `GeometryBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self(Builder::new())
    }

    /// Adds a geometry to the path builder.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_prototype_lyon::prelude::*;
    /// #
    /// fn my_system(mut commands: Commands) {
    ///     let line = shapes::Line(Vec2::ZERO, Vec2::new(10.0, 0.0));
    ///     let square = shapes::Rectangle {
    ///         extents: Vec2::splat(100.0),
    ///         ..shapes::Rectangle::default()
    ///     };
    ///     let mut builder = GeometryBuilder::new().add(&line).add(&square);
    ///
    ///     commands.spawn_bundle(builder.build(
    ///         DrawMode::Outlined {
    ///             fill_mode: FillMode::color(Color::ORANGE_RED),
    ///             outline_mode: StrokeMode::new(Color::ORANGE_RED, 10.0),
    ///         },
    ///         Transform::default(),
    ///     ));
    /// }
    /// # my_system.system();
    /// ```
    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn add(mut self, shape: &impl Geometry) -> Self {
        shape.add_geometry(&mut self.0);
        self
    }

    /// Returns a [`ShapeBundle`] using the data contained in the path
    /// builder.
    #[must_use]
    pub fn build(self, mode: DrawMode, transform: Transform) -> ShapeBundle {
        ShapeBundle {
            path: Path(self.0.build()),
            mode,
            mesh2d: bevy::sprite::MaterialMesh2dBundle {
                transform,
                ..Default::default()
            },
            ..ShapeBundle::default()
        }
    }

    /// Returns a [`ShapeBundle`] with only one geometry.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_prototype_lyon::prelude::*;
    /// #
    /// fn my_system(mut commands: Commands) {
    ///     let line = shapes::Line(Vec2::ZERO, Vec2::new(10.0, 0.0));
    ///     commands.spawn_bundle(GeometryBuilder::build_as(
    ///         &line,
    ///         DrawMode::Fill(FillMode::color(Color::ORANGE_RED)),
    ///         Transform::default(),
    ///     ));
    /// }
    /// # my_system.system();
    /// ```
    pub fn build_as(shape: &impl Geometry, mode: DrawMode, transform: Transform) -> ShapeBundle {
        Self::new().add(shape).build(mode, transform)
    }
}

impl Default for GeometryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
