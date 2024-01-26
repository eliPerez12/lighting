#version 330

uniform vec2 screenSize;

void main() {
    // Calculate the normalized screen coordinates
    vec2 uv = gl_FragCoord.xy / screenSize;

    // Interpolate from white to black based on the y-coordinate
    vec3 color = mix(vec3(1.0), vec3(0.0), uv.y);

    // Output the final color
    gl_FragColor = vec4(color, 1.0);
}
