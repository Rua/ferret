#version 450
layout(binding = 0) uniform sampler2D tex_sampler;

layout(location = 0) in vec2 frag_tex_coord;

layout(location = 0) out vec4 out_color;

void main()
{
	/*float lightFactor = ((15.0 - lightLevel) * 4.0);
	float distanceFactor = (80.0 / ((1.0 / gl_FragCoord.w / 16.0) + 1.0));
	float level = (32.0 - (lightFactor - distanceFactor)) / 32.0;
	level = clamp(level, 0.0, 1.0);*/
	
	vec4 texel = texture(tex_sampler, frag_tex_coord);
	out_color = vec4(texel.rgb, texel.a);
	
	if (out_color.a < 0.5)
		discard;
}
