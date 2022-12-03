#version 450 core

in vec3 ourColor;
in vec2 albedoTexCoord;

uniform sampler2D albedo;

out vec4 FragColor;

void main() {
    // FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    FragColor = texture(albedo, albedoTexCoord);
}
