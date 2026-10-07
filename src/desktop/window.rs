use smithay::{
    reexports::wayland_protocols::xdg::shell::server::xdg_toplevel,
    utils::{
        Logical,
        Point,
        Size,
    },
    wayland::shell::xdg::ToplevelSurface,
};

use super::monitor::MonitorId;


#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct WindowId(pub u64);

pub struct Window {
    pub id: WindowId,
    pub surface: ToplevelSurface,

    pub identity: WindowIdentity,
    pub geometry: WindowGeometry,
    pub constraints: WindowConstraints,
    pub transform: WindowTransform,
    pub state: WindowState,
    pub behavior: WindowBehavior,
    pub appearance: WindowAppearance,
    pub animation: WindowAnimation,
}

pub struct WindowIdentity {
    pub app_id: String,
    pub title: String,

    pub class: Option<String>,
    pub instance: Option<String>,

    pub parent: Option<WindowId>,
}

pub struct WindowGeometry {
    pub position: Point<i32, Logical>,
    pub size: Size<i32, Logical>,

    pub requested_position: Option<Point<i32, Logical>>,
    pub requested_size: Option<Size<i32, Logical>>,
}

pub struct WindowConstraints {
    pub min_size: Option<Size<i32, Logical>>,
    pub max_size: Option<Size<i32, Logical>>,

    pub aspect_ratio: Option<f32>,
    pub keep_aspect_ratio: bool,
}

pub struct WindowTransform {
    pub scale: f64,
    pub rotation: f32,

    pub translation: Point<f32, Logical>,
}

pub struct WindowState {
    pub mapped: bool,
    pub focused: bool,

    pub maximized: bool,
    pub fullscreen: bool,

    pub urgent: bool,
    pub activated: bool,

    pub visible: WindowVisibility,
}

pub enum WindowVisibility {
    Visible,
    Hidden,
    Minimized,
}

pub struct WindowBehavior {
    pub floating: bool,
    pub sticky: bool,

    pub focusable: bool,
    pub movable: bool,
    pub resizable: bool,

    pub draggable: bool,
    pub clickable: bool,

    pub accepts_keyboard: bool,
    pub accepts_pointer: bool,

    pub modal: bool,

    pub always_on_top: bool,
    pub always_on_bottom: bool,

    pub prevents_idle: bool,

    pub monitor: Option<MonitorId>,
}

pub struct WindowAppearance {
    pub opacity: f32,

    pub decoration: DecorationMode,

    pub background: BackgroundConfig,
    pub border: BorderConfig,
    pub shadow: ShadowConfig,
    pub blur: BlurConfig,
    pub glow: GlowConfig,
    pub effects: WindowEffects,
    
    pub rounding: RoundingConfig,
    pub clipping: ClippingConfig,

    pub dim_when_unfocused: bool,
    pub focused_opacity: f32,
    pub unfocused_opacity: f32,
}

pub enum DecorationMode {
    None,
    Server,
    Client,
}

pub struct BackgroundConfig {
    pub enabled: bool,

    pub color: Color,
    pub opacity: f32,

    pub gradient: Option<GradientConfig>,
}

#[derive(Clone)]
pub struct GradientConfig {
    pub gradient_type: GradientType,

    pub start: Color,
    pub end: Color,

    pub angle: f32,
}

#[derive(Clone)]
pub enum GradientType {
    Linear,
    Radial,
}

pub struct BorderConfig {
    pub enabled: bool,

    pub width: u32,
    pub opacity: f32,

    pub radius: f32,
    pub vignette: f32,

    pub focused_width: u32,
    pub unfocused_width: u32,

    pub focused_color: Color,
    pub unfocused_color: Color,
}

pub struct ShadowConfig {
    pub enabled: bool,

    pub radius: f32,
    pub spread: f32,

    pub offset: Point<f32, Logical>,
    pub quality: u32,

    pub color: Color,
    pub opacity: f32,

    pub monitor: Option<MonitorId>,
    pub gradient: Option<GradientConfig>,
}

pub struct BlurConfig {
    pub enabled: bool,

    pub radius: f32,
    pub passes: u32,

    pub saturation: f32,
    pub brightness: f32,

    pub behind: bool,

    pub opacity: f32,
}

pub struct GlowConfig {
    pub enabled: bool,

    pub radius: f32,
    pub spread: f32,
    pub opacity: f32,

    pub color: Color,
}

pub struct RoundingConfig {
    pub enabled: bool,

    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

pub struct ClippingConfig {
    pub enabled: bool,

    pub clip_content: bool,
    pub clip_background: bool,
    pub clip_shadow: bool,
}

pub struct WindowEffects {
    pub saturation: f32,
    pub brightness: f32,
    pub contrast: f32,

    pub hue: f32,
    pub gamma: f32,

    pub grayscale: f32,

    pub tint: Color,

    pub invert: f32,
    pub sepia: f32,
}

pub struct WindowAnimation {
    pub enabled: bool,

    pub open: AnimationConfig,
    pub close: AnimationConfig,
    pub move_: AnimationConfig,
    pub resize: AnimationConfig,
    pub focus: AnimationConfig,
}

pub struct AnimationConfig {
    pub enabled: bool,

    pub duration_ms: u32,

    pub easing: Easing,

    pub opacity: bool,
    pub scale: bool,
    pub translation: bool,
    pub rotation: bool,
}

pub enum Easing {
    Linear,

    EaseIn,
    EaseOut,
    EaseInOut,

    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,

    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,

    Spring {
        stiffness: f32,
        damping: f32,
        mass: f32,
    },
}

#[derive(
    Debug,
    Clone,
    Copy,
)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0
    };

    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
}

impl Window {
    pub fn new(
        id: WindowId,
        surface: ToplevelSurface,
    ) -> Self {
        Self {
            id,
            surface,

            identity: WindowIdentity {
                app_id: String::new(),
                title: String::new(),

                class: None,
                instance: None,

                parent: None,
            },

            geometry: WindowGeometry {
                position: Point::from((0, 0)),
                size: Size::from((1, 1)),

                requested_position: None,
                requested_size: None,
            },

            transform: WindowTransform {
                scale: 1.0,
                rotation: 0.0,

                translation: Point::from((0.0, 0.0))
            },

            constraints: WindowConstraints {
                min_size: None,
                max_size: None,

                aspect_ratio: None,
                keep_aspect_ratio: false,
            },

            state: WindowState {
                mapped: false,
                focused: false,

                maximized: false,
                fullscreen: false,

                urgent: false,
                activated: false,

                visible: WindowVisibility::Visible,
            },

            behavior: WindowBehavior {
                floating: false,
                sticky: false,

                focusable: true,
                movable: true,
                resizable: true,

                draggable: true,
                clickable: true,

                accepts_keyboard: true,
                accepts_pointer: true,

                modal: false,

                always_on_top: false,
                always_on_bottom: false,

                prevents_idle: false,

                monitor: None,
            },

            appearance: WindowAppearance {
                opacity: 1.0,

                decoration: DecorationMode::Client,

                background: BackgroundConfig {
                    enabled: false,
                    color: Color::TRANSPARENT,
                    opacity: 1.0,
                    gradient: None,
                },

                border: BorderConfig {
                    enabled: false,

                    width: 0,
                    opacity: 1.0,

                    radius: 0.0,
                    vignette: 0.0,

                    focused_width: 0,
                    unfocused_width: 0,

                    focused_color: Color::WHITE,
                    unfocused_color: Color::WHITE,
                },

                shadow: ShadowConfig {
                    enabled: false,

                    radius: 0.0,
                    spread: 0.0,

                    offset: Point::from((0.0, 0.0)),
                    quality: 1,

                    color: Color::BLACK,
                    opacity: 1.0,

                    monitor: None,
                    gradient: None,
                },

                blur: BlurConfig {
                    enabled: false,

                    radius: 0.0,
                    passes: 1,

                    saturation: 1.0,
                    brightness: 1.0,

                    behind: false,

                    opacity: 1.0,
                },

                glow: GlowConfig {
                    enabled: false,

                    radius: 0.0,
                    spread: 0.0,
                    opacity: 1.0,

                    color: Color::WHITE,
                },

                effects: WindowEffects {
                    saturation: 1.0,
                    brightness: 1.0,
                    contrast: 1.0,

                    hue: 0.0,
                    gamma: 1.0,

                    grayscale: 0.0,

                    tint: Color::TRANSPARENT,

                    invert: 0.0,
                    sepia: 0.0,
                },

                rounding: RoundingConfig {
                    enabled: false,

                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_left: 0.0,
                    bottom_right: 0.0,
                },

                clipping: ClippingConfig {
                    enabled: false,

                    clip_content: false,
                    clip_background: false,
                    clip_shadow: false,
                },

                dim_when_unfocused: false,

                focused_opacity: 1.0,
                unfocused_opacity: 1.0,
            },

            animation: WindowAnimation {
                enabled: true,

                open: AnimationConfig::default(),
                close: AnimationConfig::default(),
                move_: AnimationConfig::default(),
                resize: AnimationConfig::default(),
                focus: AnimationConfig::default(),
            },
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn surface(&self) -> &ToplevelSurface {
        &self.surface
    }

    pub fn surface_mut(&mut self) -> &mut ToplevelSurface {
        &mut self.surface
    }

    pub fn is_mapped(&self) -> bool {
        self.state.mapped
    }

    pub fn map(&mut self) {
        self.state.mapped = true;
        self.state.visible = WindowVisibility::Visible;
    }

    pub fn unmap(&mut self) {
        self.state.mapped = false;
    }

    pub fn is_focused(&self) -> bool {
        self.state.focused
    }

    pub fn is_focusable(&self) -> bool {
        self.behavior.focusable 
            && self.is_mapped()
            && self.is_visible()
    }


    pub fn focus(&mut self) {
        self.state.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.state.focused = false;
    }

    pub fn set_position(
        &mut self,
        position: Point<i32, Logical>
    ) {
        self.geometry.position = position;
    }
 
    pub fn position(&self) -> Point<i32, Logical> {
        self.geometry.position
    }

    pub fn set_size(
        &mut self,
        size: Size<i32, Logical>
    ) {
        self.geometry.size = size;
    }

    pub fn size(&self) -> Size<i32, Logical> {
        self.geometry.size
    }

    pub fn set_monitor(
        &mut self,
        monitor: Option<MonitorId>,
    ) {
        self.behavior.monitor = monitor;
    }

    pub fn monitor(&self) -> Option<MonitorId> {
        self.behavior.monitor
    }

    pub fn is_visible(&self) -> bool {
        matches!(
            self.state.visible,
            WindowVisibility::Visible,
        )
    }

    pub fn hide(&mut self) {
        self.state.visible = WindowVisibility::Hidden;
    }

    pub fn minimize(&mut self) {
        self.state.visible = WindowVisibility::Minimized;
    }

    pub fn show(&mut self) {
        self.state.visible = WindowVisibility::Visible;
    }

    pub fn set_floating(
        &mut self,
        floating: bool,
    ) {
        self.behavior.floating = floating;
    }

    pub fn is_floating(&self) -> bool {
        self.behavior.floating
    }

    pub fn set_fullscreen(
        &mut self,
        fullscreen: bool
    ) {
        self.state.fullscreen = fullscreen;
    }

    pub fn is_fullscreen(&self) -> bool {
        self.state.fullscreen
    }

    pub fn set_maximized(
        &mut self,
        maximized: bool
    ) {
        self.state.maximized = maximized;
    }

    pub fn is_maximized(&self) -> bool {
        self.state.maximized
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,

            duration_ms: 150,

            easing: Easing::EaseOutCubic,

            opacity: true,
            scale: true,
            translation: true,
            rotation: true,
        }
    }
}