use smart_default::*;

use lyon::{
    math,
    tessellation::{
        self as tess,
        basic_shapes,
    },
};

use super::mesh_builder::BevyBuffersBuilder;

/// Represents something capable of being built into a shape with the `LyonMeshBuilder`.
pub trait LyonShapeBuilder
{
    fn build(self, builder: &mut BevyBuffersBuilder);
}

/// Allow all closures and functions that take in a mutable reference to a `BevyBuffersBuilder` to be considered a shape builder.
///
/// Permits ergonomically using a closure (or function) for complicated custom meshes.
impl<F> LyonShapeBuilder for F
where
    F: FnOnce(&mut BevyBuffersBuilder)
{
    fn build(self, builder: &mut BevyBuffersBuilder) {
        self(builder);
    }
}

#[derive(Debug, SmartDefault)]
pub struct FillCircle<'a> {
    pub center: math::Point,
    #[default = 25.0]
    pub radius: f32,
    #[default(&tess::FillOptions::DEFAULT)]
    pub options: &'a tess::FillOptions
}

impl LyonShapeBuilder for FillCircle<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder) {
        let _ = basic_shapes::fill_circle(
            self.center,
            self.radius,
            self.options,
            builder
        );
    }
}

#[derive(Debug, SmartDefault)]
pub struct StrokeCircle<'a> {
    pub center: math::Point,
    #[default = 25.0]
    pub radius: f32,
    #[default(&tess::StrokeOptions::DEFAULT)]
    pub options: &'a tess::StrokeOptions
}

impl LyonShapeBuilder for StrokeCircle<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder) {
        let _ = basic_shapes::stroke_circle(
            self.center,
            self.radius,
            self.options,
            builder
        );
    }
}