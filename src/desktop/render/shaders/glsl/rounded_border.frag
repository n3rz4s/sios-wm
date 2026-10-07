#version 100

#extension GL_OES_standard_derivatives : enable

precision highp float;

uniform vec2 size;
uniform float radius;
uniform float border_width;
uniform vec4 color;

varying vec2 v_local_position;

float rounded_box(
    vec2 p,
    vec2 b,
    float r
)
{
    vec2 q =
        abs(p) - b + r;

    return
        length(max(q, 0.0))
        + min(max(q.x, q.y), 0.0)
        - r;
}

void main()
{
    vec2 center =
        size * 0.5;

    vec2 p =
        v_local_position
        - center;

    vec2 outer_half_size =
        size * 0.5;

    float outer_radius =
        max(
            radius,
            0.0
        );

    float outer_distance =
        rounded_box(
            p,
            outer_half_size,
            outer_radius
        );

    float inner_border =
        max(
            border_width,
            0.0
        );

    vec2 inner_half_size =
        max(
            outer_half_size
            - vec2(inner_border),
            vec2(0.0)
        );

    float inner_radius =
        max(
            outer_radius
            - inner_border,
            0.0
        );

    float inner_distance =
        rounded_box(
            p,
            inner_half_size,
            inner_radius
        );

    float outer_aa =
        max(
            fwidth(outer_distance),
            0.001
        );

    float inner_aa =
        max(
            fwidth(inner_distance),
            0.001
        );

    float outer_mask =
        1.0 -
        smoothstep(
            0.0,
            outer_aa,
            outer_distance
        );

    float inner_mask =
        1.0 -
        smoothstep(
            0.0,
            inner_aa,
            inner_distance
        );

    float border_mask =
        clamp(
            outer_mask
            - inner_mask,
            0.0,
            1.0
        );

    gl_FragColor =
        vec4(
            color.rgb,
            color.a
            * border_mask
        );
}