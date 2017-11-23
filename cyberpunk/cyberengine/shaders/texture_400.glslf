#version 400

uniform sampler2D t_Texture;

in vec4 v_Color;
in vec2 v_Uv;
out vec4 Target0;

void main() {
    vec4 tx = texture(t_Texture, v_Uv).rgba;

    Target0 = tx;
}