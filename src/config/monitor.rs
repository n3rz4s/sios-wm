use libloading::Symbol;

use sios_wm_api::MonitorConfig;

use super::ConfigLoader;

type MonitorFn =
    unsafe extern "C" fn(&mut MonitorConfig);

pub struct MonitorConfigLoader;

impl MonitorConfigLoader {
    pub fn load(
        loader: &ConfigLoader,
        config: &mut MonitorConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let monitor: Symbol<MonitorFn> =
                loader.library().get(b"monitor")?;

            monitor(config);
        }

        Ok(())
    }
}