use smithay::{
    backend::renderer::{
        element::{
            surface::render_elements_from_surface_tree,
            Kind,
        },
        gles::GlesRenderer,
        utils::with_renderer_surface_state,
    },
    utils::{
        Physical,
        Point,
        Size,
    },
};

use crate::desktop::{
    render::{
        border,
        surface as rsurface,
        CustomRenderElement,
        ShaderCompositor,
        RenderCache,
    },
    Desktop,
    Monitor,
};

pub fn build(
    renderer: &mut GlesRenderer,
    compositor: &ShaderCompositor,
    cache: &mut RenderCache,
    desktop: &Desktop,
    monitor: &Monitor,
) -> Result<Vec<CustomRenderElement>, Box<dyn std::error::Error>> {
    let mut elements =
        Vec::<CustomRenderElement>::new();

    let wallpaper = 
        desktop.renderer.wallpaper(monitor);

    elements.push(
        CustomRenderElement::Solid(
            wallpaper,
        ),
    );

    for layer in desktop.layers() {
        let layer_elements = 
            render_elements_from_surface_tree(
                renderer,
                layer.wl_surface(),
                (0, 0),
                monitor.scale,
                1.0,
                Kind::Unspecified,
            );

        for element in layer_elements {
            elements.push(
                CustomRenderElement::Surface(
                    element,
                ),
            );
        }
    }

    for window in desktop.mapped_windows() {
        let position =
            Point::<i32, Physical>::from((
                (
                    window.position().x as f64
                    * monitor.scale
                ).round() as i32,
                (
                    window.position().y as f64
                    * monitor.scale
                ).round() as i32,
            ));

        let surface = 
            window.surface().wl_surface();

        let Some(surface_size) =
            with_renderer_surface_state(
                surface,
                |state| state.surface_size(),
            ).flatten()
        else {
            continue;
        };

        let physical_size =
            Size::<i32, Physical>::from((
                (
                    surface_size.w as f64
                        * monitor.scale
                ).round() as i32,

                (
                    surface_size.h as f64
                        * monitor.scale
                ).round() as i32,
            ));

        let border_elements =
            border::render(
                compositor,
                cache,
                window,
                position,
                physical_size,
                monitor.resolution,
            )?;

        elements.extend(
            border_elements,
        );

        let surface_elements =
            rsurface::render(
                renderer,
                compositor,
                cache,
                window,
                position,
                physical_size,
                monitor.resolution,
                monitor.scale,
            )?;

        elements.extend(
            surface_elements,
        );
    }

    Ok(elements)
}