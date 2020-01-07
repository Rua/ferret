#version 450

layout(set = 0, binding = 0) uniform UniformBufferObject {
	mat4 view;
	mat4 proj;
} ubo;

layout(location = 0) in vec3 in_position;

layout(location = 0) out vec2 frag_texture_coord;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	gl_Position = ubo.proj * ubo.view * vec4(in_position, 1);
	frag_texture_coord = (ubo.view * vec4(in_position, 1)).xy / gl_Position.z;
}
