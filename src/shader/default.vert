#version 140

in vec3 position;

out vec3 pos;

out float z;

uniform mat4 vp;

void main() {

  gl_Position = vp * vec4(position, 1.0);

  z = gl_Position.z;

  pos = position;

}
