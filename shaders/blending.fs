#version 430

uniform sampler2D texture;
uniform sampler2D lightMap;
uniform vec2 screenSize;

void main() {
    vec2 uv = gl_FragCoord.xy / screenSize;

    vec3 color = texture(texture, uv).rgb;
    vec3 light = texture(lightMap , uv).rgb;
    // Output the final color with the original alpha
    gl_FragColor = vec4(color, 1.0);
}