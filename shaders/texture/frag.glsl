#version 330 core

in vec3 ourColor;
in /* noperspective */ vec2 albedoTexCoord;

uniform sampler2D albedo;

out vec4 FragColor;

void main() {
    vec4 texColor = texture(albedo, albedoTexCoord);
    FragColor = texColor;
}
