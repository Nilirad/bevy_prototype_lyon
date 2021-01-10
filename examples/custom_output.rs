use lyon_tessellation::{
    math::point, path::polygon::Polygon, Count, GeometryBuilder, GeometryBuilderError,
    StrokeGeometryBuilder, StrokeOptions, StrokeTessellator, StrokeVertex, VertexId,
};

fn main() {
    let mut output = ToStdOut::new();
    let mut tessellator = StrokeTessellator::new();
    tessellator
        .tessellate_polygon(
            Polygon {
                points: &[point(0.0, 0.0), point(10.0, 0.0), point(5.0, 5.0)],
                closed: true,
            },
            &StrokeOptions::default(),
            &mut output,
        )
        .unwrap();
}

pub struct ToStdOut {
    vertices: u32,
    indices: u32,
}

impl ToStdOut {
    pub fn new() -> Self {
        Self {
            vertices: 0,
            indices: 0,
        }
    }
}

impl GeometryBuilder for ToStdOut {
    fn begin_geometry(&mut self) {
        self.vertices = 0;
        self.indices = 0;
        println!(" -- begin geometry");
    }

    fn end_geometry(&mut self) -> Count {
        println!(
            " -- end geometry, {} vertices, {} indices",
            self.vertices, self.indices
        );
        Count {
            vertices: self.vertices,
            indices: self.indices,
        }
    }

    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        println!("triangle ({}, {}, {})", a.offset(), b.offset(), c.offset());
        self.indices += 3;
    }

    fn abort_geometry(&mut self) {
        println!(" -- oops!");
    }
}

impl StrokeGeometryBuilder for ToStdOut {
    fn add_stroke_vertex(
        &mut self,
        vertex: StrokeVertex,
    ) -> Result<VertexId, GeometryBuilderError> {
        println!("vertex {:?}", vertex.position());
        if self.vertices >= u32::MAX {
            return Err(GeometryBuilderError::TooManyVertices);
        }
        self.vertices += 1;
        Ok(VertexId(self.vertices as u32 - 1))
    }
}
