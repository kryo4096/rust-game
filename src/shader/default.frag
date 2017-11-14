#version 140

in vec3 world_pos;
in vec3 view_pos;

out vec4 color;

void main() {

  if(world_pos.y > 50. + sin(world_pos.z)*0.4 + sin(world_pos.x)*0.4) {
    color = vec4(1.0,1.0,1.0,1.0);
  } else if (world_pos.y > 0.1) {
    color = vec4(0.2,0.2,0.2,1.0);
  } else {
    color = vec4(0.2,1.0,0.0,1.0);
  }

  float blend = pow(length(view_pos)/75.0+1.0,-2.0);

  vec4 fog = vec4(0.1,0.5,1.0,1.0);

  color = mix(fog,color,blend);

}
