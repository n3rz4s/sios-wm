use smithay::{
    reexports::drm::control::crtc::Handle as CrtcHandle,
    output::{
        Mode as SmithayMode,
        Output,
        PhysicalProperties,
        Subpixel,
    },
    backend::{
        allocator::{
            gbm::{
                GbmAllocator,
                GbmBufferFlags,
            },
            Fourcc,
        },
        drm::{
            compositor::{
                DrmCompositor as SmithayDrmCompositor,
                FrameFlags,
            },
            exporter::gbm::GbmFramebufferExporter,
            DrmDeviceFd,
            DrmSurface,
        },
        renderer::{
            gles::GlesRenderer,
            Color32F,
            ImportDma,
        },
    },
    utils::Size,
};

use std::collections::HashSet;

use crate::{
    backend::{
        drm::{
            DrmBackend,
            DrmOutput
        },
        renderer::RendererBackend,
    },
    desktop::{
        render::{
            scene,
            CustomRenderElement,
            CursorRenderer,
            ShaderCompositor,
            RenderCache,
            Resources,
        },
        Desktop,
        Cursor,
    }
};

type SmithayCompositor = SmithayDrmCompositor<
    GbmAllocator<DrmDeviceFd>,
    GbmFramebufferExporter<DrmDeviceFd>,
    (),
    DrmDeviceFd,
>;

pub struct DrmOutputCompositor {
    pub output: DrmOutput,
    pub compositor: SmithayCompositor,
}


pub struct DrmCompositor {
    pub outputs: Vec<DrmOutputCompositor>,
    pub renderer: GlesRenderer,
    pub render_cache: RenderCache,
}

impl DrmCompositor {
    pub fn new(
        drm: &DrmBackend,
        surfaces: Vec<DrmSurface>,
        outputs: &[DrmOutput],
        mut renderer_backend: RendererBackend,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        eprintln!(
            "SIOS: Initializing DRM compositor for {} output(s)...",
            outputs.len(),
        );

        if surfaces.len() != outputs.len() {
            return Err(
                "SIOS: Number of DRM surfaces does not match outputs.".into(),
            );
        }

        let gbm = renderer_backend.gbm.clone();

        let renderer_formats = renderer_backend
            .renderer
            .dmabuf_formats()
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        let color_formats = [
            Fourcc::Argb8888,
            Fourcc::Xrgb8888,
            Fourcc::Abgr8888,
            Fourcc::Xbgr8888,
        ];

        let mut compositor_outputs =
            Vec::with_capacity(outputs.len());

        for (output_info, surface) in
            outputs.iter().zip(surfaces.into_iter())
        {
            let size = output_info.mode.size();

            eprintln!(
                "SIOS: Initializing compositor: connector={:?}, crtc={:?}, mode={}x{} @ {} Hz",
                output_info.connector,
                output_info.crtc,
                size.0,
                size.1,
                output_info.mode.vrefresh(),
            );

            let allocator = GbmAllocator::new(
                gbm.clone(),
                GbmBufferFlags::RENDERING | GbmBufferFlags::SCANOUT,
            );

            let exporter = GbmFramebufferExporter::new(
                gbm.clone(),
                Some(drm.node),
            );

            let output = Output::new(
                format!(
                    "SIOS-Output-{:?}",
                    output_info.connector
                ),
                PhysicalProperties {
                    size: Size::from((500, 300)),
                    make: "SIOS".into(),
                    model: "SIOS".into(),
                    subpixel: Subpixel::Unknown,
                },
            );

            let smithay_mode = SmithayMode {
                size: Size::from((
                    size.0 as i32,
                    size.1 as i32,
                )),
                refresh: output_info.mode.vrefresh() as i32 * 1000,
            };

            output.change_current_state(
                Some(smithay_mode),
                None,
                None,
                None,
            );

            let compositor = SmithayDrmCompositor::new(
                &output,
                surface,
                None,
                allocator,
                exporter,
                color_formats,
                renderer_formats.clone(),
                drm.drm.cursor_size(),
                Some(gbm.clone()),
            )?;

            compositor_outputs.push(
                DrmOutputCompositor {
                    output: output_info.clone(),
                    compositor,
                }
            );
        }

        eprintln!(
            "SIOS: DRM Compositor initialized for {} output(s)",
            compositor_outputs.len()
        );

        let resources =
            Resources::new(
                &mut renderer_backend.renderer,
            )?;

        Ok(Self {
            outputs: compositor_outputs,
            renderer: renderer_backend.renderer,
            render_cache: RenderCache::new(resources),
        })
    }

    pub fn renderer_mut(&mut self) -> &mut GlesRenderer {
        &mut self.renderer
    }

    pub fn render(
        &mut self,
        output_name: &str,
        elements: &[CustomRenderElement],
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let Some(output) = self.outputs.iter_mut().find(|output| {
            output.output.name == output_name
        }) else {
            eprintln!(
                "SIOS: No DRM output: '{}'",
                output_name,
            );

            return Ok(false);
        };

        let frame = output.compositor.render_frame(
            &mut self.renderer,
            elements,
            Color32F::new(
                0.08,
                0.10,
                0.14,
                1.0,
            ),
            FrameFlags::DEFAULT,
        )?;

        
        if frame.is_empty {
            return Ok(false);
        }

        output.compositor.queue_frame(())?;

        eprintln!(
            "SIOS: Frame queued: {} ({} elements)",
            output_name,
            elements.len(),
        );

        Ok(true)
    }

    pub fn render_desktop(
        &mut self,
        desktop: &Desktop,
        cursor: &Cursor,
        cursor_renderer: &mut CursorRenderer,
        compositor: &ShaderCompositor
    ) -> Result<bool, Box<dyn std::error::Error>> {

        cursor_renderer.sync(
            &mut self.renderer,
            cursor,
        )?;

        let mut queued = false;

        for output_index in 0..self.outputs.len() {
            let output_name =
                self.outputs[output_index]
                    .output
                    .name
                    .clone();

            let Some(monitor) =
                desktop.monitors.get_monitor_by_name(
                    &output_name,
                )
            else {
                eprintln!(
                    "SIOS: No monitor for DRM output: '{}'",
                    output_name,
                );

                continue;
            };

            if !monitor.enabled {
                continue;
            }

            let mut elements =
                scene::build(
                    &mut self.renderer,
                    compositor,
                    &mut self.render_cache,
                    desktop,
                    monitor,
                )?;

            if let Some(cursor_element) =
                cursor_renderer.element(
                    &mut self.renderer,
                    cursor,
                )?
            {
                elements.insert(
                    0,
                    CustomRenderElement::cursor(
                        cursor_element,
                    ),
                );
            }

            queued |= self.render(
                &output_name,
                &elements,
            )?;
        }

        Ok(queued)
    }

    pub fn frame_submitted(
        &mut self,
        crtc: CrtcHandle,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        for output in &mut self.outputs {
            if output.output.crtc == crtc {
                let submitted =
                    output
                        .compositor
                        .frame_submitted()?;

                return Ok(submitted.is_some());
            }
        }

        Ok(false)
    }
}