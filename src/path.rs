//! Types outputting lyon `Path`s.

use bevy::math::Vec2;
use lyon_tessellation::{
    geom::Angle,
    path::{builder::WithSvg, path::BuilderImpl},
};

use crate::{
    prelude::Geometry,
    utils::{ToPoint, ToVector},
};

/// Describes an atomic action defining a [`Path`].
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    MoveTo(Vec2),
    LineTo(Vec2),
    QuadraticBezierTo {
        ctrl: Vec2,
        to: Vec2,
    },
    CubicBezierTo {
        ctrl1: Vec2,
        ctrl2: Vec2,
        to: Vec2,
    },
    Arc {
        center: Vec2,
        radii: Vec2,
        sweep_angle: f32,
        x_rotation: f32,
    },
    Close,
}

#[derive(Default, Clone, PartialEq, Debug)]
/// A custom, SVG-like `Path`.
pub struct ShapePath {
    actions: Vec<Action>,
}

impl ShapePath {
    /// Creates a new `Path`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Moves the current point to the given position.
    #[must_use]
    pub fn move_to(mut self, to: Vec2) -> Self {
        self.actions.push(Action::MoveTo(to));
        self
    }

    /// Adds to the path a line from the current position to the given one.
    #[must_use]
    pub fn line_to(mut self, to: Vec2) -> Self {
        self.actions.push(Action::LineTo(to));
        self
    }

    /// Adds a quadratic bezier to the path.
    #[must_use]
    pub fn quadratic_bezier_to(mut self, ctrl: Vec2, to: Vec2) -> Self {
        self.actions.push(Action::QuadraticBezierTo { ctrl, to });
        self
    }

    /// Adds a cubic bezier to the path.
    #[must_use]
    pub fn cubic_bezier_to(mut self, ctrl1: Vec2, ctrl2: Vec2, to: Vec2) -> Self {
        self.actions
            .push(Action::CubicBezierTo { ctrl1, ctrl2, to });
        self
    }

    /// Adds an arc to the path.
    #[must_use]
    pub fn arc(mut self, center: Vec2, radii: Vec2, sweep_angle: f32, x_rotation: f32) -> Self {
        self.actions.push(Action::Arc {
            center,
            radii,
            sweep_angle,
            x_rotation,
        });
        self
    }

    /// Closes the shape, adding to the path a line from the current position to
    /// the starting location.
    #[must_use]
    pub fn close(mut self) -> Self {
        self.actions.push(Action::Close);
        self
    }
}

impl Geometry<WithSvg<BuilderImpl>> for ShapePath {
    fn add_geometry(&self, b: &mut WithSvg<BuilderImpl>) {
        for action in &self.actions {
            match_action(*action, b);
        }
    }
}

fn match_action(action: Action, b: &mut WithSvg<BuilderImpl>) {
    match action {
        Action::MoveTo(to) => {
            b.move_to(to.to_point());
        }
        Action::LineTo(to) => {
            b.line_to(to.to_point());
        }
        Action::QuadraticBezierTo { ctrl, to } => {
            b.quadratic_bezier_to(ctrl.to_point(), to.to_point());
        }
        Action::CubicBezierTo { ctrl1, ctrl2, to } => {
            b.cubic_bezier_to(ctrl1.to_point(), ctrl2.to_point(), to.to_point());
        }
        Action::Arc {
            center,
            radii,
            sweep_angle,
            x_rotation,
        } => b.arc(
            center.to_point(),
            radii.to_vector(),
            Angle::radians(sweep_angle),
            Angle::radians(x_rotation),
        ),

        Action::Close => {
            b.close();
        }
    };
}
