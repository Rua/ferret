#version 450
layout(binding = 0) uniform sampler2D tex_sampler;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_tex_coord;

layout(location = 0) out vec2 frag_tex_coord;

out gl_PerVertex {
	vec4 gl_Position;
};

void main()
{
	frag_tex_coord = in_tex_coord / textureSize(tex_sampler, 0);
	gl_Position = vec4(in_position, 1);
}
