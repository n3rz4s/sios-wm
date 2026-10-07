use std::cell::RefCell;

use smithay::{
    backend::allocator::Fourcc,
    backend::renderer::{
        element::{
            memory::{
                MemoryRenderBuffer,
                MemoryRenderBufferRenderElement,
            },
            Element,
            Id,
            Kind,
            RenderElement,
            UnderlyingStorage,
        },
        gles::{
            GlesFrame,
            GlesRenderer,
            GlesTexture,
        },
        utils::{
            CommitCounter,
            DamageSet,
            OpaqueRegions,
        },
        Frame,
        Texture,
        RendererSuper,
    },
    utils::{
        Buffer,
        Logical,
        Physical,
        Point,
        Size,
        Rectangle,
        Scale,
        Transform,
    },
};

pub struct CursorElement {
    buffer: MemoryRenderBuffer,
}


impl CursorElement {
    pub fn new(
        pixels: &[u8],
        width: u32,
        height: u32,
    ) -> Self {
        let buffer =
            MemoryRenderBuffer::from_slice(
                pixels,
                Fourcc::Argb8888,
                (
                    width as i32,
                    height as i32,
                ),
                1,
                Transform::Normal,
                None,
            );

        Self {
            buffer,
        }
    }

    pub fn update_position(
        &self,
        renderer: &mut GlesRenderer,
        position: Point<f64, Physical>,
    ) -> Result<
        MemoryRenderBufferRenderElement<GlesRenderer>,
        <GlesRenderer as RendererSuper>::Error
    > {
        MemoryRenderBufferRenderElement::from_buffer(
            renderer,
            position,
            &self.buffer,
            None,
            None,
            None,
            Kind::Cursor,
        )
    }
}