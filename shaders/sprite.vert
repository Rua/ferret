#version 450

layout(set = 0, binding = 0) uniform Matrices {
	mat4 view;
	mat4 proj;
} matrices;

layout(set = 1, binding = 0) uniform sampler2D texture_sampler;

// Per-instance
layout(location = 0) in float in_flip;
layout(location = 1) in float in_light_level;
layout(location = 2) in mat4 in_matrix;

// Output
layout(location = 0) out vec2 frag_texture_coord;
layout(location = 1) out float frag_light_level;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	frag_texture_coord.x = gl_VertexIndex >> 1;
	frag_texture_coord.y = (gl_VertexIndex & 1) ^ (gl_VertexIndex >> 1);

	vec4 vert = vec4(0.0, -frag_texture_coord.x, -frag_texture_coord.y, 1.0);
	gl_Position = matrices.proj * matrices.view * in_matrix * vert;

	frag_texture_coord.x *= in_flip;
	frag_light_level = in_light_level;
}
