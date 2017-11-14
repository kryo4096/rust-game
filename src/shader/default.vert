#version 140

in vec3 position;

out vec3 world_pos;

out vec3 view_pos;

uniform mat4 model_m;
uniform mat4 view_projection_m;


void main() {

  gl_Position = view_projection_m * model_m * vec4(position, 1.0);

  world_pos = (model_m * vec4(position, 1.0)).xyz;

  view_pos = gl_Position.xyz;

}
