use sios_wm_api::{
    BorderSettings,
    WindowConfig,
};

use crate::desktop::window_rules::{
    AppearanceActions,
    BorderActions,
    WindowActions,
    WindowMatcher,
    WindowRule,
    WindowRules,
};


pub fn build_window_rules(
    config: &WindowConfig,
) -> WindowRules {
    let mut rules = WindowRules::new();

    for rule in &config.windows {
        let mut actions = WindowActions::default();

        apply_border(
            &mut actions.appearance,
            rule.settings.appearance.border.as_ref(),
        );

        let matcher = WindowMatcher::default();
        
        rules.add(
            WindowRule::new(
                matcher,
                actions,
            ),
        );
    }

    rules
}

fn apply_border(
    appearance: &mut AppearanceActions,
    settings: Option<&BorderSettings>,
) {
    let Some(settings) = settings else {
        return;
    };

    let mut border = BorderActions::default();

    border.enabled = settings.enabled;
    border.width = settings.width;
    border.opacity = settings.opacity;

    border.radius = settings.radius;
    border.vignette = settings.vignette;
    

    border.focused_width = settings.focused_width;
    border.unfocused_width = settings.unfocused_width;

    border.focused_color = settings
        .focused_color
        .map(convert_color);

    border.unfocused_color = settings
        .unfocused_color
        .map(convert_color);

    appearance.border = Some(border);
}

fn convert_color(
    color: sios_wm_api::Color
) -> crate::desktop::window::Color {
    crate::desktop::window::Color {
        r: color.r.unwrap_or(0.0),
        g: color.g.unwrap_or(0.0),
        b: color.b.unwrap_or(0.0),
        a: color.a.unwrap_or(1.0),
    }
}