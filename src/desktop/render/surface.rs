use smithay::{
    backend::renderer::{
        element::{
            surface::render_elements_from_surface_tree,
            Element,
            Kind,
        },
        gles::GlesRenderer,
    },
    utils::{
        Physical,
        Point,
        Size,
    },
};

use crate::desktop::{
    render::{
        shaders::RoundedSurface,
        shader_compositor::ShaderCompositor,
        CustomRenderElement,
        RenderCache,
    },
    window::Window,
};


pub fn render(
    renderer: &mut GlesRenderer,
    compositor: &ShaderCompositor,
    cache: &mut RenderCache,
    window: &Window,

    position: Point<i32, Physical>,
    size: Size<i32, Physical>,
    output_size: Size<i32, Physical>,

    scale: f64,
) -> Result<
    Vec<CustomRenderElement>,
    Box<dyn std::error::Error>
> {
    let border =
        &window.appearance.border;

    let surface = 
        window
            .surface()
            .wl_surface();

    let surface_elements =
        render_elements_from_surface_tree(
            renderer,
            surface,
            (
                position.x,
                position.y,
            ),
            scale,
            window.appearance.opacity,
            Kind::Unspecified,
        );

    if border.radius <= 0.0
        && border.vignette <= 0.0
    {
        return Ok(
            surface_elements
                .into_iter()
                .map(
                    CustomRenderElement::Surface,
                )
                .collect()
        );
    }

    let mut elements =
        Vec::<CustomRenderElement>::with_capacity(
            surface_elements.len(),
        );

    for element in surface_elements {

        let rounded_surface =
            cache.surface(
                window.id(),
                element,
                compositor.programs.rounded_surface,
                border.radius,
                border.vignette,
            );

        elements.push(
            CustomRenderElement::rounded_surface(
                rounded_surface,
            ),
        );
    }

    Ok(elements)
}