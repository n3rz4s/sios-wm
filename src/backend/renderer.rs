use smithay::backend::{
    allocator::gbm::GbmDevice,
    drm::DrmDeviceFd,
    egl::{
        EGLContext,
        EGLDisplay,
    },
    renderer::gles::GlesRenderer,
};

pub struct RendererBackend {
    pub gbm: GbmDevice<DrmDeviceFd>,
    pub egl: EGLDisplay,
    pub renderer: GlesRenderer,
}

impl RendererBackend {
    pub fn new(
        drm_fd: DrmDeviceFd,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        eprintln!("SIOS: Initializing GBM...");

        let gbm = GbmDevice::new(drm_fd)?;

        eprintln!("SIOS: GBM Initialized.");

        eprintln!("SIOS: Initializing EGL...");

        let egl = unsafe {
            EGLDisplay::new(gbm.clone())?
        };

        eprintln!("SIOS: EGL Initialized.");

        eprintln!("SIOS: Creating EGL Context...");

        let context = EGLContext::new(&egl)?;

        eprintln!("SIOS: EGL Context created.");

        eprintln!("SIOS: Creating Gles renderer...");

        let renderer = unsafe {
            GlesRenderer::new(context)?
        };

        eprintln!("SIOS: Gles renderer initialized.");

        Ok(
            Self {
                gbm,
                egl,
                renderer,
            },
        )
    }
}