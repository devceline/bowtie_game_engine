#version 150 core

uniform sampler2D pride_flag;
uniform sampler2D patrick;

in vec3 Color;
in vec2 tex_cords_out;

out vec4 outColor;

void main()
{
		vec4 tex1 = texture(patrick, tex_cords_out);
		vec4 tex2 = texture(pride_flag, tex_cords_out);
    outColor = mix(tex1, tex2, 0.5);
}
