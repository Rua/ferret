#version 450
layout(binding = 1) uniform sampler2DArray tex_sampler;

layout(location = 0) in vec3 frag_tex_coord;

layout(location = 0) out vec4 out_color;

void main()
{
	/*float factor = 60.0 - 4.0 * light_level - (80.0 / ((0.0625 / gl_FragCoord.w) + 1.0));
	float level = 1 - factor / 32.0;
	level = clamp(level, 0.0, 1.0);*/
	
	vec4 texel = texture(tex_sampler, frag_tex_coord, 0);
	out_color = vec4(texel.rgb, texel.a);
	
	if (out_color.a < 0.5)
		discard;
}
