#version 140

in vec3 position;

out vec3 pos;

uniform mat4 matrix;

void main() {

  gl_Position = matrix * vec4(position, 1.0);

  pos = position;

}
