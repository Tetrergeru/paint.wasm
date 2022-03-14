#version 300 es
precision mediump float;

in vec2 fragCoord;

out vec4 color;

uniform float radius;

vec3 hsvToRgb(float hue, float s, float v) {
    float h = hue / 60.0;

    float c = v * s;

    float x = c * (1.0 - abs(mod(h, 2.0) - 1.0));

    vec3 rgb;

    if (0.0 <= h && h < 1.0) rgb = vec3(c, x, 0.0);
    else if (1.0 <= h && h < 2.0) rgb = vec3(x, c, 0.0);
    else if (2.0 <= h && h < 3.0) rgb = vec3(0.0, c, x);
    else if (3.0 <= h && h < 4.0) rgb = vec3(0.0, x, c);
    else if (4.0 <= h && h < 5.0) rgb = vec3(x, 0.0, c);
    else if (5.0 <= h && h < 6.0) rgb = vec3(c, 0.0, x);

    float m = v - c;

    return rgb + vec3(m, m, m);
}

const float PIx2 = 2.0 * 3.1415;

void main() {
    float dist = sqrt(dot(fragCoord, fragCoord));

    vec2 norm = fragCoord / dist;

    float angle;
    if (norm.y < 0.0)
        angle = acos(norm.x) * 360.0 / PIx2;
    else
        angle = (PIx2 - acos(norm.x)) * 360.0 / PIx2;

    if (dist > 1.0)
        discard;

    vec3 targetColor = hsvToRgb(angle, dist, 1.0);

    float border = 1.5 / radius;

    if (dist > 1.0 - border)
        color = vec4(targetColor, -(dist - 1.0) / border);
    else
        color = vec4(targetColor, 1.0);
}