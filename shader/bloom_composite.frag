#version 330 core

in vec2 uv;

out vec4 fragColor;

uniform float tone = 1.0;
uniform float bloomStrength = 0.1;
uniform float flareStrength = 0.25;

uniform sampler2D texture0;
uniform sampler2D texture1;
uniform sampler2D texture2;

void main() {
  fragColor = texture(texture0, uv) * tone +
              texture(texture1, uv) * bloomStrength +
              texture(texture2, uv) * flareStrength;
}
