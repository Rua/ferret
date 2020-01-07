#version 450

layout(set = 0, binding = 0) uniform Matrices {
	mat4 view;
	mat4 proj;
} matrices;

layout(set = 1, binding = 0) uniform sampler2D texture_sampler;

layout(set = 2, binding = 0) uniform Instance {
	float light_level;
	mat4 matrix;
} instance;

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec2 in_texture_coord;

layout(location = 0) out vec2 frag_texture_coord;
layout(location = 1) out float frag_light_level;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	frag_texture_coord = in_texture_coord;
	frag_light_level = instance.light_level;

	vec3 position = vec3(0, -in_position.x, in_position.y);
	gl_Position = matrices.proj * matrices.view * instance.matrix * vec4(position, 1);
}
