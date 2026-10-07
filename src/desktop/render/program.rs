use std::ffi::CString;

use smithay::backend::renderer::{
    gles::{
        ffi,
        link_program,
        GlesRenderer,
    },
};

use super::{
    shader::Shader,
    shaders::rounded_border::RoundedBorderProgram,
    shaders::rounded_surface::RoundedSurfaceProgram,
};


#[derive(Debug, Clone, Copy)]
pub struct ShaderProgram {
    pub(crate) program: ffi::types::GLuint,
}

pub struct ShaderPrograms {
    pub(crate) rounded_border: RoundedBorderProgram,
    pub(crate) rounded_surface: RoundedSurfaceProgram,
}


impl ShaderProgram {
    pub fn new(
        renderer: &mut GlesRenderer,
        shader: &Shader
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let source = shader.source()?;

        let program = renderer.with_context(|gl| unsafe {
            link_program(
                gl,
                &source.vertex,
                &source.fragment
            )
            .map_err(|error| {
                eprintln!(
                    "SIOS: Shader link failed: {:?}",
                    error,
                );

                eprintln!(
                    "SIOS: Vertex shader:\n{}",
                    source.vertex,
                );

                eprintln!(
                    "SIOS: Fragment shader:\n{}",
                    source.fragment,
                );

                Box::<dyn std::error::Error>::from(error)
            })
        })??;

        Ok(Self {
            program,
        })
    }

    pub fn uniform(
        &self,
        renderer: &mut GlesRenderer,
        name: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let name = CString::new(name)?;

        let location =
            renderer.with_context(|gl| unsafe {
                gl.GetUniformLocation(
                    self.program,
                    name.as_ptr() as *const _,
                )
            })?;

        if location < 0 {
            return Err(
                format!(
                    "SIOS: Uniform '{}' not found",
                    name.to_string_lossy(),
                )
                .into(),
            );
        }

        Ok(location)
    }

    pub fn attribute(
        &self,
        renderer: &mut GlesRenderer,
        name: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let name = CString::new(name)?;

        let location =
            renderer.with_context(|gl| unsafe {
                gl.GetAttribLocation(
                    self.program,
                    name.as_ptr() as *const _,
                )
            })?;

        if location < 0 {
            return Err(
                format!(
                    "SIOS: Attribute '{}' not found.",
                    name.to_string_lossy(),
                )
                .into(),
            );
        }

        Ok(location)
    }
}

impl ShaderPrograms {
    pub fn new(
        renderer: &mut GlesRenderer,
        rounded_border_shader: &Shader,
        rounded_surface_shader: &Shader,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        
        let rounded_border_program =
            ShaderProgram::new(
                renderer,
                rounded_border_shader,
            )?;
        
        let rounded_border =
            RoundedBorderProgram::new(
                renderer,
                rounded_border_program,
            )?;
        
        let rounded_surface_program =
            ShaderProgram::new(
                renderer,
                rounded_surface_shader,
            )?;

        let rounded_surface =
            RoundedSurfaceProgram::new(
                renderer,
                rounded_surface_program,
            )?;

        Ok(Self {
            rounded_border,
            rounded_surface,
        })
    }
}