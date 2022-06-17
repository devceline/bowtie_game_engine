#version 150 core

in vec2 position;
in vec3 targetColor;
in vec2 tex_cords_in;

out vec3 Color;
out vec2 tex_cords_out;

void main()
{
    Color = targetColor;
		tex_cords_out = tex_cords_in;
    gl_Position = vec4(position, 0.0, 1.0);
}
