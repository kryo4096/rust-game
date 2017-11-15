#version 140

in vec3 position;
in vec3 normal;

uniform mat4 model_m;
uniform mat4 view_m;
uniform mat4 projection_m;
uniform vec3 light;

out vec3 view_pos;

flat out vec4 face_color;

vec4 position_to_color(vec3 pos) {
  vec4 color;

  if(dot(normal , vec3(0.0,1.0,0.0)) > 0.6) {
    color = vec4(0.2+0.1*sin(pos.y)*cos(pos.x)*sin(pos.z),1.0,0.0,1.0);
  } else {
    color = vec4(0.2,0.2,0.2,1.0) * max(pos.y / 50.0,0.3);
  }

  return color;
}

void main() {

  vec4 world = model_m * vec4(position, 1.0);

  vec4 view = view_m * world;

  view_pos = view.xyz;

  float brightness = clamp(dot(normal,-light),0.4,1.0);

  face_color = position_to_color(world.xyz) * brightness;
  gl_Position = projection_m * view;

}
