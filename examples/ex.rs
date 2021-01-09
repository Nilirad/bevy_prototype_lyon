extern crate lyon_tessellation as tess;
use tess::{
    math::point, BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
    VertexBuffers,
};

// Our custom vertex.
#[derive(Copy, Clone, Debug)]
pub struct MyVertex {
    position: [f32; 2],
    color: [f32; 4],
}

// The vertex constructor. This is the object that will be used to create the
// custom verticex from the information provided by the tessellators.
struct WithColor([f32; 4]);

impl FillVertexConstructor<MyVertex> for WithColor {
    fn new_vertex(&mut self, vertex: FillVertex) -> MyVertex {
        MyVertex {
            position: vertex.position().to_array(),
            color: self.0,
        }
    }
}

fn main() {
    let mut output: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    // Tessellate a red and a green circle.
    tessellator
        .tessellate_circle(
            point(0.0, 0.0),
            10.0,
            &FillOptions::tolerance(0.05),
            &mut BuffersBuilder::new(&mut output, WithColor([1.0, 0.0, 0.0, 1.0])),
        )
        .unwrap();
    tessellator
        .tessellate_circle(
            point(10.0, 0.0),
            5.0,
            &FillOptions::tolerance(0.05),
            &mut BuffersBuilder::new(&mut output, WithColor([0.0, 1.0, 0.0, 1.0])),
        )
        .unwrap();

    println!(
        " -- {} vertices, {} indices",
        output.vertices.len(),
        output.indices.len()
    );
}
