use smithay::backend::renderer::gles::GlesRenderer;

use super::{
    program::ShaderPrograms,
    shader::Shader,
};

pub struct ShaderCompositor {
    pub programs: ShaderPrograms,
}


impl ShaderCompositor {
    pub fn new(
        renderer: &mut GlesRenderer,
        rounded_border: &Shader,
        rounded_surface: &Shader,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let programs = 
            ShaderPrograms::new(
                renderer,
                rounded_border,
                rounded_surface,
            )?;

        Ok(Self {
            programs,
        })
    }
}