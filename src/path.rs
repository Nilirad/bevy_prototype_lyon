//! Interface to build custom paths.

use crate::conversions::Convert;
use bevy_math::Vec2;
use lyon_tessellation::{
    geom::Angle,
    path::{builder::WithSvg, path::Builder, EndpointId, Path},
};

/// A SVG-like path builder.
pub struct PathBuilder(WithSvg<Builder>);

impl PathBuilder {
    /// Returns a new, empty `PathBuilder`
    pub fn new() -> Self {
        Self(Builder::new().with_svg())
    }

    /// Returns a finalized [`Path`].
    pub fn build(self) -> Path {
        self.0.build()
    }

    /// Moves the current point to the given position.
    pub fn move_to(&mut self, to: Vec2) -> EndpointId {
        self.0.move_to(to.convert())
    }

    /// Adds to the path a line from the current position to the given one.
    pub fn line_to(&mut self, to: Vec2) -> EndpointId {
        self.0.line_to(to.convert())
    }

    /// Closes the shape, adding to the path a line from the current position to
    /// the starting location.
    pub fn close(&mut self) {
        self.0.close();
    }

    /// Adds a quadratic bezier to the path.
    pub fn quadratic_bezier_to(&mut self, ctrl: Vec2, to: Vec2) -> EndpointId {
        self.0.quadratic_bezier_to(ctrl.convert(), to.convert())
    }

    /// Adds a cubic bezier to the path.
    pub fn cubic_bezier_to(&mut self, ctrl1: Vec2, ctrl2: Vec2, to: Vec2) -> EndpointId {
        self.0
            .cubic_bezier_to(ctrl1.convert(), ctrl2.convert(), to.convert())
    }

    /// Adds an arc to the path.
    pub fn arc(&mut self, center: Vec2, radii: Vec2, sweep_angle: f32, x_rotation: f32) {
        self.0.arc(
            center.convert(),
            radii.convert(),
            Angle::radians(sweep_angle),
            Angle::radians(x_rotation),
        );
    }

    /// Returns the path's current position.
    pub fn current_position(&self) -> Vec2 {
        let p = self.0.current_position();
        Vec2::new(p.x, p.y)
    }
}
