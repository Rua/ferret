#version 450

layout(set = 0, binding = 0) uniform Matrices {
	mat4 view;
	mat4 proj;
} matrices;

layout(set = 1, binding = 0) uniform sampler2D texture_sampler;

layout(set = 2, binding = 0) uniform Instance {
	vec3 position;
} instance;

/*layout(set = 2, binding = 0) uniform Billboard {
	mat4 matrix;
	vec3 position;
} billboard;*/

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec2 in_texture_coord;

layout(location = 0) out vec2 frag_texture_coord;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	mat4 new_view = mat4(1.0);
	new_view[3] = matrices.view[3];
/*		1, 0, 0, matrices.view[3][0],
		0, 1, 0, matrices.view[3][1],
		0, 0, 1, matrices.view[3][2],
		0, 0, 0, 1
	);*/

	frag_texture_coord = in_texture_coord;
	vec3 position = instance.position + vec3(0, -in_position.x, in_position.y);
	gl_Position = matrices.proj * matrices.view * vec4(position, 1);
}
