#version 430

const int MAX_LIGHTS = 480;

uniform sampler2D textureSampler;
uniform vec2 screenSize;
uniform vec2 lightsPosition[MAX_LIGHTS];
uniform vec4 lightsColor[MAX_LIGHTS];
uniform int lightsAmount;
uniform float lightsRadius[MAX_LIGHTS];
uniform int lightsType[MAX_LIGHTS];

float quad_falloff(float x) {
    return max(1.0 / (-x / 2.0 + 1) - 1, 0);
}

void main() {
    vec2 uv = gl_FragCoord.xy / screenSize;
    vec3 color_gradient = vec3(0.0);
    vec4 color = texture(textureSampler, uv);

    for (int i = 0; i < lightsAmount; i++) {
        if (lightsType[i] == 1) {
            color_gradient += lightsColor[i].rgb * lightsColor[i].a;
        }
        else {
            // Calculate the normalized screen coordinates
            vec2 lightPosition = vec2(lightsPosition[i].x, -lightsPosition[i].y + screenSize.y);
            float worldDistanceToLight = distance(lightPosition, gl_FragCoord.xy);

            // Calculate the distance from the current pixel to the center of the light
            float cur_gradient = max(0.0, 1.0 - worldDistanceToLight / lightsRadius[i]);
            cur_gradient = quad_falloff(cur_gradient);

            // Apply the gradient as a mask to the texture color
            color_gradient += cur_gradient * lightsColor[i].rgb * lightsColor[i].a;
        }
    }
    // Output the final color with the original alpha
    gl_FragColor = color * vec4(color_gradient, 1.0);
}
