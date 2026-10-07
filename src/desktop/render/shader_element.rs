use std::{
    cell::RefCell,
    rc::Rc,
};

use smithay::{
    backend::renderer::{
        element::{
            RenderElement,
            Element,
            Id,
            Kind,
            UnderlyingStorage,
        },
        gles::{
            ffi,
            Capability,
            GlesRenderer,
            GlesError,
            GlesFrame,
        },
        utils::{
            CommitCounter,
            OpaqueRegions,
        },
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

use super::{
    ShaderProgram,
    Resources,
};

struct GlState {
    program: i32,
    array_buffer: i32,

    blend_enabled: bool,

    blend_src_rgb: i32,
    blend_dst_rgb: i32,
    blend_src_alpha: i32,
    blend_dst_alpha: i32,

    active_texture: i32,
    texture_0: i32,
}

#[derive(Clone)]
pub struct ShaderRenderElement {
    pub(crate) id: Id,
    
    pub(crate) position: Point<i32, Physical>,
    pub(crate) size: Size<i32, Physical>,

    pub(crate) commit: CommitCounter,

    pub(crate) program: ShaderProgram,

    pub(crate) projection_uniform: i32,
    pub(crate) position_attribute: i32,
    pub(crate) damage_attribute: i32,

    pub(crate) resources: Rc<RefCell<Resources>>,
}

impl ShaderRenderElement {
    pub fn new(
        program: ShaderProgram,
        projection_uniform: i32,
        position_attribute: i32,
        damage_attribute: i32,
        resources: Rc<RefCell<Resources>>,
        position: Point<i32, Physical>,
        size: Size<i32, Physical>,
    ) -> Self {
        let mut element = Self {
            id: Id::new(),

            position,
            size,

            commit: CommitCounter::default(),

            program,

            projection_uniform,
            position_attribute,
            damage_attribute,

            resources,
        };

        element.commit.increment();

        element
    }

    pub fn damage_all(&mut self) {
        self.commit.increment();
    }

    pub fn commit_changed(&mut self) {
        self.commit.increment();
    }

    pub fn set_geometry(
        &mut self,
        position: Point<i32, Physical>,
        size: Size<i32, Physical>,
    ) -> bool {
        if self.position == position
            && self.size == size
        {
            return false;
        }

        self.position = position;
        self.size = size;

        self.commit.increment();

        true
    }


    pub(crate) fn draw_with_setup<F>(
        &self,
        frame: &mut GlesFrame<'_, '_>,
        _src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        setup: F
    ) -> Result<(), GlesError>
    where
        F: FnOnce(&ffi::Gles2),
    {
        if damage.is_empty() {
            return Ok(());
        }

        let supports_instancing =
            frame
                .capabilities()
                .contains(
                    &Capability::Instancing,
                );
        
        let projection =
            *frame.projection();

        let mut damage_vertices =
            Vec::with_capacity(
                damage.len() * 4,
            );

        let dst_size =  
            dst.size.to_point();
        
        for damage_rect in damage {
            let x0 = damage_rect 
                .loc
                .x
                .saturating_sub(2)
                .clamp(
                    0,
                    dst_size.x,
                );

            let y0 = damage_rect
                .loc
                .y
                .saturating_sub(2)
                .clamp(
                    0,
                    dst_size.y,
                );

            let x1 = damage_rect
                .loc
                .x
                .saturating_add(
                    damage_rect.size.w,
                )
                .saturating_add(2)
                .clamp(
                    0,
                    dst_size.x,
                );

            let y1 = damage_rect
                .loc
                .y
                .saturating_add(
                    damage_rect.size.h,
                )
                .saturating_add(2)
                .clamp(
                    0,
                    dst_size.y,
                );

            if x1 <= 0
                || y1 <= 0
            {
                continue;
            }

            damage_vertices
                .extend_from_slice(
                    &[
                        x0 as f32,
                        y0 as f32,
                        (x1 - x0) as f32,
                        (y1 - y0) as f32,
                    ],
                );
        }

        let damage_count = 
            damage_vertices.len() / 4;

        if damage_count == 0 {
            return Ok(())
        }

        frame.with_context(
            |gl| unsafe {
                self.draw_gl(
                    gl,
                    projection,
                    &damage_vertices,
                    damage_count,
                    supports_instancing,
                    setup,
                )
            },
        )
    }

    unsafe fn draw_gl<F>(
        &self,
        gl: &ffi::Gles2,
        projection: [f32; 9],
        damage_vertices: &[f32],
        damage_count: usize,
        supports_instancing: bool,
        setup: F,
    )
    where 
        F: FnOnce(&ffi::Gles2),
    {
        let state =
            Self::save_gl_state(gl);

        let resources = self.resources
            .borrow();

        gl.UseProgram(
            self.program.program,
        );

        gl.UniformMatrix3fv(
            self.projection_uniform,
            1,
            ffi::FALSE,
            projection.as_ptr(),
        );

        setup(gl);

        gl.BindBuffer(
            ffi::ARRAY_BUFFER,
            resources.quad_vbo,
        );

        gl.EnableVertexAttribArray(
            self.position_attribute as u32,
        );

        gl.VertexAttribPointer(
            self.position_attribute as u32,
            2,
            ffi::FLOAT,
            ffi::FALSE,
            0,
            std::ptr::null(),
        );

        gl.BindBuffer(
            ffi::ARRAY_BUFFER,
            resources.damage_vbo,
        );

        gl.BufferData(
            ffi::ARRAY_BUFFER,
            (
                damage_vertices.len()
                * std::mem::size_of::<f32>()
            ) as isize,
            damage_vertices.as_ptr()
                as *const _,
            ffi::STREAM_DRAW,
        );

        gl.EnableVertexAttribArray(
            self.damage_attribute as u32,
        );

        gl.VertexAttribPointer(
            self.damage_attribute as u32,
            4,
            ffi::FLOAT,
            ffi::FALSE,
            0,
            std::ptr::null(),
        );

        if supports_instancing {
            gl.VertexAttribDivisor(
                self.damage_attribute as u32,
                1,
            );

            gl.DrawArraysInstanced(
                ffi::TRIANGLE_STRIP,
                0,
                4,
                damage_count as i32,
            );

            gl.VertexAttribDivisor(
                self.damage_attribute as u32,
                0,
            );
        } else {
            gl.VertexAttribDivisor(
                self.position_attribute as u32,
                0,
            );

            gl.VertexAttribDivisor(
                self.damage_attribute as u32,
                0,
            );

            for i in 0..damage_count {
                let offset =
                    i * 4 * std::mem::size_of::<f32>();

                gl.VertexAttribPointer(
                    self.damage_attribute as u32,
                    4,
                    ffi::FLOAT,
                    ffi::FALSE,
                    0,
                    offset as *const _,
                );

                gl.DrawArrays(
                    ffi::TRIANGLE_STRIP,
                    0,
                    4,
                );
            }
        }

        gl.VertexAttribDivisor(
            self.position_attribute as u32,
            0,
        );

        gl.VertexAttribDivisor(
            self.damage_attribute as u32,
            0,
        );

        gl.DisableVertexAttribArray(
            self.position_attribute as u32,
        );

        gl.DisableVertexAttribArray(
            self.damage_attribute as u32,
        );

        Self::restore_gl_state(
            gl,
            &state,
        );
    }

    unsafe fn save_gl_state(
        gl: &ffi::Gles2
    ) -> GlState {
        let mut program = 0i32;
        let mut array_buffer = 0i32;

        gl.GetIntegerv(
            ffi::CURRENT_PROGRAM,
            &mut program,
        );

        gl.GetIntegerv(
            ffi::ARRAY_BUFFER_BINDING,
            &mut array_buffer,
        );

        let blend_enabled =
            gl.IsEnabled(
                ffi::BLEND,
            ) != 0;

        let mut blend_src_rgb = 0i32;
        let mut blend_dst_rgb = 0i32;
        let mut blend_src_alpha = 0i32;
        let mut blend_dst_alpha = 0i32;

        gl.GetIntegerv(
            ffi::BLEND_SRC_RGB,
            &mut blend_src_rgb,
        );

        gl.GetIntegerv(
            ffi::BLEND_DST_RGB,
            &mut blend_dst_rgb,
        );

        gl.GetIntegerv(
            ffi::BLEND_SRC_ALPHA,
            &mut blend_src_alpha,
        );

        gl.GetIntegerv(
            ffi::BLEND_DST_ALPHA,
            &mut blend_dst_alpha,
        );

        let mut active_texture = 0i32;
        let mut texture_0 = 0i32;

        gl.GetIntegerv(
            ffi::ACTIVE_TEXTURE,
            &mut active_texture,
        );

        gl.GetIntegerv(
            ffi::TEXTURE_BINDING_2D,
            &mut texture_0,
        );

        GlState {
            program,
            array_buffer,

            blend_enabled,

            blend_src_rgb,
            blend_dst_rgb,
            blend_src_alpha,
            blend_dst_alpha,

            active_texture,
            texture_0,
        }
    }

    unsafe fn restore_gl_state(
        gl: &ffi::Gles2,
        state: &GlState,
    ) {
        gl.ActiveTexture(
            state.active_texture as u32,
        );

        gl.BindTexture(
            ffi::TEXTURE_2D,
            state.texture_0 as u32,
        );

        gl.BlendFuncSeparate(
            state.blend_src_rgb as u32,
            state.blend_dst_rgb as u32,
            state.blend_src_alpha as u32,
            state.blend_dst_alpha as u32,
        );

        if state.blend_enabled {
            gl.Enable(
                ffi::BLEND,
            );
        } else {
            gl.Disable(
                ffi::BLEND,
            );
        }

        gl.BindBuffer(
            ffi::ARRAY_BUFFER,
            state.array_buffer as u32,
        );

        gl.UseProgram(
            state.program as u32,
        );
    }
    
    pub(crate) fn program(&self) -> &ShaderProgram {
        &self.program
    }
}

impl Element for ShaderRenderElement {
    fn id(&self) -> &Id {
        &self.id
    }

    fn current_commit(&self) -> CommitCounter {
        self.commit
    }

    fn src(
        &self,
    ) -> Rectangle<f64, Buffer> {
        Rectangle::from_size(
            Size::from(
                (1.0, 1.0),
            ),
        )
    }

    fn geometry(
        &self,
        _scale: Scale<f64>,
    ) -> Rectangle<i32, Physical> {
        Rectangle::new(
            self.position,
            self.size,
        )
    }

    fn transform(&self) -> Transform {
        Transform::Normal
    }

    fn alpha(&self) -> f32 {
        1.0
    }

    fn opaque_regions(
        &self,
        _scale: Scale<f64>,
    ) -> OpaqueRegions<i32, Physical> {
        OpaqueRegions::default()
    }

    fn kind(&self) -> Kind {
        Kind::Unspecified
    }
}

impl RenderElement<GlesRenderer>
    for ShaderRenderElement
{
    fn draw(
        &self,
        frame: &mut GlesFrame<'_, '_>,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        _opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), GlesError> {
        self.draw_with_setup(
            frame,
            src,
            dst,
            damage,
            |_| {},
        )
    }

    fn underlying_storage(
        &self,
        _renderer: &mut GlesRenderer
    ) -> Option<UnderlyingStorage<'_>> {
        None
    }
}