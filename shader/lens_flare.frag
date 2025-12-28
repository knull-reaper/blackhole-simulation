#version 330 core

in vec2 uv;

out vec4 fragColor;

uniform sampler2D texture0;
uniform vec2 resolution;

const vec3 luminanceVector = vec3(0.2125, 0.7154, 0.0721);

void main() {
  float texel = 1.0 / max(resolution.x, 1.0);
  vec3 color = vec3(0.0);

  vec3 sample0 = texture(texture0, uv + vec2(-3.0 * texel, 0.0)).rgb;
  vec3 sample1 = texture(texture0, uv + vec2(-2.0 * texel, 0.0)).rgb;
  vec3 sample2 = texture(texture0, uv + vec2(-1.0 * texel, 0.0)).rgb;
  vec3 sample3 = texture(texture0, uv).rgb;
  vec3 sample4 = texture(texture0, uv + vec2(1.0 * texel, 0.0)).rgb;
  vec3 sample5 = texture(texture0, uv + vec2(2.0 * texel, 0.0)).rgb;
  vec3 sample6 = texture(texture0, uv + vec2(3.0 * texel, 0.0)).rgb;

  float h0 = smoothstep(0.9, 1.6, dot(sample0, luminanceVector));
  float h1 = smoothstep(0.9, 1.6, dot(sample1, luminanceVector));
  float h2 = smoothstep(0.9, 1.6, dot(sample2, luminanceVector));
  float h3 = smoothstep(0.9, 1.6, dot(sample3, luminanceVector));
  float h4 = smoothstep(0.9, 1.6, dot(sample4, luminanceVector));
  float h5 = smoothstep(0.9, 1.6, dot(sample5, luminanceVector));
  float h6 = smoothstep(0.9, 1.6, dot(sample6, luminanceVector));

  color += sample0 * (0.07 * h0);
  color += sample1 * (0.12 * h1);
  color += sample2 * (0.17 * h2);
  color += sample3 * (0.28 * h3);
  color += sample4 * (0.17 * h4);
  color += sample5 * (0.12 * h5);
  color += sample6 * (0.07 * h6);

  color *= vec3(0.6, 0.85, 1.2);

  fragColor = vec4(color, 1.0);
}
