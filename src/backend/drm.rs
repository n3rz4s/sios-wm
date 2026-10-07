use smithay::{
    backend::{
        drm::{
            DrmDevice,
            DrmDeviceFd,
            DrmDeviceNotifier,
            DrmSurface,
            DrmNode,
        },
        session::{
            libseat::LibSeatSession,
            Session,
        },
    },
    reexports::{
        drm::control::{
            Device as DrmControlDevice,
            connector::Handle as ConnectorHandle,
            connector::State::Connected as ConnectorStateConnected,
            crtc::Handle as CrtcHandle,
            Mode,
            connector::Interface::{
                DisplayPort,
                HDMIA,
                HDMIB,
                EmbeddedDisplayPort,
            },
        },
        rustix::fs::OFlags,
    },
    utils::DeviceFd,
};

use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use sios_wm_api::MonitorConfig;

pub struct DrmBackend {
    pub drm: DrmDevice,
    pub fd: DrmDeviceFd,
    pub node: DrmNode,
}

#[derive(Debug, Clone)]
pub struct DrmOutput {
    pub name: String,
    pub connector: ConnectorHandle,
    pub crtc: CrtcHandle,
    pub mode: Mode,
    pub physical_size: (i32, i32),
}

impl DrmBackend {

    fn find_display_gpu() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let dri_path = Path::new("/dev/dri");
        let sysfs_path = Path::new("/sys/class/drm");

        for entry in fs::read_dir(dri_path)? {
            let entry = entry?;
            let path = entry.path();

            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };

            if !name.starts_with("card") {
                continue;
            }

            for connector in fs::read_dir(sysfs_path)? {
                let connector = connector?;
                let connector_name = connector.file_name();
                let connector_name = connector_name.to_string_lossy();

                if !connector_name.starts_with(&format!("{name}-")) {
                    continue;
                }

                let status_path = connector.path().join("status");

                if let Ok(status) = fs::read_to_string(status_path) {
                    if status.trim() == "connected" {
                        return Ok(path);
                    }
                }
            }
        }

        Err(
            "SIOS: No DRM Device with connected display found.".into(),
        )
    }

    pub fn new(
        session: &mut LibSeatSession,
    ) -> Result<
    (Self, DrmDeviceNotifier),
    Box<dyn std::error::Error>,
    > {

        let path = Self::find_display_gpu()?;

        eprintln!(
            "SIOS: Primary GPU = {}",
            path.display()
        );

        let node = DrmNode::from_path(&path)?;
        
        let fd = session.open(
            &path,
            OFlags::RDWR
                | OFlags::CLOEXEC
                | OFlags::NOCTTY
                | OFlags::NONBLOCK,
        )?;

        let drm_fd = DrmDeviceFd::new(
            DeviceFd::from(fd)
        );

        let (drm, notifier) = 
            DrmDevice::new(drm_fd.clone(), true)?;

        eprintln!("SIOS: DRM Device initialized.");

        eprintln!(
            "SIOS: DRM atomic = {}",
            drm.is_atomic()
        );

        eprintln!(
            "SIOS: DRM CRTC count = {}",
            drm.crtcs().len()
        );

        Ok((
            Self {
                drm,
                fd: drm_fd,
                node,
            },
            notifier,
        ))
    }

    pub fn find_outputs(
        &self,
        monitor: &MonitorConfig,
    ) -> Result<Vec<DrmOutput>, Box<dyn std::error::Error>> {
        let resources = self.drm.resource_handles()?;

        let mut outputs = Vec::new();
        let mut used_crtcs = Vec::new();

        for connector_handle in resources.connectors() {
            let connector = self.drm.get_connector(*connector_handle, true)?;

            if connector.state() != ConnectorStateConnected {
                continue;
            }

            let physical_size = connector
                .size()
                .map(|(width, height)|{
                    (width as i32, height as i32)
                })
                .unwrap_or((0, 0));

            let name = match connector.interface() {
                DisplayPort => {
                    format!("DP-{}", connector.interface_id())
                }

                HDMIA => {
                    format!("HDMI-A-{}", connector.interface_id())
                }

                HDMIB => {
                    format!("HDMI-B-{}", connector.interface_id())
                }

                EmbeddedDisplayPort => {
                    format!("eDP-{}", connector.interface_id())
                }

                interface => {
                    format!("{:?}-{}", interface, connector.interface_id())
                }
            };

            let modes = connector.modes();


            if modes.is_empty() {
                return Err(
                    "SIOS: Connected connector has no modes.".into(),
                );
            }



            let mode = if let Some(settings) = monitor.monitors
                .iter()
                .find(|settings| settings.name == name) 
            {
                if !settings.enabled {
                    eprintln!(
                        "SIOS: Output disabled by config: {}",
                        name,
                    );

                    continue;
                }

                let requested_width = settings.width as u16;
                let requested_height = settings.height as u16;
                let requested_refresh = settings.refresh as u32;

                let selected = modes
                    .iter()
                    .filter(|mode| {
                        let (width, height) = mode.size();

                        width == requested_width
                            && height == requested_height
                    })
                    .min_by_key(|mode| {
                        (
                            mode.vrefresh() as i64
                                - requested_refresh as i64
                        )
                        .abs()
                    })
                    .copied();

                match selected {
                    Some(mode) => {
                        eprintln!(
                            "SIOS: Configured mode for {}: {}x{} @ {} Hz",
                            name,
                            mode.size().0,
                            mode.size().1,
                            mode.vrefresh()
                        );

                        mode
                    }

                    None => {
                        eprintln!(
                            "SIOS: Requested mode: {}x{} @ {} Hz not found on {}",
                            settings.width,
                            settings.height,
                            settings.refresh,
                            name,
                        );

                        eprintln!(
                            "SIOS: Available modes: ",
                        );

                        for available in modes {
                            eprintln!(
                                "SIOS: {}x{} @ {} Hz",
                                available.size().0,
                                available.size().1,
                                available.vrefresh(),
                            );
                        }

                        return Err(
                            format!(
                                "SIOS: Requested monitor mode is not supported: {} {}x{} @ {} Hz",
                                name,
                                settings.width,
                                settings.height,
                                settings.refresh,
                            )
                            .into(),
                        );
                    }
                }
            } else {
                *modes
                    .first()
                    .ok_or(
                        "SIOS: Connected connector has no modes."
                    )?
                };

            let mut selected_crtc = None;

            for encoder_handle in connector.encoders() {
                let encoder = self.drm.get_encoder(*encoder_handle)?;

                for crtc in resources.filter_crtcs(encoder.possible_crtcs()) {
                    if used_crtcs.contains(&crtc) {
                        continue;
                    }

                    selected_crtc = Some(crtc);
                    break;
                }

                if selected_crtc.is_some() {
                    break;
                }
            }

            let Some(crtc) = selected_crtc else {
                eprintln!(
                    "SIOS: No available CRTC for connector {:?}",
                    connector_handle,
                );

                continue;
            };

            used_crtcs.push(crtc);

            let output = DrmOutput {
                name,
                connector: *connector_handle,
                crtc,
                mode,
                physical_size,
            };

            eprintln!(
                "SIOS: DRM Output: name={}, connector={:?}, crtc={:?}, mode={}x{} @ {} Hz",
                output.name,
                output.connector,
                output.crtc,
                output.mode.size().0,
                output.mode.size().1,
                output.mode.vrefresh(),
            );

            outputs.push(output);
        }

        if outputs.is_empty() {
            return Err("SIOS: No usable DRM outputs found".into());
        }

        eprintln!(
            "SIOS: Found {} usable DRM outputs.",
            outputs.len()
        );

        Ok(outputs)
    }

    pub fn create_surfaces(&mut self, outputs: &[DrmOutput]) -> Result<Vec<DrmSurface>, Box<dyn std::error::Error>> {
        let mut surfaces = Vec::with_capacity(outputs.len());

        for output in outputs {
            let surface = self.drm.create_surface(
                output.crtc,
                output.mode,
                &[output.connector],
            )?;

            eprintln!(
                "SIOS: DRM Surface initialized: connector={:?}, crtc={:?}",
                output.connector,
                output.crtc,
            );

            surfaces.push(surface);
        }

        Ok(surfaces)
    }
}