use libloading::Symbol;

use sios_wm_api::CursorConfig;

use super::ConfigLoader;

type CursorFn =
    unsafe extern "C" fn(&mut CursorConfig);

pub struct CursorConfigLoader;

impl CursorConfigLoader {
    pub fn load(
        loader: &ConfigLoader,
        config: &mut CursorConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let cursor: Symbol<CursorFn> =
                loader.library().get(b"cursor")?;

            cursor(config);
        }

        Ok(())
    }
}