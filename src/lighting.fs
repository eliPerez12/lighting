#version 330

uniform sampler2D textureSampler;  // Input texture
uniform vec2 screenSize;

void main() {
    // Calculate the normalized screen coordinates
    vec2 uv = gl_FragCoord.xy / screenSize;

    // Interpolate from white to black based on the y-coordinate
    float gradient = mix(1.0, 0.0, uv.y);

    // Sample the texture color
    vec4 texColor = texture2D(textureSampler, uv);

    // Apply the gradient as a mask to the texture color
    vec3 finalColor = texColor.rgb * gradient;

    // Output the final color with the original alpha
    gl_FragColor = vec4(finalColor, texColor.a);
}
