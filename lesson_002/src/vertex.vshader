#version 300 es
in vec3 a_position;

uniform float uPointSize;
uniform float uAngle;

void main(void){
    gl_PointSize = uPointSize;
    // gl_Position = vec4(a_position, 1.0);
    gl_Position = vec4(
      cos(uAngle) * 0.8 + a_position.x,
      sin(uAngle) * 0.8 + a_position.x,
      a_position.z,
      1.0
    );
}
