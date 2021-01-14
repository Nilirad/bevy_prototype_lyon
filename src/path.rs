use bevy::math::Vec2;
use lyon_tessellation::{
    geom::Angle,
    path::{builder::WithSvg, path::Builder, EndpointId, Path},
};

use crate::conversions::{ToLyonPoint, ToLyonVector};

pub struct PathBuilder(WithSvg<Builder>);

impl PathBuilder {
    pub fn new() -> Self {
        Self(Builder::new().with_svg())
    }

    pub fn build(self) -> Path {
        self.0.build()
    }

    pub fn move_to(&mut self, to: Vec2) -> EndpointId {
        self.0.move_to(to.to_lyon_point())
    }

    pub fn line_to(&mut self, to: Vec2) -> EndpointId {
        self.0.line_to(to.to_lyon_point())
    }

    pub fn close(&mut self) {
        self.0.close();
    }

    pub fn quadratic_bezier_to(&mut self, ctrl: Vec2, to: Vec2) -> EndpointId {
        self.0
            .quadratic_bezier_to(ctrl.to_lyon_point(), to.to_lyon_point())
    }

    pub fn cubic_bezier_to(&mut self, ctrl1: Vec2, ctrl2: Vec2, to: Vec2) -> EndpointId {
        self.0.cubic_bezier_to(
            ctrl1.to_lyon_point(),
            ctrl2.to_lyon_point(),
            to.to_lyon_point(),
        )
    }

    pub fn arc(&mut self, center: Vec2, radii: Vec2, sweep_angle: f32, x_rotation: f32) {
        self.0.arc(
            center.to_lyon_point(),
            radii.to_lyon_vector(),
            Angle::radians(sweep_angle),
            Angle::radians(x_rotation),
        );
    }

    pub fn current_position(&self) -> Vec2 {
        let p = self.0.current_position();
        Vec2::new(p.x, p.y)
    }
}
