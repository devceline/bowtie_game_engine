#version 150 core


in vec3 Color;
out vec4 outColor;
uniform vec3 triangleColor;

void main()
{
		
    outColor = vec4(Color, 1.0);
}
