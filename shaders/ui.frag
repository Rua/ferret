#version 450

layout(set = 1, binding = 0) uniform sampler2D texture_sampler;

layout(location = 0) in vec2 in_texture_coord;

layout(location = 0) out vec4 out_color;

void main() {
	vec4 texture_texel = texture(texture_sampler, in_texture_coord);
	out_color = texture_texel;

	if (out_color.a < 0.5)
		discard;
}
