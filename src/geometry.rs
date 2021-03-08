//! Types for defining and using geometries.

use bevy::transform::components::Transform;
use lyon_tessellation::path::{path::Builder, Path};

use crate::{entity::ShapeBundle, prelude::ShapeColors, utils::DrawMode};

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
    /// Adds the geometry of the shape to the given Lyon path [`Builder`].
    fn add_geometry(&self, b: &mut Builder);
}

/// This implementation permits to use a Lyon [`Path`] as a [`Geometry`].
impl Geometry for Path {
    fn add_geometry(&self, b: &mut Builder) {
        b.concatenate(&[self.as_slice()])
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
    /// use bevy::prelude::*;
    /// use bevy_prototype_lyon::prelude::*;
    ///
    /// fn some_system(commands: &mut Commands) {
    ///     let line = shapes::Line(Vec2::ZERO, Vec2::new(10.0, 0.0));
    ///     let rectangle = shapes::Rectangle {
    ///         width: 100.0,
    ///         height: 50.0,
    ///         ..shapes::Rectangle::default()
    ///     };
    ///     let mut builder = GeometryBuilder::new();
    ///     builder.add(&line).add(&rectangle);
    ///
    ///     commands.spawn(builder.build(
    ///         ShapeColors::new(Color::ORANGE_RED),
    ///         DrawMode::Fill(FillOptions::default()),
    ///         Transform::default(),
    ///     ));
    /// }
    /// ```
    pub fn add(&mut self, shape: &impl Geometry) -> &mut Self {
        shape.add_geometry(&mut self.0);

        self
    }

    /// Generates a [`ShapeBundle`] using the data contained in the path
    /// builder.
    #[must_use]
    pub fn build(self, colors: ShapeColors, mode: DrawMode, transform: Transform) -> ShapeBundle {
        ShapeBundle {
            path: self.0.build(),
            colors,
            mode,
            transform,
            ..ShapeBundle::default()
        }
    }

    /// Generates a [`ShapeBundle`] with only one geometry.
    /// Adds a geometry to the path builder.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_prototype_lyon::prelude::*;
    ///
    /// fn some_system(commands: &mut Commands) {
    ///     let line = shapes::Line(Vec2::ZERO, Vec2::new(10.0, 0.0));
    ///     commands.spawn(GeometryBuilder::build_as(
    ///         &line,
    ///         ShapeColors::new(Color::ORANGE_RED),
    ///         DrawMode::Fill(FillOptions::default()),
    ///         Transform::default(),
    ///     ));
    /// }
    /// ```
    pub fn build_as(
        shape: &impl Geometry,
        colors: ShapeColors,
        mode: DrawMode,
        transform: Transform,
    ) -> ShapeBundle {
        let mut multishape = Self::new();
        multishape.add(shape);
        multishape.build(colors, mode, transform)
    }
}

impl Default for GeometryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
