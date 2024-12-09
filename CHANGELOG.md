# Changelog

## 0.14.0

This version uses Bevy's **Required Components**,
therefore many items have been changed.
Examples may help migration.

- `Path` component renamed to `Shape`. It is now the central component of a shape entity.
- `Shape` now includes fill and stroke data.
- `Fill` and `Stroke` are no longer `Component`s.
- Deprecated `ShapeBundle` in favor of the `Shape` component.
- `prelude` no longer exports `ShapeBundle`.
- Added `ShapeBuilder`: works similarly to `GeometryBuilder`
- Removed `GeometryBuilder` and `ShapePath`.
- `PathBuilder` now allows chaining methods.

## 0.13.0
- Support for Bevy 0.15.0.
- `Rectangle` now supports border radii (see `rectangle.rs` example).
- Removed deprecated `SpatialBundle` from `ShapeBundle`: `Transform` and `Visibility` are now added separately.

## 0.12.0
- Support for Bevy 0.14.

## 0.11.0
- Support for Bevy 0.13.

## 0.10.0
- Support for Bevy 0.12.
- `ShapeBundle` now contains the `spatial: SpatialBundle` field, which bundles together `Transform`, `GlobalTransform`, `Visibility` and `InheritedVisibility`.

## 0.9.0
- Support for Bevy 0.11.
- `ShapeBundle` now contains the `spatial: SpatialBundle` field,
  which bundles together
  `Transform`,
  `GlobalTransform`,
  `Visibility`
  and `InheritedVisibility`.

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
