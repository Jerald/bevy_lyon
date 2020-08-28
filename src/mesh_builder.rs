use bevy::render::{
    mesh::{
        VertexAttribute,
        Mesh
    },
    pipeline::PrimitiveTopology,
};

use lyon::{
    math::{
        self,
        Point
    },
    tessellation as tess,
};

use super::shapes::LyonShapeBuilder;

/// Type alias for the type of a mesh index in bevy.
pub type BevyIndex = u32;
/// Type alias for a `VertexBuffers` of `BevyVertex`'s and `BevyIndex`'s.
pub type BevyVertexBuffers = tess::VertexBuffers<BevyVertex, BevyIndex>;
/// Type alias for a buffer builder that contains the information to properly convert lyon points to `BevyVertex`'s and `BevyIndex`'s.
pub type BevyBuffersBuilder<'a> = tess::BuffersBuilder<'a, BevyVertex, BevyIndex, BevyVertexConstructor>;

/// Builder that provides customizable functionality to create `lyon` tessellated meshes and build them so `bevy` can consume them.
#[derive(Debug, Clone)]
pub struct LyonMeshBuilder
{
    geometry: BevyVertexBuffers
}

impl LyonMeshBuilder
{
    /// Create a new mesh builder.
    pub fn new() -> Self
    {
        LyonMeshBuilder {
            geometry: BevyVertexBuffers::new()
        }
    }

    /// Finish the building and produce the final mesh.
    ///
    /// Uses TriangleStrip as the default primitive topology.
    pub fn build(self) -> Mesh
    {
        self.build_with_topology(PrimitiveTopology::TriangleStrip)
    }

    /// Finishes a mesh using a custom specified `PrimitiveTopology`.
    ///
    /// Prefer using `build()` as its default works in the vast majority of cases.
    pub fn build_with_topology(self, topology: PrimitiveTopology) -> Mesh
    {
        Mesh {
            primitive_topology: topology,
            attributes: self.verts_to_attributes(),
            indices: Some(self.geometry.indices),
        }
    }

    /// Adds a shape specified by its `LyonShapeBuilder` implementation to the mesh being constructed.
    pub fn with(mut self, shape: impl LyonShapeBuilder) -> Self
    {
        shape.build(&mut self.buffers_builder());
        self
    }

    /// A convenience function that makes a new `LyonMeshBuilder` and builds it with only the single shape provided.
    pub fn only(shape: impl LyonShapeBuilder) -> Mesh
    {
        LyonMeshBuilder::new()
            .with(shape)
            .build()
    }

    /// Internal utility function to simplify creation of an output buffer builder.
    fn buffers_builder(&mut self) -> tess::BuffersBuilder<BevyVertex, BevyIndex, BevyVertexConstructor>
    {
        tess::BuffersBuilder::new(&mut self.geometry, BevyVertexConstructor)
    }

    /// Internal utility function that transforms an iterator of `BevyVertex`'s into the proper array of vertex attributes.
    fn verts_to_attributes(&self) -> Vec<VertexAttribute>
    {
        let mut positions = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
    
        for vertex in &self.geometry.vertices
        {
            positions.push(vertex.pos);
            normals.push(vertex.norm);
            uvs.push(vertex.uv);
        }

        vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs),
        ]
    }
}

/// Utility type for containing the trait implementations that transforms a lyon point into a `BevyVertex`.
pub struct BevyVertexConstructor;

// TODO: Figure out if uv mapping should be specific for this
impl tess::BasicVertexConstructor<BevyVertex> for BevyVertexConstructor
{
    fn new_vertex(&mut self, point: Point) -> BevyVertex
    {
        point.into()
    }
}

// TODO: Figure out if uv mapping should be specific for this
impl tess::FillVertexConstructor<BevyVertex> for BevyVertexConstructor
{
    fn new_vertex(&mut self, point: Point, _: tess::FillAttributes) -> BevyVertex
    {
        point.into()
    }
}

// TODO: Figure out if uv mapping should be specific for this
impl tess::StrokeVertexConstructor<BevyVertex> for BevyVertexConstructor
{
    fn new_vertex(&mut self, point: Point, _: tess::StrokeAttributes) -> BevyVertex
    {
        point.into()
    }
}

/// Contains all the vertex information needed by bevy to correctly create a mesh.
#[derive(Debug, Clone)]
pub struct BevyVertex
{
    pub pos: [f32; 3],
    pub norm: [f32; 3],
    pub uv: [f32; 2],
}

/// Performs a trivial conversion from a lyon point into a `BevyVertex`
impl From<math::Point> for BevyVertex
{
    fn from(point: math::Point) -> Self
    {
        // In 2d, Z can just be 0
        BevyVertex {
            pos: [point.x, point.y, 0.0],
            norm: [0.0, 0.0, 1.0],
            uv: [point.x, point.y],
        }
    }
}

