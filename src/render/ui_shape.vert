#version 450

layout(location = 0) in vec2 Vertex_Position_2D;
layout(location = 1) in vec4 Vertex_Color;

layout(location = 0) out vec4 v_color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Object;
};

void main() {
    v_color = Vertex_Color;
    vec2 position = Vertex_Position_2D;
    gl_Position = ViewProj * Object * vec4(position, 0.0, 1.0);
}
