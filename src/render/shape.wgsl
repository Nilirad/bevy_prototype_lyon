// Import the standard 2d mesh uniforms and set their bind groups
#import bevy_sprite::mesh2d_view_bind_group
[[group(0), binding(0)]]
var<uniform> view: View;
#import bevy_sprite::mesh2d_struct
[[group(1), binding(0)]]
var<uniform> mesh: Mesh2d;
// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: u32;
};
struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    [[builtin(position)]] clip_position: vec4<f32>;
    // We pass the vertex color to the framgent shader in location 0
    [[location(0)]] color: vec4<f32>;
};
/// Entry point for the vertex shader
[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position of the mesh into screen position
    out.clip_position = view.view_proj * mesh.model * vec4<f32>(vertex.position, 1.0);
    out.color = vec4<f32>((vec4<u32>(vertex.color) >> vec4<u32>(0u, 8u, 16u, 24u)) & vec4<u32>(255u)) / 255.0;
    return out;
}
// The input of the fragment shader must correspond to the output of the vertex shader for all `location`s
struct FragmentInput {
    // The color is interpolated between vertices by default
    [[location(0)]] color: vec4<f32>;
};
/// Entry point for the fragment shader
[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    return in.color;
}
