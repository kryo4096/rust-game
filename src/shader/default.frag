#version 140

in vec3 world_pos;
in vec3 view_pos;

uniform vec4 fog_color;

out vec4 color;

void main() {

  if(world_pos.y > 50. + sin(world_pos.z)*0.4 + sin(world_pos.x)*0.4) {
    color = vec4(1.0,1.0,1.0,1.0);
  } else if (world_pos.y > 0.1) {
    color = vec4(0.2,0.2,0.2,1.0);
  } else {
    color = vec4(0.2,1.0,0.0,1.0);
  }

  float blend = pow(length(view_pos)/1000.0+1.0,-2.0);

  color = mix(fog_color,color,blend);

}
