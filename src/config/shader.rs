use libloading::Symbol;

use sios_wm_shader_api::ShaderAPI;

use super::ConfigLoader;


type ShaderFn = 
    unsafe extern "C" fn(&mut ShaderAPI);


pub struct ShaderConfigLoader;

impl ShaderConfigLoader {
    pub fn load(
        loader: &ConfigLoader,
        config: &mut ShaderAPI,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let shader: Symbol<ShaderFn> =
                loader.library().get(b"shader")?;

            shader(config);
        }

        Ok(())
    }
}