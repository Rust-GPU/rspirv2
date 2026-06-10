// glslc textureGradOffset.frag -o textureGradOffset.spv

#version 450 core

layout (set = 0, binding = 0) buffer myBuffer {
    uint mode;
};
layout (set = 0, binding = 1) uniform sampler2D image;

layout (location = 0) in vec2 vtx_tex_coord;
layout (location = 0) out vec4 out_color;

void main() {
    vec2 tex_coord;
    // OpSwitch
    switch (mode) {
        case 0:
            tex_coord = vtx_tex_coord;
            break;
        case 1:
            tex_coord = 1. - vtx_tex_coord;
            break;
        case 2:
            tex_coord = vtx_tex_coord * 0.5 + 0.5;
            break;
    }
    // OpPhi

    vec2 grad_x = dFdx(tex_coord);
    vec2 grad_y = dFdy(tex_coord);

    // Uses parametized bitmask operand `ImageOperands` with `Grad|Offset` bitmask and both having 2 params
    out_color = textureGradOffset(image, tex_coord, grad_x, grad_y, ivec2(1, -1));
}
