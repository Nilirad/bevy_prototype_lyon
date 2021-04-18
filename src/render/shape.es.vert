#version 300 es

in vec2 Vertex_Position_2D;
in vec4 Vertex_Color;

out vec4 v_color;

layout(std140) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(std140) uniform Transform {
    mat4 Model;
};

void main() {
    v_color = Vertex_Color;
    gl_Position = ViewProj * Model * vec4(Vertex_Position_2D, 0.0, 1.0);
}