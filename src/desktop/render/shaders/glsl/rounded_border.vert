#version 100

precision highp float;

attribute vec2 position;
attribute vec4 damage;

uniform mat3 projection;
uniform vec2 window_position;
uniform vec2 size;

varying vec2 v_local_position;

void main()
{
    vec2 local_position =
        damage.xy
        + position * damage.zw;

    v_local_position =
        local_position;

    vec2 screen_position =
        window_position
        + local_position;

    gl_Position =
        vec4(
            (projection * vec3(
                screen_position,
                1.0
            )).xy,
            0.0,
            1.0
        );
}