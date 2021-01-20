//! Draw 2D shapes in Bevy.
//!
//! This crate provides a Bevy [plugin] to easily draw shapes.
//! Some shapes are provided for convenience, however you can extend the
//! functionality of this crate by implementing the
//! [`Geometry`](geometry::Geometry) trait by your own.
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

pub mod entity;
pub mod geometry;
pub mod path;
pub mod plugin;
pub mod shapes;
pub mod utils;

/// Import this module as `use bevy_prototype_lyon::prelude::*` to get
/// convenient imports.
pub mod prelude {
    pub use crate::{
        geometry::{Geometry, GeometryBuilder},
        path::PathBuilder,
        plugin::ShapePlugin,
        shapes,
        utils::TessellationMode,
    };
    pub use lyon_tessellation::{
        path::path::Builder, FillOptions, FillRule, LineCap, LineJoin, Orientation, StrokeOptions,
    };
}
