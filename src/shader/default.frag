#version 140

in vec3 world_pos;
in vec3 view_pos;
flat in float brightness;
in vec3 o_normal;

uniform vec4 fog_color;
uniform vec3 light;

out vec4 color;


void main() {

  vec4 mat_color;

  if(world_pos.y > 50. + sin(world_pos.z)*0.4 + sin(world_pos.x)*0.4) {
    mat_color = vec4(1.0,1.0,1.0,1.0);
  } else if (world_pos.y > 0.1) {
    mat_color = vec4(0.2,0.2,0.2,1.0);
  } else {
    mat_color = vec4(0.2,1.0,0.0,1.0);
  }

  color = mat_color * brightness * 1.2;

  float fog_amount = 1.0;

  float blend = (1-pow(length(view_pos)/1000.0+1.0,-2.0))*fog_amount;

  color = mix(color,fog_color,blend);

}
