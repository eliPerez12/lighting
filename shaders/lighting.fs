#version 430

const int MAX_LIGHTS = 480;

uniform sampler2D textureSampler;
uniform vec2 screenSize;
uniform vec2 lightsPosition[MAX_LIGHTS];
uniform vec4 lightsColor[MAX_LIGHTS];
uniform int lightsAmount;
uniform float lightsRadius[MAX_LIGHTS];
uniform int lightsType[MAX_LIGHTS];

const int AMBIENT_LIGHT = 1;
const int RADIAL_LIGHT = 0;

void main() {
    vec2 uv = gl_FragCoord.xy / screenSize;
    vec3 color_gradient = vec3(0.0);
    vec4 color = texture(textureSampler, uv);
    float falloffFactor;

    for (int i = 0; i < lightsAmount; i++) {
        float lightAlpha = lightsColor[i].a;
        if (lightsType[i] == AMBIENT_LIGHT) {
            color_gradient += lightsColor[i].rgb * lightAlpha;
        }
        else {
            float curveAmount = 1.5;
            // Calculate the normalized screen coordinates
            vec2 lightPosition = vec2(lightsPosition[i].x, -lightsPosition[i].y + screenSize.y);
            float worldDistanceToLight = distance(lightPosition, gl_FragCoord.xy);

            // Calculate the distance from the current pixel to the center of the light
            falloffFactor = 1.0 / (-max(0.0, 1.0 - worldDistanceToLight / lightsRadius[i]) / 2.0 + 1.0);
            float cur_gradient = max(0.0, pow((falloffFactor - 1.0), curveAmount));

            // Apply the gradient as a mask to the texture color
            color_gradient += cur_gradient * lightsColor[i].rgb * lightAlpha;
        }
    }

    // Output the final color with the original alpha
    gl_FragColor = color * vec4(color_gradient, 1.0);
}