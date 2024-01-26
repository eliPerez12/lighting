#version 330

uniform sampler2D textureSampler;                  // Input texture
uniform vec2 screenSize;                           // 

const int MAX_LIGHTS = 3;

struct Light {
    vec2 position;
    float radius;
    vec3 color;
};

Light default_light(vec2 pos, vec3 color) {
    return Light(pos, 250.0, color);
}

float quad_falloff(float x) {
    return 1.0/(-x/2.0 + 1)-1;
}

void main() {

    Light lights[MAX_LIGHTS] = {
        default_light(vec2(200.0, 200.0), vec3(1.0, 0.2, 0.2)),
        default_light(vec2(400.0, 400.0), vec3(1.0, 1.0, 0.2)),
        default_light(vec2(50.0, 50.0), vec3(0.2, 0.2,1.0)),
        };

    vec2 uv = gl_FragCoord.xy / screenSize;
    vec3 color_gradient = vec3(0.0, 0.0, 0.0);
    vec3 color = texture2D(textureSampler, uv).rgb;
    
    for (int i; i < MAX_LIGHTS; i++) {
        Light light0 = lights[i];
        // Calculate the normalized screen coordinates
        vec2 lightPosition = vec2(light0.position.x, -light0.position.y + screenSize.y);
        float worldDistanceToLight = distance(lightPosition, gl_FragCoord.xy);

        // Calculate the distance from the current pixel to the center of the light
        float cur_gradient = 1 - worldDistanceToLight / light0.radius;
        cur_gradient = quad_falloff(cur_gradient);

        // Apply the gradient as a mask to the texture color
        if (cur_gradient > 0) {
            color_gradient += cur_gradient * light0.color;
        }
    }

    // Output the final color with the original alpha
    gl_FragColor = vec4(color * color_gradient, 1.0);
}
