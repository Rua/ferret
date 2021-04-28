#version 450

layout(set = 0, binding = 0) uniform Matrices {
	mat4 proj;
};

layout(set = 1, binding = 0) uniform sampler2D texture_sampler;

// Per-instance
layout(location = 0) in vec2 in_position;
layout(location = 1) in vec2 in_size;
layout(location = 2) in vec2 in_texture_position;
layout(location = 3) in vec2 in_texture_size;

// Output
layout(location = 0) out vec2 vert_texture_coord;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	vert_texture_coord.x = gl_VertexIndex >> 1;
	vert_texture_coord.y = (gl_VertexIndex & 1) ^ (gl_VertexIndex >> 1);

	vec4 vert = vec4(in_size * vert_texture_coord, 0.0, 1.0);
	vert.xy += in_position;
	gl_Position = proj * vert;

	vert_texture_coord = vert_texture_coord * in_texture_size + in_texture_position;
	vert_texture_coord /= textureSize(texture_sampler, 0);
}
