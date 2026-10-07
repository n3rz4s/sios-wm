use sios_wm_api::{
    MonitorConfig,
    WindowConfig,
    CursorConfig,
};

use sios_wm_shader_api::ShaderAPI;


pub struct Config {
    pub monitor: MonitorConfig,
    pub window: WindowConfig,
    pub cursor: CursorConfig,
    pub shader: ShaderAPI,
}

impl Config {
    pub fn new() -> Self {
        Self {
            monitor: MonitorConfig::new(),
            window: WindowConfig::new(),
            cursor: CursorConfig::new(),
            shader: ShaderAPI::new(),
        }
    }
}