# Changelog

## 0.8.0
- Support for Bevy 0.10.
- Uses original render.
- Added `RoundedPolygon`.
- `FillMode` and `StrokeMode` are now components and have been renamed to `Fill` and `Stroke`.

## 0.7.2
- Fixed crash when using HDR textures.

## 0.7.1
- Fixed wrong rectangle origin bug.

## 0.7.0
- Support for Bevy 0.9
- Update lyon_tesselation to 1.0
- Update svgtypes to 0.8

## 0.6.0
- Support for Bevy 0.8

## 0.5.0
- Support for Bevy 0.7

## 0.4.0
- Support for Bevy 0.6
- Shape properties can be dynamically changed

## 0.3.1
- Restored support for bevy_webgl2 (lost on v0.3.0).

## 0.3.0
- Support for Bevy 0.5
- Shapes with outline

## 0.2.0
- Complete API reworking
- Regular polygon support
- Extensible shape system through `Geometry` trait

## 0.1.5
- updated dependency to `lyon_tessellation v0.17`
- with `lyon_tessellation v0.17`, unfortunately rectangles with rounded borders are no longer supported.
- `Quad`, `Triangle` and `Polyline` have been substituted by a general-purpose `Polygon` shape.
