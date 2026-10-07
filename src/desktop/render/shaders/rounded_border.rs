use smithay::{
    backend::renderer::{
        element::{
            Element,
            Kind,
            Id,
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
        },
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


#[derive(Debug, Clone, Copy)]
pub struct RoundedBorderProgram {
    pub(crate) program: ShaderProgram,

    pub(crate) position_attribute: i32,
    pub(crate) damage_attribute: i32,

    pub(crate) projection_uniform: i32,

    pub(crate) window_position_uniform: i32,
    pub(crate) size_uniform: i32,
    
    pub(crate) radius_uniform: i32,
    pub(crate) border_width_uniform: i32,
    
    pub(crate) color_uniform: i32,
}

#[derive(Clone)]
pub struct RoundedBorder {
    pub(crate) inner: ShaderRenderElement,

    pub(crate) position: Point<i32, Physical>,
    pub(crate) size: Size<i32, Physical>,

    pub(crate) radius: f32,
    pub(crate) border_width: f32,

    pub(crate) color: [f32; 4],
    
    pub(crate) program: RoundedBorderProgram,
}

impl RoundedBorderProgram {
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

        let radius_uniform =
            program.uniform(
                renderer,
                "radius",
            )?;

        let border_width_uniform = 
            program.uniform(
                renderer,
                "border_width",
            )?;

        let color_uniform = 
            program.uniform(
                renderer,
                "color",
            )?;
        
        Ok(Self {
            program,

            position_attribute,
            damage_attribute,

            projection_uniform,

            window_position_uniform,
            size_uniform,
            
            radius_uniform,
            border_width_uniform,

            color_uniform,
        })
    }
}

impl RoundedBorder {
    pub fn new(
        program: RoundedBorderProgram,
        resources: SharedResources,
    ) -> Self {
        let inner =
            ShaderRenderElement::new(
                program.program.clone(),
                program.projection_uniform,
                program.position_attribute,
                program.damage_attribute,
                resources,
                Point::from((0, 0)),
                Size::from((0, 0)),
            );

        Self {
            inner,

            position: Point::from((
                0,
                0,
            )),

            size: Size::from((
                0,
                0,
            )),

            radius: 0.0,
            border_width: 0.0,

            color: [
                0.0,
                0.0,
                0.0,
                0.0,
            ],

            program,
        }
    }
    
    pub fn update(
        &mut self,
        position: Point<i32, Physical>,
        size: Size<i32, Physical>,
        radius: f32,
        border_width: f32,
        color: [f32; 4],
    ) {
        let geometry_changed =
            self.inner.set_geometry(
                position,
                size,
            );
        
        let visual_changed =
            self.position != position
            || self.size != size
            || self.radius != radius
            || self.border_width != border_width
            || self.color != color;

        self.position = position;
        self.size = size;

        self.radius = 
            radius.max(0.0);

        self.border_width =
            border_width.max(0.0);

        self.color = color;

        if geometry_changed
            || visual_changed
        {
            self.inner.commit_changed();
        }
    }

    pub(crate) fn src(
        &self
    ) -> Rectangle<f64, Buffer> {
        self.inner.src()
    }

    pub(crate) fn draw(
        &self,
        frame: &mut GlesFrame<'_, '_>,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
    ) -> Result<
        (),
        <GlesRenderer as RendererSuper>::Error,
    > {
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
                    p.border_width_uniform,
                    self.border_width,
                );

                gl.Uniform4f(
                    p.color_uniform,
                    self.color[0],
                    self.color[1],
                    self.color[2],
                    self.color[3],
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

impl Element for RoundedBorder {
    fn id(&self) -> &Id {
        self.inner.id()
    }

    fn current_commit(
        &self,
    ) -> CommitCounter {
        self.inner.current_commit()
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        self.inner.src()
    }

    fn transform(&self) -> Transform {
        Transform::Normal
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
        self.inner.opaque_regions(scale)
    }

    fn alpha(&self) -> f32 {
        self.color[3]
    }

    fn kind(&self) -> Kind {
        Kind::Unspecified
    }
}