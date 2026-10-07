use std::rc::Rc;

use smithay::{
    backend::{
        allocator::Fourcc,
        renderer::{
            element::memory::MemoryRenderBufferRenderElement,
            gles::GlesRenderer,
            ImportMem,
        },
    },
    utils::{
        Physical,
        Point,
    },
};

use super::{
    super::Cursor,
    CursorElement,
};

pub struct CursorRenderer {
    element: Option<Rc<CursorElement>>,

    loaded_key: Option<(String, u32, String)>,
}

impl CursorRenderer {
    pub fn new() -> Self {
        Self {
            element: None,
            loaded_key: None,
        }
    }

    pub fn sync(
        &mut self,
        renderer: &mut GlesRenderer,
        cursor: &Cursor,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let Some(image) = cursor.image.as_ref() else {
            self.element = None;
            self.loaded_key = None;
            return Ok(());
        };

        let key = (
            cursor.theme.clone(),
            cursor.size,
            cursor.icon.clone(),
        );

        if self.element.is_some()
            && self.loaded_key.as_ref() == Some(&key)
        {
            return Ok(());
        }

        eprintln!(
            "SIOS: Uploading cursor to GPU: {}x{}",
            image.width,
            image.height,
        );

        let element = CursorElement::new(
            &image.pixels,
            image.width,
            image.height,
        );

        self.element = Some(
            Rc::new(element),
        );

        self.loaded_key = Some(key);

        Ok(())
    }

    pub fn element(
        &self,
        renderer: &mut GlesRenderer,
        cursor: &Cursor
    ) -> Result<
        Option<MemoryRenderBufferRenderElement<GlesRenderer>>,
        Box<dyn std::error::Error>
    > {
        let Some(element) = self.element.as_ref() else {
            return Ok(None);
        };

        let Some(image) = cursor.image.as_ref() else {
            return Ok(None);
        };

        let position = cursor.position();

        let location =
            Point::<f64, Physical>::from((
                position.x
                    - image.hotspot_x as f64,

                position.y 
                    - image.hotspot_y as f64,
            ));

        let render_element = element.update_position(
            renderer,
            location,
        )?;

        Ok(Some(render_element))
    }
}