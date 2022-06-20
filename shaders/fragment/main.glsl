#version 150 core

uniform sampler2D tex0_sampler;
uniform sampler2D tex1_sampler;
uniform sampler2D tex2_sampler;
uniform sampler2D tex3_sampler;

in vec4 Color;
in vec2 tex_cords_out;
out vec4 outColor;
in float Tex_id;

void main()
{
	// TODO: Refactor this, figure it out.
	if (Tex_id == 0.0f) {
		outColor = texture(tex0_sampler, tex_cords_out);
	}
	else if (Tex_id == 1.0f) {
		outColor = texture(tex1_sampler, tex_cords_out);
	}
	else if (Tex_id == 2.0f) {
		outColor = texture(tex2_sampler, tex_cords_out);
	}
	else if (Tex_id == 3.0f) {
		outColor = texture(tex3_sampler, tex_cords_out);
	}
	else {
		outColor = vec4(Color);
	}


}
