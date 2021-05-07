#version 450

layout(set = 0, binding = 0) uniform Matrices {
	mat4 proj;
};

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec2 in_texture_coord;

layout(location = 0) out vec2 vert_texture_coord;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	gl_Position = proj * vec4(in_position, 0.0, 1.0);
	vert_texture_coord = in_texture_coord;
}
