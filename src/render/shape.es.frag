#version 300 es
precision highp float;

in vec4 v_color;

out vec4 o_Target;

void main() {
    o_Target = v_color;
}