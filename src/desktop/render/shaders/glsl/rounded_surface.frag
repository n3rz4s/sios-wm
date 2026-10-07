#version 100

precision highp float;

uniform sampler2D surface_texture;

uniform vec2 size;
uniform float radius;
uniform float vignette;
uniform float alpha;
uniform int y_inverted;

varying vec2 v_local_position;
varying vec2 v_texcoord;

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
    vec2 uv =
        v_texcoord;

    if (y_inverted != 0)
    {
        uv.y =
            1.0 - uv.y;
    }

    vec4 pixel =
        texture2D(
            surface_texture,
            uv
        );

    vec2 center =
        size * 0.5;

    vec2 p =
        v_local_position
        - center;

    vec2 half_size =
        size * 0.5;

    float r = min(
        max(radius, 0.0),
        min(half_size.x, half_size.y)
    );

    float distance =
        rounded_box(
            p,
            half_size,
            r
        );

    float mask =
        1.0 -
        smoothstep(
            -1.0,
            1.0,
            distance
        );

    float edge =
        min(
            min(
                v_local_position.x,
                size.x - v_local_position.x
            ),
            min(
                v_local_position.y,
                size.y - v_local_position.y
            )
        );

    float vignette_factor =
        1.0;

    if (vignette > 0.0)
    {
        vignette_factor =
            smoothstep(
                0.0,
                vignette,
                edge
            );
    }

    pixel.a *=
        mask
        * vignette_factor
        * alpha;

    gl_FragColor =
        pixel;
}