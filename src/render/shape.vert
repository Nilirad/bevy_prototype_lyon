#version 450

layout(location = 0) in vec3 Vertex_Position_2D;
layout(location = 1) in vec4 Vertex_Color;

layout(location = 0) out vec4 v_color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_color = Vertex_Color;
    gl_Position = ViewProj * Model * vec4(Vertex_Position_2D, 1.0);
}