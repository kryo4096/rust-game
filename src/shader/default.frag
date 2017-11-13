#version 140

in vec3 pos;
out vec4 color;

void main() {
  color = vec4( vec3(sin(10*pos.x)/2+0.5, cos(10*pos.y)/2+0.5, sin(10*pos.z)/2+0.5) / 10.0, 1.0);
}
