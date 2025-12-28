#version 330 core

in vec2 uv;

out vec4 fragColor;

uniform float gamma = 2.2;
uniform float tonemappingEnabled;
uniform float time;
uniform float chromaStrength = 0.005;
uniform float grainStrength = 0.02;
uniform float saturation = 1.0;
uniform sampler2D texture0;
uniform vec2 resolution;

const vec3 luminanceVector = vec3(0.2125, 0.7154, 0.0721);

///----
/// Narkowicz 2015, "ACES Filmic Tone Mapping Curve"
vec3 aces(vec3 x) {
  const float a = 2.51;
  const float b = 0.03;
  const float c = 2.43;
  const float d = 0.59;
  const float e = 0.14;
  return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

float aces(float x) {
  const float a = 2.51;
  const float b = 0.03;
  const float c = 2.43;
  const float d = 0.59;
  const float e = 0.14;
  return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}
///----

void main() {
  vec2 center = vec2(0.5);
  vec2 toCenter = uv - center;
  float dist = length(toCenter);
  vec2 dir = dist > 0.0 ? toCenter / dist : vec2(0.0);
  vec2 offset = dir * dist * chromaStrength;

  vec3 color;
  color.r = texture(texture0, uv - offset).r;
  color.g = texture(texture0, uv).g;
  color.b = texture(texture0, uv + offset).b;

  if (tonemappingEnabled > 0.5) {
    // ACES filmic tone mapping
    color = aces(color);

    // Gamma correction
    color = pow(color, vec3(1.0 / gamma));
  }

  float luma = dot(color, luminanceVector);
  color = mix(vec3(luma), color, saturation);

  vec2 noiseUv = uv * resolution + vec2(time * 60.0, time * 37.0);
  float grain = fract(sin(dot(noiseUv, vec2(12.9898, 78.233))) * 43758.5453);
  color += (grain - 0.5) * grainStrength;

  fragColor = vec4(clamp(color, 0.0, 1.0), 1.0);
}
