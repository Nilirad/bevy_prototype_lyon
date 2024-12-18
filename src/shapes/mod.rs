//! Collection of common shapes that can be drawn.
//!
//! The structs defined in this module implement the
//! [`Geometry`](crate::geometry::Geometry) trait. You can also implement
//! the trait for your own shapes.

pub mod circle;
pub mod ellipse;
pub mod line;
pub mod polygon;
pub mod rectangle;
pub mod svg;

pub use circle::*;
pub use ellipse::*;
pub use line::*;
pub use polygon::*;
pub use rectangle::*;
pub use svg::*;
