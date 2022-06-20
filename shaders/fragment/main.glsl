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
	
	int tex_int_idx = int(Tex_id);

	switch(tex_int_idx) {
		case 0:
			outColor = texture(tex0_sampler, tex_cords_out);
			break;
		case 1:
			outColor = texture(tex1_sampler, tex_cords_out);
			break;
		case 2:
			outColor = texture(tex2_sampler, tex_cords_out);
			break;
		case 3:
			outColor = texture(tex3_sampler, tex_cords_out);
			break;
		default:
		outColor = vec4(Color);

	}


}
