#version 330 core

uniform sampler2D tex0_sampler;
uniform sampler2D tex1_sampler;
uniform sampler2D tex2_sampler;
uniform sampler2D tex3_sampler;
uniform sampler2D tex4_sampler;
uniform sampler2D tex6_sampler;

in vec4 Color;
in vec2 tex_cords_out;
out vec4 outColor;
in float Tex_id;

void main()
{
	
	int tex_int_idx = int(Tex_id);
	vec4 base_texture = vec4(1.0, 1.0, 1.0, 1.0);

	switch(tex_int_idx) {
		case 0:
			base_texture = texture(tex0_sampler, tex_cords_out);
			break;
		case 1:
			base_texture = texture(tex1_sampler, tex_cords_out);
			break;
		case 2:
			base_texture = texture(tex2_sampler, tex_cords_out);
			break;
		case 3:
			base_texture = texture(tex3_sampler, tex_cords_out);
			break;
		case 4: 
			base_texture = texture(tex4_sampler, tex_cords_out);
			break;
		case 5: 
			base_texture = texture(tex6_sampler, tex_cords_out);
			break;
	}

	outColor = base_texture * vec4(Color);

}
