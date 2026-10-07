pub enum Shader {
    Basic(BasicShader),
    Custom(CustomShader),
}

pub enum BasicShader {
    RoundedBorder,
    RoundedSurface,
}

pub struct CustomShader {
    pub vertex: String,
    pub fragment: String,
}

pub struct ShaderSource {
    pub vertex: String,
    pub fragment: String,
}

impl Shader {

    pub fn from_api(
        shader: &sios_wm_shader_api::Shader,
    ) -> Self {
        match shader {
            sios_wm_shader_api::Shader::Basic(shader) => {
                match shader {
                    sios_wm_shader_api::BasicShader::RoundedBorder => {
                        Self::Basic(
                            BasicShader::RoundedBorder,
                        )
                    }

                    sios_wm_shader_api::BasicShader::RoundedSurface => {
                        Self::Basic(
                            BasicShader::RoundedSurface,
                        )
                    }
                }
            }

            sios_wm_shader_api::Shader::Custom(shader) => {
                Self::Custom(
                    CustomShader {
                        vertex: shader.vertex.clone(),
                        fragment: shader.fragment.clone(),
                    },
                )
            }
        }
    }

    pub fn basic(shader: BasicShader) -> Self {
        Self::Basic(shader)
    }

    pub fn custom(
        vertex: impl Into<String>,
        fragment: impl Into<String>,
    ) -> Self {
        Self::Custom(
            CustomShader {
                vertex: vertex.into(),
                fragment: fragment.into(),
            },
        )
    }

    pub fn source(
        &self
    ) -> Result<ShaderSource, Box<dyn std::error::Error>> {
        match self {
            Self::Basic(shader) => {
                shader.source()
            }

            Self::Custom(shader) => {
                Ok(ShaderSource {
                    vertex: shader.vertex.clone(),
                    fragment: shader.fragment.clone(),
                })
            }
        }
    }
}

impl BasicShader {
    pub fn source(
        &self,
    ) -> Result<ShaderSource, Box<dyn std::error::Error>> {
        match self {
            Self::RoundedBorder => {
                Ok(ShaderSource {
                    vertex: include_str!(
                        "shaders/glsl/rounded_border.vert",
                    )
                    .to_owned(),

                    fragment: include_str!(
                        "shaders/glsl/rounded_border.frag",
                    )
                    .to_owned(),
                })
            }

            Self::RoundedSurface => {
                Ok(ShaderSource {
                    vertex: include_str!(
                        "shaders/glsl/rounded_surface.vert",
                    )
                    .to_owned(),

                    fragment: include_str!(
                        "shaders/glsl/rounded_surface.frag",
                    )
                    .to_owned(),
                })
            }
        }
    }
}