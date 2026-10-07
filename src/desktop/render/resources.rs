use std::{
    cell::RefCell,
    rc::Rc,
};

use smithay::{
    backend::renderer::gles::{
        ffi,
        GlesRenderer,
        GlesError,
    },
};


pub type SharedResources = Rc<RefCell<Resources>>;

pub struct Resources {
    pub(crate) quad_vbo: u32,
    pub(crate) damage_vbo: u32,

    pub(crate) vertices: Vec<f32>,
}

impl Resources {
    pub fn new(
        renderer: &mut GlesRenderer,
    ) -> Result<SharedResources, GlesError> {
        renderer.with_context(|gl| unsafe {
            let quad_vertices: [f32; 8] = [
                0.0, 0.0,
                1.0, 0.0,
                0.0, 1.0,
                1.0, 1.0,
            ];

            let mut quad_vbo = 0;
            let mut damage_vbo = 0;

            gl.GenBuffers(
                1,
                &mut quad_vbo,
            );

            gl.GenBuffers(
                1,
                &mut damage_vbo,
            );

            gl.BindBuffer(
                ffi::ARRAY_BUFFER,
                quad_vbo,
            );

            gl.BufferData(
                ffi::ARRAY_BUFFER,
                (
                    quad_vertices.len()
                    * std::mem::size_of::<f32>()
                ) as isize,
                quad_vertices.as_ptr() as *const _,
                ffi::STATIC_DRAW,
            );

            gl.BindBuffer(
                ffi::ARRAY_BUFFER,
                damage_vbo,
            );

            gl.BufferData(
                ffi::ARRAY_BUFFER,
                1,
                std::ptr::null(),
                ffi::STREAM_DRAW,
            );

            gl.BindBuffer(
                ffi::ARRAY_BUFFER,
                0,
            );

            Rc::new(
                RefCell::new(
                    Resources {
                        quad_vbo,
                        damage_vbo,
                        vertices: Vec::with_capacity(256),
                    },
                ),
            )
        })
    }

    pub fn destroy(
        &self,
        renderer: &mut GlesRenderer,
    ) -> Result<(), GlesError> {
        let quad_vbo = 
            self.quad_vbo;

        let damage_vbo = 
            self.damage_vbo;

        renderer.with_context(|gl| unsafe {
            gl.DeleteBuffers(
                1,
                &quad_vbo,
            );

            gl.DeleteBuffers(
                1,
                &damage_vbo,
            );
        })
    }
}