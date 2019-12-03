#version 450

layout(set = 1, binding = 0) uniform sampler2D texture_sampler;
layout(set = 1, binding = 1) uniform FragParams {
	float yaw;
	float pitch;
	vec2 screenSize;
} fp;

layout(location = 0) in vec2 frag_texture_coord;

layout(location = 0) out vec4 out_color;

void main() {
	vec2 texCoords = gl_FragCoord.xy / fp.screenSize;
	float ratio = fp.screenSize.x / fp.screenSize.y;
	texCoords.x = (1.0 - texCoords.x) + (fp.yaw + 45.0) / 360.0 * 4;
	texCoords.y = (texCoords.y + fp.pitch / 60.0) * ratio;

	vec4 texture_texel = texture(texture_sampler, texCoords);
	out_color = vec4(texture_texel.rgb, 1.0);
}
