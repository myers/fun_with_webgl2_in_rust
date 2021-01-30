#version 300 es
in vec3 a_position;

uniform float uPointSize;
uniform float uAngle;

void main(void){
    gl_PointSize = uPointSize;
    gl_Position = vec4(a_position, 1.0);
}
