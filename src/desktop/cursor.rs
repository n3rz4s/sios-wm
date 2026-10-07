use smithay::utils::{
    Logical,
    Point,
};

use super::CursorTheme;

use sios_wm_api::CursorConfig;

pub enum CursorState {
    Visible,
    Hidden,
}

pub struct Cursor {
    pub position: Point<f64, Logical>,

    pub state: CursorState,

    pub theme: String,
    pub size: u32,
    pub icon: String,

    pub image: Option<CursorImage>,
}

pub struct CursorImage {
    pub width: u32,
    pub height: u32,

    pub hotspot_x: u32,
    pub hotspot_y: u32,

    pub pixels: Vec<u8>,
}

impl Cursor {
    pub fn new(config: &CursorConfig) -> Self {

        let enabled = 
            config.enabled.unwrap_or(true);

        let theme = config
            .theme
            .clone()
            .unwrap_or_else(|| "Adwaita".into());

        let size = config
            .size
            .unwrap_or(24);

        let icon = config
            .icon
            .clone()
            .unwrap_or_else(|| "left_ptr".into());

        let image = if enabled {
            match CursorTheme::new(&theme).load(&icon, size) {
                Ok(image) => {
                    eprintln!(
                        "SIOS: Cursor image loaded: {}x{} hotspot=({}, {})",
                        image.width,
                        image.height,
                        image.hotspot_x,
                        image.hotspot_y,
                    );

                    Some(image)
                }

                Err(error) => {
                    eprintln!("SIOS: Failed to load cursor: {}", error);
                    None
                }
            }
        } else {
            None
        };

        Self {
            position: Point::from((0.0, 0.0)),

            state: if enabled {
                CursorState::Visible
            } else {
                CursorState::Hidden
            },

            theme,
            size,
            icon,

            image,
        }
    }

    pub fn position(&self) -> Point<f64, Logical> {
        self.position
    }

    pub fn motion(
        &mut self,
        delta: (f64, f64),
    ) {
        self.position.x += delta.0;
        self.position.y += delta.1;
    }

    pub fn set_position(
        &mut self,
        position: Point<f64, Logical>,
    ) {
        self.position = position;
    }
}