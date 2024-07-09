/**
 * @file shader.vert
 * @brief This file contains the vertex shader code for rendering objects.
 *
 * The vertex shader takes in the position and color attributes of each vertex and
 * calculates the final position of the vertex in clip space. It also passes the color
 * attribute to the fragment shader for further processing.
 */

#version 450

layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec3 inColor;

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = vec4(inPosition, 0.0, 1.0);
    fragColor = inColor;
}
