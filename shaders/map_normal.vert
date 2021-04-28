#version 450

layout(set = 0, binding = 0) uniform Matrices {
	mat4 proj;
	mat4 view;
	mat4 billboard;
	float extra_light;
};

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_texture_coord;
layout(location = 2) in float in_light_level;

layout(location = 0) out vec2 vert_texture_coord;
layout(location = 1) out float vert_light_level;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	vert_texture_coord = in_texture_coord;
	vert_light_level = in_light_level + extra_light;
	gl_Position = proj * view * vec4(in_position, 1);
}
