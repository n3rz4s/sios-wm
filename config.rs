use sios_wm_api::{
    Color,
    MonitorConfig,
    WindowConfig,
    CursorConfig,
};

use sios_wm_shader_api::{
    BasicShader,
    Shader,
    ShaderAPI,
};

#[unsafe(no_mangle)]
pub extern "C" fn monitor(api: &mut MonitorConfig) {
    api.monitor("eDP-1", |monitor| {
        monitor.mode(1920, 1080, 60);
        monitor.scale(1.0);
        monitor.gaps(8);
        monitor.enabled(true);
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn window(api: &mut WindowConfig) {
    api.all_windows(|window| {
        window.appearance.border(|border| {
            border.enabled(true);
            border.width(1);
            border.opacity(1.0);
            border.radius(18.0);
            border.vignette(0.50);
            border.focused_width(2);
            border.unfocused_width(2);
            border.focused_color(
                Color::rgba(
                    83.0 / 255.0,
                    168.0 / 255.0,
                    237.0 / 255.0,
                    1.0,
                ),
            );
            border.unfocused_color(
                Color::rgba(
                    239.0 / 255.0,
                    29.0 / 255.0,
                    29.0 / 255.0,
                    1.0,
                ),
            );
        });
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn shader(api: &mut ShaderAPI) {
    api.rounded_border(
        Shader::basic(
            BasicShader::RoundedBorder,
        ),
    );
    api.rounded_surface(
        Shader::basic(
            BasicShader::RoundedSurface,
        ),
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn cursor(api: &mut CursorConfig) {
    api.enabled(true);
    api.theme("Adwaita");
    api.size(24);
    api.icon("left_ptr");
}