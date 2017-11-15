#version 140

in vec3 position;
in vec3 normal;

uniform mat4 model_m;
uniform mat4 view_m;
uniform mat4 projection_m;
uniform vec3 light;

out vec3 world_pos;
out vec3 view_pos;

out vec3 o_normal;

flat out float brightness;



void main() {

  vec4 world = model_m * vec4(position, 1.0);

  vec4 view = view_m * world;

  gl_Position = projection_m * view;

  world_pos = world.xyz;

  view_pos = view.xyz;

  brightness = clamp(dot(normal,-light),0.6,1.0);

  o_normal = normal;

}
