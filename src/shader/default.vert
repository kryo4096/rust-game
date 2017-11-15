#version 140

in vec3 position;

out vec3 world_pos;

out vec3 view_pos;

uniform mat4 model_m;
uniform mat4 view_m;
uniform mat4 projection_m;


void main() {

  vec4 world = model_m * vec4(position, 1.0);

  vec4 view = view_m * world;

  gl_Position = projection_m * view;

  world_pos = world.xyz;

  view_pos = view.xyz;

}
