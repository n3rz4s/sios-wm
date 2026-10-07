use smithay::{
    backend::renderer::Color32F,
    utils::{
        Physical,
        Point,
        Size,
    },
};

use crate::desktop::{
    render::{
        shaders::RoundedBorder,
        shader_compositor::ShaderCompositor,
        CustomRenderElement,
        RenderCache,
    },
    window::Window,
};

pub fn render(
    compositor: &ShaderCompositor,
    cache: &mut RenderCache,
    window: &Window,
    position: Point<i32, Physical>,
    size: Size<i32, Physical>,
    output_size: Size<i32, Physical>,
) -> Result<Vec<CustomRenderElement>, Box<dyn std::error::Error>> {
    let border = &window.appearance.border;

    if !border.enabled {
        return Ok(Vec::new());
    }

    let width = if window.state.focused {
        border.focused_width
    } else {
        border.unfocused_width
    };

    if width == 0 {
        return Ok(Vec::new());
    }

    let color = if window.state.focused {
        border.focused_color
    } else {
        border.unfocused_color
    };

    let color = Color32F::new(
        color.r,
        color.g,
        color.b,
        color.a * border.opacity,
    );

    let width = width as i32;

    let window_width = size.w;
    let window_height = size.h;

    if border.radius <= 0.0 {
        return Ok(
            vec![
                CustomRenderElement::rect(
                    position,
                    Size::from((
                        window_width,
                        width,
                    )),
                    color,
                ),

                CustomRenderElement::rect(
                    Point::from((
                        position.x,
                        position.y
                            + window_height
                            - width,
                    )),
                    Size::from((
                        window_width,
                        width,
                    )),
                    color,
                ),

                CustomRenderElement::rect(
                    position,
                    Size::from((
                        width,
                        window_height,
                    )),
                    color,
                ),

                CustomRenderElement::rect(
                    Point::from((
                        position.x
                            + window_width
                            - width,
                        position.y
                    )),
                    Size::from((
                        width,
                        window_height,
                    )),
                    color,
                ),
            ]
        );
    }

    let rounded_border =
        cache.border(
            window.id(),
            compositor.programs.rounded_border,
            position,
            size,
            border.radius,
            width as f32,
            [
                color.r(),
                color.g(),
                color.b(),
                color.a(),
            ],
        );

    Ok(
        vec![
            CustomRenderElement::rounded_border(
                rounded_border,
            ),
        ]
    )
}