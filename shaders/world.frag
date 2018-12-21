#version 450
layout(binding = 1) uniform sampler2DArray texture_sampler;
layout(binding = 2) uniform sampler2DArray lightmap_sampler;

layout(location = 0) in vec3 frag_texture_coord;
layout(location = 1) in vec3 frag_lightmap_coord;

layout(location = 0) out vec4 out_color;

void main()
{
	vec3 lightmap_texel = texture(lightmap_sampler, frag_lightmap_coord, 0).rgb;
	vec3 light_factor = 60.0 - 64.0 * lightmap_texel;
	float distance_factor = 80.0 / ((0.0625 / gl_FragCoord.w) + 1.0);
	vec3 light_level = 1.0 - (light_factor - distance_factor) / 32.0;
	light_level = clamp(light_level, 0.0, 1.0);
	
	vec4 texture_texel = texture(texture_sampler, frag_texture_coord, 0);
	out_color = vec4(texture_texel.rgb * light_level, texture_texel.a);
	
	if (out_color.a < 0.5)
		discard;
}
