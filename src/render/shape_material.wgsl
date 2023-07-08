#import bevy_sprite::mesh2d_types  Mesh2d
#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;
@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
