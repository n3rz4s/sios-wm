use std::rc::Rc;

use smithay::{
    backend::renderer::{
        element::{
            surface::{
                WaylandSurfaceRenderElement,
                WaylandSurfaceTexture,
            },
            Element,
            Id,
            Kind,
            RenderElement,
        },
        gles::{
            ffi,
            GlesRenderer,
            GlesFrame,
            GlesError,
        },
        utils::{
            CommitCounter,
            OpaqueRegions,
            DamageSet,
        },
        Texture,
        RendererSuper,
    },
    utils::{
        Buffer,
        Physical,
        Point,
        Rectangle,
        Scale,
        Size,
        Transform,
    },
};


use super::super::{
    ShaderRenderElement,
    SharedResources,
    elements::CustomRenderElement,
    program::ShaderProgram,
};

#[derive(Clone)]
pub struct RoundedSurface {
    pub(crate) inner: ShaderRenderElement,

    pub(crate) surface:
        Rc<WaylandSurfaceRenderElement<GlesRenderer>>,

    pub(crate) radius: f32,
    pub(crate) vignette: f32,

    pub(crate) program: RoundedSurfaceProgram,
}

#[derive(Debug, Clone, Copy)]
pub struct RoundedSurfaceProgram {
    pub(crate) program: ShaderProgram,

    pub(crate) position_attribute: i32,
    pub(crate) damage_attribute: i32,

    pub(crate) projection_uniform: i32,

    pub(crate) window_position_uniform: i32,
    pub(crate) size_uniform: i32,

    pub(crate) texture_uniform: i32,

    pub(crate) radius_uniform: i32,
    pub(crate) vignette_uniform: i32,

    pub(crate) alpha_uniform: i32,
    pub(crate) y_inverted_uniform: i32,
}


impl RoundedSurfaceProgram {
    pub fn new(
        renderer: &mut GlesRenderer,
        program: ShaderProgram,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let position_attribute =
            program.attribute(
                renderer,
                "position",
            )?;

        let damage_attribute =
            program.attribute(
                renderer,
                "damage",
            )?;

        let projection_uniform =
            program.uniform(
                renderer,
                "projection",
            )?;

        let window_position_uniform =
            program.uniform(
                renderer,
                "window_position",
            )?;

        let size_uniform =
            program.uniform(
                renderer,
                "size",
            )?;

        let texture_uniform =
            program.uniform(
                renderer,
                "surface_texture",
            )?;

        let radius_uniform =
            program.uniform(
                renderer,
                "radius",
            )?;

        let vignette_uniform =
            program.uniform(
                renderer,
                "vignette",
            )?;

        let alpha_uniform =
            program.uniform(
                renderer,
                "alpha",
            )?;

        let y_inverted_uniform =
            program.uniform(
                renderer,
                "y_inverted",
            )?;

        Ok(Self {
            program,

            position_attribute,
            damage_attribute,

            projection_uniform,

            window_position_uniform,
            size_uniform,

            texture_uniform,

            radius_uniform,
            vignette_uniform,

            alpha_uniform,
            y_inverted_uniform,
        })
    }
}

impl RoundedSurface {
    pub fn new(
        program: RoundedSurfaceProgram,
        resources: SharedResources,
        surface:
            Rc<WaylandSurfaceRenderElement<GlesRenderer>>,
    ) -> Self {
        let geometry =
            surface.geometry(
                Scale::from(1.0),
            );

        let texture = match surface.texture() {
            WaylandSurfaceTexture::Texture(texture) => Some(texture.clone()),
            WaylandSurfaceTexture::SolidColor(_) => None,
        };

        let inner =
            ShaderRenderElement::new(
                program.program.clone(),
                program.projection_uniform,
                program.position_attribute,
                program.damage_attribute,
                resources,
                geometry.loc,
                geometry.size,
            );

        Self {
            inner,

            surface,

            radius: 0.0,
            vignette: 0.0,

            program,
        }
    }

    pub fn update(
        &mut self,
        surface:
            Rc<WaylandSurfaceRenderElement<GlesRenderer>>,
        radius: f32,
        vignette: f32,
    ) {
        let old_surface_commit =
            self.surface.current_commit();

        let new_surface_commit =
            surface.current_commit();
        
        let geometry =
            surface.geometry(
                Scale::from(1.0),
            );

        let geometry_changed =
            self.inner.set_geometry(
                geometry.loc,
                geometry.size,
            );

        let visual_changed =
            self.radius != radius
            || self.vignette != vignette;

        let surface_changed =
            old_surface_commit
            != new_surface_commit;

        self.surface = surface;

        self.radius = radius.max(0.0);
        self.vignette = vignette.max(0.0);

        if geometry_changed
            || visual_changed
            || surface_changed
        {
            self.inner.commit_changed();
        }
    }

    pub(crate) fn src(
        &self
    ) -> Rectangle<f64, Buffer> {
        self.surface.src()
    }

    pub(crate) fn draw(
        &self,

        frame: 
            &mut GlesFrame<'_, '_>,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), GlesError> {
        let texture =
            match self.surface.texture() {
                WaylandSurfaceTexture::Texture(texture) => {
                    texture
                }

                WaylandSurfaceTexture::SolidColor(_) => {
                    return <
                        WaylandSurfaceRenderElement<GlesRenderer>
                        as RenderElement<GlesRenderer>
                    >::draw(
                        &self.surface,
                        frame,
                        src,
                        dst,
                        damage,
                        opaque_regions,
                    );
                }
            };

        self.inner.draw_with_setup(
            frame,
            src,
            dst,
            damage,
            |gl| unsafe {
                let p =
                    &self.program;
                
                gl.Uniform2f(
                    p.window_position_uniform,
                    dst.loc.x as f32,
                    dst.loc.y as f32,
                );

                gl.Uniform2f(
                    p.size_uniform,
                    dst.size.w as f32,
                    dst.size.h as f32,
                );

                gl.Uniform1f(
                    p.radius_uniform,
                    self.radius,
                );

                gl.Uniform1f(
                    p.vignette_uniform,
                    self.vignette,
                );
                
                gl.Uniform1f(
                    p.alpha_uniform,
                    self.surface.alpha(),
                );

                gl.Uniform1i(
                    p.y_inverted_uniform,
                    if texture.is_y_inverted() {
                        1
                    } else {
                        0
                    },
                );

                gl.ActiveTexture(
                    ffi::TEXTURE0,
                );

                gl.BindTexture(
                    ffi::TEXTURE_2D,
                    texture.tex_id(),
                );

                gl.Uniform1i(
                    p.texture_uniform,
                    0,
                );

                gl.TexParameteri(
                    ffi::TEXTURE_2D,
                    ffi::TEXTURE_MIN_FILTER,
                    ffi::LINEAR as i32,
                );

                gl.TexParameteri(
                    ffi::TEXTURE_2D,
                    ffi::TEXTURE_MAG_FILTER,
                    ffi::LINEAR as i32,
                );

                gl.TexParameteri(
                    ffi::TEXTURE_2D,
                    ffi::TEXTURE_WRAP_S,
                    ffi::CLAMP_TO_EDGE as i32,
                );
                
                gl.TexParameteri(
                    ffi::TEXTURE_2D,
                    ffi::TEXTURE_WRAP_T,
                    ffi::CLAMP_TO_EDGE as i32,
                );

                gl.Enable(
                    ffi::BLEND,
                );

                gl.BlendFuncSeparate(
                    ffi::SRC_ALPHA,
                    ffi::ONE_MINUS_SRC_ALPHA,
                    ffi::ONE,
                    ffi::ONE_MINUS_SRC_ALPHA,
                );
            }
        )
    }
}


impl Element for RoundedSurface {
    fn id(&self) -> &Id {
        self.inner.id()
    }

    fn current_commit(&self) -> CommitCounter {
        self.inner.current_commit()
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        self.surface.src()
    }

    fn transform(&self) -> Transform {
        self.surface.transform()
    }

    fn geometry(
        &self,
        scale: Scale<f64>,
    ) -> Rectangle<i32, Physical> {
        self.inner.geometry(scale)
    }

    fn opaque_regions(
        &self,
        scale: Scale<f64>,
    ) -> OpaqueRegions<i32, Physical> {
        self.surface.opaque_regions(scale)
    }

    fn alpha(&self) -> f32 {
        self.surface.alpha()
    }

    fn kind(&self) -> Kind {
        self.surface.kind()
    }
}