#version 330 core

in vec3 ourColor;
in vec2 albedoTexCoord;

// uniform sampler2D albedo;
uniform vec3 color;

out vec4 FragColor;

void main() {
    FragColor = vec4(color, 1.0f);
    // FragColor = texture(albedo, albedoTexCoord);
}
