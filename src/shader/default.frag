#version 140

flat in vec4 face_color;
in vec3 view_pos;

uniform vec4 fog_color;

out vec4 color;


void main() {

  color = face_color;

  float fog_amount = 1.0;

  float blend = (1-pow(length(view_pos)/1000.0+1.0,-2.0))*fog_amount;

  color = mix(color,fog_color,blend);

}
