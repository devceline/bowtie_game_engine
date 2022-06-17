#version 150 core

uniform sampler2D tex;

in vec3 Color;
in vec2 tex_cords_out;

out vec4 outColor;


void main()
{
    outColor = texture(tex, tex_cords_out) * vec4(Color, 1.0);
    outColor = texture(tex, tex_cords_out);
}
