#version 450

layout(set = 1, binding = 0) uniform sampler2D texture_sampler;

layout(location = 0) in vec2 in_texture_coord;
layout(location = 1) in float in_light_level;

layout(location = 0) out vec4 out_color;

void main() {
	float light_factor = 60.0 - 64.0 * in_light_level;
	float distance_factor = 80.0 / ((0.0625 / gl_FragCoord.w) + 1.0);
	float light_level = 1.0 - (light_factor - distance_factor) / 32.0;
	light_level = clamp(light_level, 0.0, 1.0);

	vec4 texture_texel = texture(texture_sampler, in_texture_coord);
	out_color = vec4(texture_texel.rgb * light_level, texture_texel.a);

	if (out_color.a < 0.5)
		discard;
}
