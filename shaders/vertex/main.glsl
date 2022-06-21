#version 150 core

in vec2 position;
in vec4 targetColor;
in vec2 tex_cords_in;

in float tex_id;

out vec4 Color;
out vec2 tex_cords_out;
out float Tex_id;

uniform mat4 trans;

void main()
{
		Tex_id = tex_id;
    Color = targetColor;
		tex_cords_out = tex_cords_in;
    gl_Position = trans * vec4(position, 0.0, 1.0);
}
