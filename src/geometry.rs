//! Types for defining and using geometries.

use lyon_algorithms::path::{builder::WithSvg, BuilderImpl};
use lyon_tessellation::path::path::Builder;

use crate::{
    draw::{Fill, Stroke},
    entity::Shape,
};

/// Structs that implement this trait can be drawn as a shape. See the
/// [`shapes`](crate::shapes) module for some examples.
///
/// # Implementation example
///
/// ```
/// use bevy_prototype_lyon::geometry::Geometry;
/// use lyon_tessellation::{
///     math::{Box2D, Point, Size},
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
/// impl Geometry<Builder> for Rectangle {
///     fn add_geometry(&self, b: &mut Builder) {
///         b.add_rectangle(
///             &Box2D::new(Point::zero(), Point::new(self.width, self.height)),
///             Winding::Positive,
///         );
///     }
/// }
/// ```
pub trait Geometry<GenericBuilder> {
    /// Adds the geometry of the shape to the given Lyon path `Builder`.
    fn add_geometry(&self, b: &mut GenericBuilder);
}

/// Provides basic functionality common
/// to [`ShapeBuilder`] and [`ReadyShapeBuilder`].
pub trait ShapeBuilderBase<GenericBuilder> {
    /// Adds a `Geometry` to the builder.
    #[must_use]
    fn add(self, shape: &impl Geometry<GenericBuilder>) -> Self;

    /// Sets the fill mode of the builder.
    #[must_use]
    fn fill(self, fill: impl Into<Fill>) -> ReadyShapeBuilder<GenericBuilder>;

    /// Sets the stroke mode of the builder.
    #[must_use]
    fn stroke(self, stroke: impl Into<Stroke>) -> ReadyShapeBuilder<GenericBuilder>;
}

/// Provides methods for building a shape.
#[derive(Default, Clone)]
pub struct ShapeBuilder<GenericBuilder>(GenericBuilder);

impl<GenericBuilder> ShapeBuilderBase<GenericBuilder> for ShapeBuilder<GenericBuilder> {
    fn add(mut self, shape: &impl Geometry<GenericBuilder>) -> Self {
        shape.add_geometry(&mut self.0);
        self
    }

    fn fill(self, fill: impl Into<Fill>) -> ReadyShapeBuilder<GenericBuilder> {
        ReadyShapeBuilder {
            builder: self.0,
            fill: Some(fill.into()),
            stroke: None,
        }
    }

    fn stroke(self, stroke: impl Into<Stroke>) -> ReadyShapeBuilder<GenericBuilder> {
        ReadyShapeBuilder {
            builder: self.0,
            fill: None,
            stroke: Some(stroke.into()),
        }
    }
}

impl<GenericBuilder> ShapeBuilder<GenericBuilder>
where
    GenericBuilder: LyonPathBuilderExt,
{
    /// Constructs a new `ShapeBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self(GenericBuilder::new())
    }

    /// Constructs a new `ShapeBuilder` with an initial `Geometry`.
    #[must_use]
    pub fn with(geometry: &impl Geometry<GenericBuilder>) -> Self {
        Self::new().add(geometry)
    }
}

/// Provides methods for building a shape.
///
/// This struct can only be obtained by using [`ShapeBuilder`].
#[derive(Clone)]
pub struct ReadyShapeBuilder<GenericBuilder> {
    pub(crate) builder: GenericBuilder,
    pub(crate) fill: Option<Fill>,
    pub(crate) stroke: Option<Stroke>,
}

impl<GenericBuilder> ShapeBuilderBase<GenericBuilder> for ReadyShapeBuilder<GenericBuilder> {
    fn add(mut self, shape: &impl Geometry<GenericBuilder>) -> Self {
        shape.add_geometry(&mut self.builder);
        self
    }

    fn fill(self, fill: impl Into<Fill>) -> Self {
        Self {
            fill: Some(fill.into()),
            ..self
        }
    }

    fn stroke(self, stroke: impl Into<Stroke>) -> Self {
        Self {
            stroke: Some(stroke.into()),
            ..self
        }
    }
}

/// Generalizes the implementation of the `build` method
/// over multiple Lyon builders.
pub trait ReadyShapeBuilderTrait {
    /// Builds a `Shape` according to the builder's settings.
    fn build(self) -> Shape;
}

impl ReadyShapeBuilderTrait for ReadyShapeBuilder<Builder> {
    fn build(self) -> Shape {
        Shape::new(self.builder.build(), self.fill, self.stroke)
    }
}

impl ReadyShapeBuilderTrait for ReadyShapeBuilder<WithSvg<BuilderImpl>> {
    fn build(self) -> Shape {
        Shape::new(self.builder.build(), self.fill, self.stroke)
    }
}

/// Extends `lyon` path builders with analogous methods
/// under the same interface.
pub trait LyonPathBuilderExt {
    /// Creates a new path builder.
    fn new() -> Self;
}

impl LyonPathBuilderExt for Builder {
    fn new() -> Self {
        Self::new()
    }
}

impl LyonPathBuilderExt for WithSvg<BuilderImpl> {
    fn new() -> Self {
        Builder::new().with_svg()
    }
}
