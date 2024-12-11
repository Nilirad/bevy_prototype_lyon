//! Types outputting lyon `Path`s.

use bevy::math::Vec2;
use lyon_tessellation::{
    geom::Angle,
    path::{
        builder::WithSvg,
        path::{Builder, BuilderImpl},
    },
};

use crate::{
    draw::{Fill, Stroke},
    entity::Shape,
    utils::{ToPoint, ToVector},
};

/// A SVG-like path builder.
pub struct PathBuilder {
    builder: WithSvg<BuilderImpl>,
    fill: Option<Fill>,
    stroke: Option<Stroke>,
}

impl PathBuilder {
    /// Returns a new, empty `PathBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            builder: Builder::new().with_svg(),
            fill: None,
            stroke: None,
        }
    }

    /// Returns a finalized [`Shape`].
    #[must_use]
    pub fn build(self) -> Shape {
        Shape::new(self.builder.build(), self.fill, self.stroke)
    }

    /// Sets the fill mode of the path.
    #[must_use]
    pub fn fill(self, fill: impl Into<Fill>) -> Self {
        Self {
            fill: Some(fill.into()),
            ..self
        }
    }

    /// Sets the stroke mode of the path.
    #[must_use]
    pub fn stroke(self, stroke: impl Into<Stroke>) -> Self {
        Self {
            stroke: Some(stroke.into()),
            ..self
        }
    }

    /// Moves the current point to the given position.
    #[must_use]
    pub fn move_to(mut self, to: Vec2) -> Self {
        self.builder.move_to(to.to_point());
        self
    }

    /// Adds to the path a line from the current position to the given one.
    #[must_use]
    pub fn line_to(mut self, to: Vec2) -> Self {
        self.builder.line_to(to.to_point());
        self
    }

    /// Closes the shape, adding to the path a line from the current position to
    /// the starting location.
    #[must_use]
    pub fn close(mut self) -> Self {
        self.builder.close();
        self
    }

    /// Adds a quadratic bezier to the path.
    #[must_use]
    pub fn quadratic_bezier_to(mut self, ctrl: Vec2, to: Vec2) -> Self {
        self.builder
            .quadratic_bezier_to(ctrl.to_point(), to.to_point());
        self
    }

    /// Adds a cubic bezier to the path.
    #[must_use]
    pub fn cubic_bezier_to(mut self, ctrl1: Vec2, ctrl2: Vec2, to: Vec2) -> Self {
        self.builder
            .cubic_bezier_to(ctrl1.to_point(), ctrl2.to_point(), to.to_point());
        self
    }

    /// Adds an arc to the path.
    #[must_use]
    pub fn arc(mut self, center: Vec2, radii: Vec2, sweep_angle: f32, x_rotation: f32) -> Self {
        self.builder.arc(
            center.to_point(),
            radii.to_vector(),
            Angle::radians(sweep_angle),
            Angle::radians(x_rotation),
        );
        self
    }

    /// Returns the path's current position.
    #[allow(clippy::must_use_candidate)]
    pub fn current_position(&self) -> Vec2 {
        let p = self.builder.current_position();
        Vec2::new(p.x, p.y)
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}
