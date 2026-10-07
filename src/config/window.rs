use libloading::Symbol;

use sios_wm_api::WindowConfig;

use super::ConfigLoader;

type WindowFn =
    unsafe extern "C" fn(&mut WindowConfig);

pub struct WindowConfigLoader;

impl WindowConfigLoader {
    pub fn load(
        loader: &ConfigLoader,
        config: &mut WindowConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let window: Symbol<WindowFn> =
                loader.library().get(b"window")?;

            window(config);
        }

        Ok(())
    }
}