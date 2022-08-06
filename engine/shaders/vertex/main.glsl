#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec4 targetColor;
layout (location = 2) in vec2 tex_cords_in;
layout (location = 3) in float tex_id;
layout (location = 4) in mat4 trans;


out vec4 Color;
out vec2 tex_cords_out;
out float Tex_id;

void main()
{
		Tex_id = tex_id;
    Color = targetColor;
		tex_cords_out = tex_cords_in;
    gl_Position = trans * vec4(position, 0.0, 1.0);
}


