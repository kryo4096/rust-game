#version 140

in vec3 pos;
out vec4 color;

in float z;

void main() {

  if(pos.y > 50.0) {
    color = vec4(1.0,1.0,1.0,1.0);
  } else if (pos.y > 0.0) {
    color = vec4(0.2,0.2,0.2,1.0)* (1.5 - pos.y/50);
  } else {
    color = vec4(0.2,1.0,0.0,1.0);
  }

  color *= 1 - z/300;

}
