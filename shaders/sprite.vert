#version 450

layout(set = 0, binding = 0) uniform Matrices {
	mat4 proj;
	mat4 view;
	mat4 billboard;
	float extra_light;
};

layout(set = 1, binding = 0) uniform sampler2D texture_sampler;
layout(set = 1, binding = 1) uniform ImageMatrix {
	mat4 image_matrix;
};

// Per-instance
layout(location = 0) in mat4 in_transform;
layout(location = 4) in float in_flip;
layout(location = 5) in float in_light_level;

// Output
layout(location = 0) out vec2 frag_texture_coord;
layout(location = 1) out float frag_light_level;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	frag_texture_coord.x = gl_VertexIndex >> 1;
	frag_texture_coord.y = (gl_VertexIndex & 1) ^ (gl_VertexIndex >> 1);

	vec4 vert = image_matrix * vec4(frag_texture_coord, 0.0, 1.0);
	vert = vec4(0.0, -vert.x, -vert.y, 1.0);
	gl_Position = proj * view * in_transform * billboard * vert;

	frag_texture_coord.x *= in_flip;
	frag_light_level = in_light_level + extra_light;
}
