//! Shapes for easily constructing basic meshes with [`LyonMeshBuilder`].
//!
//! # Overview
//! 
//! This module provides a set of shapes consumable by the [`LyonMeshBuilder`] which draws some simple basic shapes.
//! The shapes provided here match with the shapes that have simple tesselators provided by `lyon`.
//! 
//! [`LyonMeshBuilder`]: crate::mesh_builder::LyonMeshBuilder

use smart_default::*;

use lyon::{
    math,
    tessellation::{
        self as tess,
        basic_shapes,
    },
};

use super::mesh_builder::BevyBuffersBuilder;

/// Represents something capable of being built into a shape with the [`LyonMeshBuilder`](crate::mesh_builder::LyonMeshBuilder).
pub trait LyonShapeBuilder
{
    fn build(self, builder: &mut BevyBuffersBuilder);
}

/// Allow all closures and functions that take in a mutable reference to a [`BevyBuffersBuilder`] to be considered a shape builder.
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

/// Requires the points to represent a convex shape. If the shape is concave the result will likely be incorrect.
#[derive(Debug, SmartDefault)]
pub struct FillConvexPolyline<'a, I, G>
where
    I: IntoIterator<Item=math::Point, IntoIter=G> + Default,
    G: Iterator<Item=math::Point> + Clone
{
    pub points: I,
    #[default(&tess::FillOptions::DEFAULT)]
    pub options: &'a tess::FillOptions,
}

impl<I, G> LyonShapeBuilder for FillConvexPolyline<'_, I, G>
where
    I: IntoIterator<Item=math::Point, IntoIter=G> + Default,
    G: Iterator<Item=math::Point> + Clone
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::fill_convex_polyline(
            self.points.into_iter(),
            self.options, 
            builder
        );
    }
}

// Debug is not derived because tess::FillTessellator does not derive it.
// We can hack around now having Default, but hacking around not having Debug is a bit much...
#[derive(SmartDefault)]
pub struct FillPolyline<'a, I>
where
    I: IntoIterator<Item=math::Point> + Default
{
    pub points: I,
    #[default(tess::FillTessellator::new())]
    pub tessellator: tess::FillTessellator,
    #[default(&tess::FillOptions::DEFAULT)]
    pub options: &'a tess::FillOptions,
}

impl<I> LyonShapeBuilder for FillPolyline<'_, I>
where
    I: IntoIterator<Item=math::Point> + Default
{
    fn build(mut self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::fill_polyline(
            self.points, 
            &mut self.tessellator, 
            self.options, 
            builder
        );
    }
}

// TODO: Check what happens when the points aren't at right angles!
#[derive(Debug, SmartDefault)]
pub struct FillQuad<'a> {
    #[default([ math::point(0.0, 0.0), math::point(0.0, 25.0), math::point(25.0, 25.0), math::point(25.0, 0.0) ])]
    points: [math::Point; 4],
    #[default(&tess::FillOptions::DEFAULT)]
    options: &'a tess::FillOptions,
}

impl LyonShapeBuilder for FillQuad<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder) 
    {
        let _ = basic_shapes::fill_quad(
            self.points[0],
            self.points[1],
            self.points[2],
            self.points[3],
            self.options,
            builder
        );
    }
}

#[derive(Debug, SmartDefault)]
pub struct FillRect<'a> {
    rect: math::Rect,
    #[default(&tess::FillOptions::DEFAULT)]
    options: &'a tess::FillOptions,
}

impl LyonShapeBuilder for FillRect<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::fill_rectangle(
            &self.rect,
            self.options,
            builder
        );
    }
}

// Debug is not derived because basic_shapes::BorderRadii does not derive it.
// We can hack around now having Default, but hacking around not having Debug is a bit much...
#[derive(SmartDefault)]
pub struct FillRoundedRect<'a> {
    pub rect: math::Rect,
    #[default(basic_shapes::BorderRadii::new_all_same(10.0))]
    pub radii: basic_shapes::BorderRadii,
    #[default(&tess::FillOptions::DEFAULT)]
    pub options: &'a tess::FillOptions,
}

impl LyonShapeBuilder for FillRoundedRect<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::fill_rounded_rectangle(
            &self.rect,
            &self.radii,
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

#[derive(Debug, SmartDefault)]
pub struct StrokeEllipse<'a> {
    pub center: math::Point,
    #[default(math::vector(40.0, 25.0))]
    pub radii: math::Vector,
    pub x_rotation: math::Angle,
    #[default(&tess::StrokeOptions::DEFAULT)]
    pub options: &'a tess::StrokeOptions
}

impl LyonShapeBuilder for StrokeEllipse<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::stroke_ellipse(
            self.center,
            self.radii,
            self.x_rotation,
            self.options,
            builder
        );
    }
}

#[derive(Debug, SmartDefault)]
pub struct StrokePolyline<'a, I>
where
    I: IntoIterator<Item=math::Point> + Default
{
    pub points: I,
    #[default = true]
    pub is_closed: bool,
    #[default(&tess::StrokeOptions::DEFAULT)]
    pub options: &'a tess::StrokeOptions,
}

impl<I> LyonShapeBuilder for StrokePolyline<'_, I>
where
    I: IntoIterator<Item=math::Point> + Default
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::stroke_polyline(
            self.points,
            self.is_closed,
            self.options,
            builder
        );

    }
}

#[derive(Debug, SmartDefault)]
pub struct StrokeQuad<'a>
{
    #[default([ math::point(0.0, 0.0), math::point(0.0, 25.0), math::point(25.0, 25.0), math::point(25.0, 0.0) ])]
    pub points: [math::Point; 4],
    #[default(&tess::StrokeOptions::DEFAULT)]
    pub options: &'a tess::StrokeOptions
}

impl LyonShapeBuilder for StrokeQuad<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::stroke_quad(
            self.points[0],
            self.points[1],
            self.points[2],
            self.points[3],
            self.options, 
            builder
        );
    }
}

#[derive(Debug, SmartDefault)]
pub struct StrokeRect<'a> {
    rect: math::Rect,
    #[default(&tess::StrokeOptions::DEFAULT)]
    options: &'a tess::StrokeOptions,
}

impl LyonShapeBuilder for StrokeRect<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::stroke_rectangle(
            &self.rect,
            self.options,
            builder
        );
    }
}

// Debug is not derived because basic_shapes::BorderRadii does not derive it.
// We can hack around now having Default, but hacking around not having Debug is a bit much...
#[derive(SmartDefault)]
pub struct StrokeRoundedRect<'a> {
    pub rect: math::Rect,
    #[default(basic_shapes::BorderRadii::new_all_same(10.0))]
    pub radii: basic_shapes::BorderRadii,
    #[default(&tess::StrokeOptions::DEFAULT)]
    pub options: &'a tess::StrokeOptions,
}

impl LyonShapeBuilder for StrokeRoundedRect<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::stroke_rounded_rectangle(
            &self.rect,
            &self.radii,
            self.options,
            builder
        );
    }
}

#[derive(Debug, SmartDefault)]
pub struct StrokeTriangle<'a> {
    #[default([ math::point(0.0, 0.0), math::point(25.0/2.0, 25.0), math::point(25.0, 0.0),])]
    pub points: [math::Point; 3],
    #[default(&tess::StrokeOptions::DEFAULT)]
    pub options: &'a tess::StrokeOptions
}

impl LyonShapeBuilder for StrokeTriangle<'_>
{
    fn build(self, builder: &mut BevyBuffersBuilder)
    {
        let _ = basic_shapes::stroke_triangle(
            self.points[0],
            self.points[1],
            self.points[2],
            self.options,
            builder
        );
    }
}
