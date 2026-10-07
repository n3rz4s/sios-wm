use smithay::{
    backend::renderer::{
        element::{
            solid::{
                SolidColorBuffer,
                SolidColorRenderElement,
            },
            Kind,
        },
        Color32F,
    },
    utils::{
        Physical,
        Point,
        Scale,
    },
};

use super::monitor::Monitor;

pub struct DesktopRenderer {
    wallpaper: SolidColorBuffer,
}


impl DesktopRenderer {
    pub fn new() -> Self {
        Self {
            wallpaper: SolidColorBuffer::new(
                (1, 1),
                Color32F::new(0.08, 0.10, 0.14, 1.0),
            ),
        }
    }

    pub fn wallpaper(
        &self,
        monitor: &Monitor
    ) -> SolidColorRenderElement {
        SolidColorRenderElement::from_buffer(
            &self.wallpaper,
            Point::<i32, Physical>::from((0, 0)),
            Scale::from(monitor.scale),
            1.0,
            Kind::Unspecified,
        )
    }
}