#version 450

layout(binding = 0) uniform UniformBufferObject {
	mat4 model;
	mat4 view;
	mat4 proj;
} ubo;
layout(binding = 1) uniform sampler2DArray tex_sampler;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_tex_coord;

layout(location = 0) out vec3 frag_tex_coord;

out gl_PerVertex {
	vec4 gl_Position;
};

void main()
{
	frag_tex_coord = vec3(in_tex_coord.xy / textureSize(tex_sampler, 0).xy, in_tex_coord.z);
	gl_Position = ubo.proj * ubo.view * ubo.model * vec4(in_position, 1);
}
