use bevy::math::Vec2;
use lyon_tessellation::{
    math::{Angle, Point, Vector},
    path::{builder::WithSvg, path::Builder, traits::SvgPathBuilder, ArcFlags},
};
use svgtypes::{PathParser, PathSegment};

use crate::geometry::Geometry;

///An easy way to display svg paths as a shape, takes an svg path string and a
///document size(Vec2).
///
///For documentation on svg paths: <https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths>
///
///Make sure that your units are pixels(px) and that the transform of the \<g\>
///in your svg document is set to transform="translate(0,0)" so as to not
///offset the coordinates of the paths
///
///In inkscape for example, to turn your units into pixels, you:
/// 1) Go to File>Document Properties>General>Display Units and set it to px
///
/// 2) In File>Document Properties>Custom Size>Units set it to px, also, this
///    size would be used for `svg_doc_size_in_px`
///
/// 3) In File>Document Properties>Scale>Scale x make sure it is set to 1 User
///    unit per px
///
///Example exists in the examples folder
pub struct SvgPathShape {
    ///The document size of the svg art, make sure the units are in pixels
    pub svg_doc_size_in_px: Vec2,
    ///The string that describes the path, make sure the units are in pixels
    ///and that the transform of the \<g\> in your svg document is set to
    ///transform="translate(0,0)" so as to not offset the coordinates of the
    ///paths
    pub svg_path_string: String,
}
fn get_y_in_bevy_orientation(y: f64) -> f32 {
    y as f32 * -1.
}
fn get_y_after_offset(y: f64, offset_y: f32) -> f32 {
    get_y_in_bevy_orientation(y) + offset_y
}
fn get_x_after_offset(x: f64, offset_x: f32) -> f32 {
    x as f32 - offset_x
}
fn get_point_after_offset(x: f64, y: f64, offset_x: f32, offset_y: f32) -> Point {
    Point::new(
        get_x_after_offset(x, offset_x),
        get_y_after_offset(y, offset_y),
    )
}
fn get_corrected_relative_vector(x: f64, y: f64) -> Vector {
    Vector::new(x as f32, get_y_in_bevy_orientation(y))
}
impl Geometry<Builder> for SvgPathShape {
    #[allow(clippy::too_many_lines)]
    fn add_geometry(&self, b: &mut Builder) {
        let builder = Builder::new();
        let mut svg_builder = WithSvg::new(builder);
        let offset_x = self.svg_doc_size_in_px.x / 2.;
        let offset_y = self.svg_doc_size_in_px.y / 2.;
        let mut used_move_command = false;

        for path_segment in PathParser::from(self.svg_path_string.as_str()) {
            match path_segment.unwrap() {
                PathSegment::MoveTo { abs, x, y } => {
                    if abs || !used_move_command {
                        svg_builder.move_to(get_point_after_offset(x, y, offset_x, offset_y));
                        used_move_command = true;
                    } else {
                        svg_builder.relative_move_to(get_corrected_relative_vector(x, y));
                    }
                }
                PathSegment::LineTo { abs, x, y } => {
                    if abs {
                        svg_builder.line_to(get_point_after_offset(x, y, offset_x, offset_y));
                    } else {
                        svg_builder.relative_line_to(get_corrected_relative_vector(x, y));
                    }
                }
                PathSegment::HorizontalLineTo { abs, x } => {
                    if abs {
                        svg_builder.horizontal_line_to(get_x_after_offset(x, offset_x));
                    } else {
                        svg_builder.relative_horizontal_line_to(x as f32);
                    }
                }
                PathSegment::VerticalLineTo { abs, y } => {
                    if abs {
                        svg_builder.vertical_line_to(get_y_after_offset(y, offset_y));
                    } else {
                        svg_builder.relative_vertical_line_to(get_y_in_bevy_orientation(y));
                    }
                }
                PathSegment::CurveTo {
                    abs,
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => {
                    if abs {
                        svg_builder.cubic_bezier_to(
                            get_point_after_offset(x1, y1, offset_x, offset_y),
                            get_point_after_offset(x2, y2, offset_x, offset_y),
                            get_point_after_offset(x, y, offset_x, offset_y),
                        );
                    } else {
                        svg_builder.relative_cubic_bezier_to(
                            get_corrected_relative_vector(x1, y1),
                            get_corrected_relative_vector(x2, y2),
                            get_corrected_relative_vector(x, y),
                        );
                    }
                }
                PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                    if abs {
                        svg_builder.smooth_cubic_bezier_to(
                            get_point_after_offset(x2, y2, offset_x, offset_y),
                            get_point_after_offset(x, y, offset_x, offset_y),
                        );
                    } else {
                        svg_builder.smooth_relative_cubic_bezier_to(
                            get_corrected_relative_vector(x2, y2),
                            get_corrected_relative_vector(x, y),
                        );
                    }
                }
                PathSegment::Quadratic { abs, x1, y1, x, y } => {
                    if abs {
                        svg_builder.quadratic_bezier_to(
                            get_point_after_offset(x1, y1, offset_x, offset_y),
                            get_point_after_offset(x, y, offset_x, offset_y),
                        );
                    } else {
                        svg_builder.relative_quadratic_bezier_to(
                            get_corrected_relative_vector(x1, y1),
                            get_corrected_relative_vector(x, y),
                        );
                    }
                }
                PathSegment::SmoothQuadratic { abs, x, y } => {
                    if abs {
                        svg_builder.smooth_quadratic_bezier_to(get_point_after_offset(
                            x, y, offset_x, offset_y,
                        ));
                    } else {
                        svg_builder.smooth_relative_quadratic_bezier_to(
                            get_corrected_relative_vector(x, y),
                        );
                    }
                }
                PathSegment::EllipticalArc {
                    abs,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    x,
                    y,
                } => {
                    if abs {
                        svg_builder.arc_to(
                            Vector::new(rx as f32, ry as f32),
                            Angle {
                                radians: x_axis_rotation as f32,
                            },
                            ArcFlags { large_arc, sweep },
                            get_point_after_offset(x, y, offset_x, offset_y),
                        );
                    } else {
                        svg_builder.relative_arc_to(
                            Vector::new(rx as f32, ry as f32),
                            Angle {
                                radians: x_axis_rotation as f32,
                            },
                            ArcFlags { large_arc, sweep },
                            get_corrected_relative_vector(x, y),
                        );
                    }
                }
                PathSegment::ClosePath { abs: _ } => {
                    svg_builder.close();
                }
            }
        }
        let path = svg_builder.build();
        b.extend_from_paths(&[path.as_slice()]);
    }
}
