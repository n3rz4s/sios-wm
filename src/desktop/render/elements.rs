use smithay::{
    backend::renderer::{
        element::{
            memory::MemoryRenderBufferRenderElement,
            solid::SolidColorRenderElement,
            surface::WaylandSurfaceRenderElement,
            texture::TextureRenderElement,
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
        Color32F,
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

use std::rc::Rc;

use crate::desktop::render::{
    CursorRenderer,
    CursorElement,
    shaders::{
        RoundedBorder,
        RoundedSurface,
    },
};

pub enum CustomRenderElement {
    Solid(
        SolidColorRenderElement,
    ),

    Surface(
        WaylandSurfaceRenderElement<GlesRenderer>,
    ),

    Cursor(
        MemoryRenderBufferRenderElement<GlesRenderer>,
    ),

    RoundedBorder(
        RoundedBorder,
    ),

    RoundedSurface(
        RoundedSurface,
    )
}

impl CustomRenderElement {
    pub fn rect(
        position: Point<i32, Physical>,
        size: Size<i32, Physical>,
        color: Color32F,
    ) -> Self {
        let geometry = Rectangle::new(
            position,
            size,
        );

        let element =
            SolidColorRenderElement::new(
                Id::new(),
                geometry,
                CommitCounter::default(),
                color,
                Kind::Unspecified,
            );

        Self::Solid(element)
    }

    pub fn rgb(
        position: Point<i32, Physical>,
        size: Size<i32, Physical>,
        r: f32,
        g: f32,
        b: f32,
    ) -> Self {
        Self::rect(
            position,
            size,
            Color32F::new(
                r,
                g,
                b,
                1.0,
            ),
        )
    }

    pub fn rgba(
        position: Point<i32, Physical>,
        size: Size<i32, Physical>,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> Self {
        Self::rect(
            position,
            size,
            Color32F::new(
                r,
                g,
                b,
                a,
            ),
        )
    }

    pub fn cursor(
        cursor: MemoryRenderBufferRenderElement<GlesRenderer>,
    ) -> Self {
        Self::Cursor(cursor)
    }

    pub fn rounded_border(
        border: RoundedBorder,
    ) -> Self {
        Self::RoundedBorder(border)
    }

    pub fn rounded_surface(
        surface: RoundedSurface,
    ) -> Self {
        Self::RoundedSurface(surface)
    }
}

impl Element for CustomRenderElement {
    fn id(&self) -> &Id {
        match self {
            Self::Solid(element) => {
                element.id()
            }

            Self::Surface(element) => {
                element.id()
            }

            Self::Cursor(element) => {
                element.id()
            }

            Self::RoundedBorder(element) => {
                element.id()
            }

            Self::RoundedSurface(element) => {
                element.id()
            }
        }
    }

    fn current_commit(&self) -> CommitCounter {
        match self {
            Self::Solid(element) => {
                element.current_commit()
            }

            Self::Surface(element) => {
                element.current_commit()
            }

            Self::Cursor(element) => {
                element.current_commit()
            }

            Self::RoundedBorder(element) => {
                element.current_commit()
            }

            Self::RoundedSurface(element) => {
                element.current_commit()
            }
        }
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        match self {
            Self::Solid(element) => {
                element.src()
            }

            Self::Surface(element) => {
                element.src()
            }

            Self::Cursor(element) => {
                element.src()
            }

            Self::RoundedBorder(element) => {
                element.src()
            }

            Self::RoundedSurface(element) => {
                element.src()
            }
        }
    }

    fn transform(&self) -> Transform {
        match self {
            Self::Solid(element) => {
                element.transform()
            }

            Self::Surface(element) => {
                element.transform()
            }

            Self::Cursor(element) => {
                element.transform()
            }

            Self::RoundedBorder(element) => {
                element.transform()
            }

            Self::RoundedSurface(element) => {
                element.transform()
            }
        }
    }

    fn geometry(
        &self,
        scale: Scale<f64>,
    ) -> Rectangle<i32, Physical> {
        match self {
            Self::Solid(element) => {
                element.geometry(scale)
            }

            Self::Surface(element) => {
                element.geometry(scale)
            }

            Self::Cursor(element) => {
                element.geometry(scale)
            }

            Self::RoundedBorder(element) => {
                element.geometry(scale)
            }

            Self::RoundedSurface(element) => {
                element.geometry(scale)
            }
        }
    }

    fn damage_since(
        &self,
        scale: Scale<f64>,
        commit: Option<CommitCounter>,
    ) -> DamageSet<i32, Physical> {
        match self {
            Self::Solid(element) => {
                element.damage_since(
                    scale,
                    commit,
                )
            }

            Self::Surface(element) => {
                element.damage_since(
                    scale,
                    commit,
                )
            }

            Self::Cursor(element) => {
                element.damage_since(
                    scale,
                    commit,
                )
            }

            Self::RoundedBorder(element) => {
                element.inner.damage_since(
                    scale,
                    commit,
                )
            }

            Self::RoundedSurface(element) => {
                element.inner.damage_since(
                    scale,
                    commit,
                )
            }
        }
    }

    fn opaque_regions(
        &self,
        scale: Scale<f64>,
    ) -> OpaqueRegions<i32, Physical> {
        match self {
            Self::Solid(element) => {
                element.opaque_regions(scale)
            }

            Self::Surface(element) => {
                element.opaque_regions(scale)
            }

            Self::Cursor(element) => {
                element.opaque_regions(scale)
            }

            Self::RoundedBorder(_) => {
                OpaqueRegions::default()
            }

            Self::RoundedSurface(element) => {
                element.opaque_regions(scale)
            }
        }
    }

    fn alpha(&self) -> f32 {
        match self {
            Self::Solid(element) => {
                element.alpha()
            }

            Self::Surface(element) => {
                element.alpha()
            }

            Self::Cursor(element) => {
                element.alpha()
            }

            Self::RoundedBorder(element) => {
                element.alpha()
            }

            Self::RoundedSurface(element) => {
                element.alpha()
            }
        }
    }

    fn kind(&self) -> Kind {
        match self {
            Self::Solid(element) => {
                element.kind()
            }

            Self::Surface(element) => {
                element.kind()
            }

            Self::Cursor(element) => {
                element.kind()
            }

            Self::RoundedBorder(element) => {
                element.kind()
            }

            Self::RoundedSurface(element) => {
                element.kind()
            }
        }
    }
}

impl RenderElement<GlesRenderer>
    for CustomRenderElement
{
    fn draw(
        &self,
        frame: &mut GlesFrame<'_, '_>,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<
        (),
        <GlesRenderer as RendererSuper>::Error,
    > {
        match self {
            Self::Solid(element) => {
                <SolidColorRenderElement as RenderElement<GlesRenderer>>::draw(
                    element,
                    frame,
                    src,
                    dst,
                    damage,
                    opaque_regions,
                )
            }

            Self::Surface(element) => {
                <WaylandSurfaceRenderElement<GlesRenderer> as RenderElement<GlesRenderer>>::draw(
                    element,
                    frame,
                    src,
                    dst,
                    damage,
                    opaque_regions,
                )
            }

            Self::Cursor(element) => {
                element.draw(
                    frame,
                    src,
                    dst,
                    damage,
                    opaque_regions,
                )
            }

            Self::RoundedBorder(element) => {
                element.draw(
                    frame,
                    src,
                    dst,
                    damage,
                )
            }

            Self::RoundedSurface(element) => {
                element.draw(
                    frame,
                    src,
                    dst,
                    damage,
                    opaque_regions,
                )
            }
        }
    }

    fn underlying_storage(
        &self,
        renderer: &mut GlesRenderer,
    ) -> Option<UnderlyingStorage<'_>> {
        match self {
            Self::Solid(element) => {
                element.underlying_storage(
                    renderer,
                )
            }

            Self::Surface(element) => {
                element.underlying_storage(
                    renderer,
                )
            }

            Self::Cursor(element) => {
                element.underlying_storage(
                    renderer,
                )
            }

            Self::RoundedBorder(_) => {
                None
            }

            Self::RoundedSurface(_) => {
                None
            }
        }
    }
}