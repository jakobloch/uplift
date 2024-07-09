#version 450

layout(location = 0) in vec3 fragColor;

layout(location = 0) out vec4 outColor;

/**
 * Fragment shader that sets the output color to the input color with an alpha value of 1.0.
 */
void main() {
    outColor = vec4(fragColor, 1.0);
}
