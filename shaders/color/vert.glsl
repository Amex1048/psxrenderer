#version 450 core
layout (location = 0) in vec3 vPos;
layout (location = 1) in vec3 vColor;
layout (location = 2) in vec2 vAlbedoTexCoord;

out vec3 ourColor;
out vec2 albedoTexCoord;

uniform mat4 mvp;
uniform vec2 renderResolution;

// vertex: the vertex to be snapped (needs to be in projection-space)
// resolution: the lower resolution, e.g. if my screen resolution is 1280x720, I might choose 640x320
vec4 snap(vec4 vertex, vec2 resolution)
{
    vec4 snappedPos = vertex;
    snappedPos.xyz = vertex.xyz / vertex.w; // convert to normalised device coordinates (NDC)
    snappedPos.xy = floor(resolution * snappedPos.xy) / resolution; // snap the vertex to the lower-resolution grid
    snappedPos.xyz *= vertex.w; // convert back to projection-space
    return snappedPos;
}

void main() {
   vec4 position = snap(mvp * vec4(vPos, 1.0), renderResolution);
   gl_Position = position;
   ourColor = vColor;
   albedoTexCoord = vAlbedoTexCoord;
}
