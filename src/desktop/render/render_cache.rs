use std::collections::HashMap;
use std::rc::Rc;

use smithay::{
    backend::renderer::{
        element::{
            surface::WaylandSurfaceRenderElement,
            Element,
            Id,
        },
        gles::GlesRenderer,
    },
    utils::{
        Physical,
        Point,
        Size,
    },
};

use crate::desktop::WindowId;

use super::{
    SharedResources,
    shaders::{
        RoundedBorder,
        RoundedBorderProgram,
        RoundedSurface,
        RoundedSurfaceProgram,
    },
};


pub struct RenderCache {
    resources: SharedResources,

    border: HashMap<WindowId, RoundedBorder>,
    surface:
        HashMap<
           (WindowId, Id),
           RoundedSurface,
        >,
}


impl RenderCache {
    pub fn new(
        resources: SharedResources,
    ) -> Self {
        Self {
            resources,

            border: HashMap::new(),
            surface: HashMap::new(),
        }
    }

    pub fn border(
        &mut self,
        window_id: WindowId,
        program: RoundedBorderProgram,
        position: Point<i32, Physical>,
        size: Size<i32, Physical>,
        radius: f32,
        border_width: f32,
        color: [f32; 4],
    ) -> RoundedBorder {
        let resources = 
            self.resources.clone();
        
        let element =
            self.border
                .entry(window_id)
                .or_insert_with(|| {
                    RoundedBorder::new(
                        program,
                        resources,
                    )
                });

        element.update(
            position,
            size,
            radius,
            border_width,
            color,
        );
        
        element.clone()
    }

    pub fn surface(
        &mut self,
        window_id: WindowId,
        surface:
            WaylandSurfaceRenderElement<GlesRenderer>,
        program: RoundedSurfaceProgram,
        radius: f32,
        vignette: f32,
    ) -> RoundedSurface {

        let surface =
            Rc::new(surface);

        let surface_id =
            surface.id().clone();

        let resources =
            self.resources.clone();

        let element =
            self.surface
                .entry(
                    (
                        window_id,
                        surface_id,
                    ),
                )
                .or_insert_with(|| {
                    RoundedSurface::new(
                        program,
                        resources,
                        surface.clone(),
                    )
                });

        element.update(
            surface.clone(),
            radius,
            vignette,
        );

        element.clone()
    }

    pub fn remove_window(
        &mut self,
        window_id: WindowId,
    ) {
        self.border.remove(&window_id);
        
        self.surface.retain(
            |(id, _),
            _| *id != window_id,
        );
    }
}