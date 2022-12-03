#version 450 core
layout (location = 0) in vec3 vPos;
layout (location = 1) in vec3 vColor;
layout (location = 2) in vec2 vAlbedoTexCoord;

out vec3 ourColor;
out vec2 albedoTexCoord;

uniform mat4 mvp;

void main() {
   gl_Position = mvp * vec4(vPos, 1.0);
   ourColor = vColor;
   albedoTexCoord = vAlbedoTexCoord;
}
