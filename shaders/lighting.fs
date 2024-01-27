#version 430

const int MAX_LIGHTS = 128;

uniform sampler2D textureSampler;
uniform vec2 screenSize;
uniform vec2 lightsPosition[MAX_LIGHTS];
uniform vec4 lightsColor[MAX_LIGHTS];
uniform int lightsAmount;

float quad_falloff(float x) {
    return max(1.0 / (-x / 2.0 + 1) - 1, 0);
}

void main() {
    vec2 uv = gl_FragCoord.xy / screenSize;
    vec4 color_gradient = vec4(0.0);
    vec4 color = texture(textureSampler, uv);

    for (int i = 0; i < lightsAmount; i++) {;
        // Calculate the normalized screen coordinates
        vec2 lightPosition = vec2(lightsPosition[i].x, -lightsPosition[i].y + screenSize.y);
        float worldDistanceToLight = distance(lightPosition, gl_FragCoord.xy);

        // Calculate the distance from the current pixel to the center of the light
        float cur_gradient = max(0.0, 1.0 - worldDistanceToLight / 350.0);
        cur_gradient = quad_falloff(cur_gradient);

        // Apply the gradient as a mask to the texture color
        color_gradient += cur_gradient * lightsColor[i];
    }

    // Output the final color with the original alpha
    gl_FragColor = color * color_gradient;
}
