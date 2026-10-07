use smithay::utils::{
    Logical,
    Point,
    Size,
};

use regex::Regex;

use super::{
    monitor::MonitorId,
    window::{
        AnimationConfig,
        BackgroundConfig,
        BlurConfig,
        BorderConfig,
        ClippingConfig,
        Color,
        DecorationMode,
        Easing,
        GlowConfig,
        RoundingConfig,
        ShadowConfig,
        GradientConfig,
        Window,
        WindowEffects,
        WindowId,
        WindowVisibility,
    },
};

#[derive(Default)]
pub struct WindowMatcher {
    pub app_id: Option<Match>,
    pub title: Option<Match>,

    pub class: Option<Match>,
    pub instance: Option<Match>,

    pub parent: Option<ParentMatch>,
}

pub enum Match {
    Exact(String),
    Contains(String),
    Prefix(String),
    Suffix(String),
    Regex(String),
}

pub enum ParentMatch {
    Any,
    None,
    Window(WindowId),
    Match(Match),
}

pub struct WindowRule {
    pub matcher: WindowMatcher,
    pub actions: WindowActions,
}

pub struct WindowRules {
    pub rules: Vec<WindowRule>,
}

#[derive(Default)]
pub struct WindowActions {
    pub geometry: GeometryActions,
    pub constraints: ConstraintActions,
    pub transform: TransformActions,
    pub state: StateActions,
    pub behavior: BehaviorActions,
    pub appearance: AppearanceActions,
    pub animation: AnimationActions,
}

#[derive(Default)]
pub struct GeometryActions {
    pub position: Option<Point<i32, Logical>>,
    pub size: Option<Size<i32, Logical>>,

    pub requested_position: Option<Option<Point<i32, Logical>>>,
    pub requested_size: Option<Option<Size<i32, Logical>>>,
}

#[derive(Default)]
pub struct ConstraintActions {
    pub min_size: Option<Option<Size<i32, Logical>>>,
    pub max_size: Option<Option<Size<i32, Logical>>>,

    pub aspect_ratio: Option<Option<f32>>,
    pub keep_aspect_ratio: Option<bool>,
}

#[derive(Default)]
pub struct TransformActions {
    pub scale: Option<f64>,
    pub rotation: Option<f32>,

    pub translation: Option<Point<f32, Logical>>,
}

#[derive(Default)]
pub struct StateActions {
    pub mapped: Option<bool>,
    pub focused: Option<bool>,

    pub maximized: Option<bool>,
    pub fullscreen: Option<bool>,

    pub urgent: Option<bool>,
    pub activated: Option<bool>,

    pub visible: Option<WindowVisibility>,
}

#[derive(Default)]
pub struct BehaviorActions {
    pub floating: Option<bool>,
    pub sticky: Option<bool>,

    pub focusable: Option<bool>,
    pub movable: Option<bool>,
    pub resizable: Option<bool>,

    pub draggable: Option<bool>,
    pub clickable: Option<bool>,

    pub accepts_keyboard: Option<bool>,
    pub accepts_pointer: Option<bool>,

    pub modal: Option<bool>,

    pub always_on_top: Option<bool>,
    pub always_on_bottom: Option<bool>,

    pub prevents_idle: Option<bool>,

    pub monitor: Option<Option<MonitorId>>,
}

#[derive(Default)]
pub struct BackgroundActions {
    pub enabled: Option<bool>,
    pub color: Option<Color>,
    pub opacity: Option<f32>,
    pub gradient: Option<Option<GradientConfig>>,
}

#[derive(Default)]
pub struct BorderActions {
    pub enabled: Option<bool>,

    pub width: Option<u32>,
    pub opacity: Option<f32>,

    pub radius: Option<f32>,
    pub vignette: Option<f32>,

    pub focused_width: Option<u32>,
    pub unfocused_width: Option<u32>,
    
    pub focused_color: Option<Color>,
    pub unfocused_color: Option<Color>,
}

#[derive(Default)]
pub struct ShadowActions {
    pub enabled: Option<bool>,

    pub radius: Option<f32>,
    pub spread: Option<f32>,

    pub offset: Option<Point<f32, Logical>>,
    pub quality: Option<u32>,

    pub color: Option<Color>,
    pub opacity: Option<f32>,

    pub monitor: Option<Option<MonitorId>>,
    pub gradient: Option<Option<GradientConfig>>,
}

#[derive(Default)]
pub struct BlurActions {
    pub enabled: Option<bool>,

    pub radius: Option<f32>,
    pub passes: Option<u32>,

    pub saturation: Option<f32>,
    pub brightness: Option<f32>,

    pub behind: Option<bool>,

    pub opacity: Option<f32>,
}

#[derive(Default)]
pub struct GlowActions {
    pub enabled: Option<bool>,

    pub radius: Option<f32>,
    pub spread: Option<f32>,
    pub opacity: Option<f32>,

    pub color: Option<Color>,
}

#[derive(Default)]
pub struct EffectsActions {
    pub saturation: Option<f32>,
    pub brightness: Option<f32>,
    pub contrast: Option<f32>,

    pub hue: Option<f32>,
    pub gamma: Option<f32>,

    pub grayscale: Option<f32>,

    pub tint: Option<Color>,

    pub invert: Option<f32>,
    pub sepia: Option<f32>,
}

#[derive(Default)]
pub struct RoundingActions {
    pub enabled: Option<bool>,

    pub top_left: Option<f32>,
    pub top_right: Option<f32>,

    pub bottom_left: Option<f32>,
    pub bottom_right: Option<f32>,
}

#[derive(Default)]
pub struct ClippingActions {
    pub enabled: Option<bool>,

    pub clip_content: Option<bool>,
    pub clip_background: Option<bool>,
    pub clip_shadow: Option<bool>,
}

#[derive(Default)]
pub struct AppearanceActions {
    pub opacity: Option<f32>,

    pub decoration: Option<DecorationMode>,
    
    pub background: Option<BackgroundActions>,
    pub border: Option<BorderActions>,
    pub shadow: Option<ShadowActions>,
    pub blur: Option<BlurActions>,
    pub glow: Option<GlowActions>,
    pub effects: Option<EffectsActions>,

    pub rounding: Option<RoundingActions>,
    pub clipping: Option<ClippingActions>,
    
    pub dim_when_unfocused: Option<bool>,

    pub focused_opacity: Option<f32>,
    pub unfocused_opacity: Option<f32>,
}

#[derive(Default)]
pub struct AnimationConfigActions {
    pub enabled: Option<bool>,

    pub duration_ms: Option<u32>,
    pub easing: Option<Easing>,

    pub opacity: Option<bool>,
    pub scale: Option<bool>,
    pub translation: Option<bool>,
    pub rotation: Option<bool>,
}

#[derive(Default)]
pub struct AnimationActions {
    pub enabled: Option<bool>,

    pub open: Option<AnimationConfigActions>,
    pub close: Option<AnimationConfigActions>,
    pub move_: Option<AnimationConfigActions>,
    pub resize: Option<AnimationConfigActions>,
    pub focus: Option<AnimationConfigActions>,
}

impl WindowRules {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    pub fn add(&mut self, rule: WindowRule) {
        self.rules.push(rule);
    }

    pub fn apply(&mut self, window: &mut Window) {
        for rule in &self.rules {
            if rule.matches(window) {
                rule.apply(window);
            }
        }
    }

    pub fn clear(&mut self) {
        self.rules.clear();
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl WindowRule {
    pub fn new(
        matcher: WindowMatcher,
        actions: WindowActions,
    ) -> Self {
        Self {
            matcher,
            actions,
        }
    }

    pub fn matches(&self, window: &Window) -> bool {
        self.matcher.matches(window)
    }

    pub fn apply(&self, window: &mut Window) {
        self.actions.apply(window);
    }
}


impl WindowMatcher {
    pub fn matches(&self, window: &Window) -> bool {
        if let Some(matcher) = &self.app_id {
            if !matcher.matches(&window.identity.app_id) {
                return false;
            }
        }

        if let Some(matcher) = &self.title {
            if !matcher.matches(&window.identity.title) {
                return false;
            }
        }

        if let Some(matcher) = &self.class {
            if !matcher.matches_optional(
                window.identity.class.as_deref(),
            ) {
                return false;
            }
        }

        if let Some(matcher) = &self.instance {
            if !matcher.matches_optional(
                window.identity.instance.as_deref(),
            ) {
                return false;
            }
        }

        if let Some(matcher) = &self.parent {
            if !matcher.matches(window) {
                return false;
            }
        }

        true
    }
}

impl Match {
    pub fn exact(value: impl Into<String>) -> Self {
        Self::Exact(value.into())
    }

    pub fn contains(value: impl Into<String>) -> Self {
        Self::Contains(value.into())
    }

    pub fn prefix(value: impl Into<String>) -> Self {
        Self::Prefix(value.into())
    }

    pub fn suffix(value: impl Into<String>) -> Self {
        Self::Suffix(value.into())
    }

    pub fn regex(value: impl Into<String>) -> Self {
        Self::Regex(value.into())
    }

    pub fn matches(&self, value: &str) -> bool {
        match self {
            Self::Exact(expected) => {
                value == expected
            }

            Self::Contains(expected) => {
                value.contains(expected)
            }

            Self::Prefix(expected) => {
                value.starts_with(expected)
            }

            Self::Suffix(expected) => {
                value.ends_with(expected)
            }

            Self::Regex(pattern) => {
                Regex::new(pattern)
                    .map(|regex| regex.is_match(value))
                    .unwrap_or(false)
            }
        }
    }

    pub fn matches_optional(
        &self,
        value: Option<&str> 
    ) -> bool {
        match value {
            Some(value) => self.matches(value),
            None => false,
        }
    }
}

impl ParentMatch {
    pub fn matches(&self, window: &Window) -> bool {
        match self {
            Self::Any => {
                window.identity.parent.is_some()
            }

            Self::None => {
                window.identity.parent.is_none()
            }

            Self::Window(id) => {
                window.identity.parent == Some(*id)
            }

            Self::Match(_) => {
                false
            }
        }
    }
}

impl WindowActions {
    pub fn apply(&self, window: &mut Window) {
        self.geometry.apply(window);
        self.constraints.apply(window);
        self.transform.apply(window);
        self.state.apply(window);
        self.behavior.apply(window);
        self.appearance.apply(window);
        self.animation.apply(window);
    }
}

impl GeometryActions {
    fn apply(&self, window: &mut Window) {
        if let Some(value) = self.position {
            window.geometry.position = value;
        }

        if let Some(value) = self.size {
            window.geometry.size = value;
        }

        if let Some(value) = self.requested_position {
            window.geometry.requested_position = value;
        }

        if let Some(value) = self.requested_size {
            window.geometry.requested_size = value;
        }
    }
}

impl ConstraintActions {
    fn apply(&self, window: &mut Window) {
        if let Some(value) = &self.min_size {
            window.constraints.min_size = *value;
        }

        if let Some(value) = &self.max_size {
            window.constraints.max_size = *value;
        }

        if let Some(value) = &self.aspect_ratio {
            window.constraints.aspect_ratio = *value;
        }

        if let Some(value) = self.keep_aspect_ratio {
            window.constraints.keep_aspect_ratio = value;
        }
    }
}

impl TransformActions {
    fn apply(&self, window: &mut Window) {
        if let Some(value) = self.scale {
            window.transform.scale = value;
        }

        if let Some(value) = self.rotation {
            window.transform.rotation = value;
        }

        if let Some(value) = self.translation {
            window.transform.translation = value;
        }
    }
}

impl StateActions {
    fn apply(&self, window: &mut Window) {
        if let Some(value) = self.mapped {
            window.state.mapped = value;
        }

        if let Some(value) = self.focused {
            window.state.focused = value;
        }

        if let Some(value) = self.maximized {
            window.state.maximized = value;
        }

        if let Some(value) = self.fullscreen {
            window.state.fullscreen = value;
        }

        if let Some(value) = self.urgent {
            window.state.urgent = value;
        }

        if let Some(value) = self.activated {
            window.state.activated = value;
        }

        if let Some(value) = &self.visible {
            window.state.visible = match value {
                WindowVisibility::Visible => 
                    WindowVisibility::Visible,

                WindowVisibility::Hidden =>
                    WindowVisibility::Hidden,

                WindowVisibility::Minimized =>
                    WindowVisibility::Minimized,
            };
        }
    }
}

impl BehaviorActions {
    fn apply(&self, window: &mut Window) {
        if let Some(value) = self.floating {
            window.behavior.floating = value;
        }

        if let Some(value) = self.sticky {
            window.behavior.sticky = value;
        }

        if let Some(value) = self.focusable {
            window.behavior.focusable = value;
        }

        if let Some(value) = self.movable {
            window.behavior.movable = value;
        }

        if let Some(value) = self.resizable {
            window.behavior.resizable = value;
        }
        
        if let Some(value) = self.draggable {
            window.behavior.draggable = value;
        }

        if let Some(value) = self.clickable {
            window.behavior.clickable = value;
        }

        if let Some(value) = self.accepts_keyboard {
            window.behavior.accepts_keyboard = value;
        }

        if let Some(value) = self.accepts_pointer {
            window.behavior.accepts_pointer = value;
        }

        if let Some(value) = self.modal {
            window.behavior.modal = value;
        }

        if let Some(value) = self.always_on_top {
            window.behavior.always_on_top = value;
        }

        if let Some(value) = self.always_on_bottom {
            window.behavior.always_on_bottom = value;
        }

        if let Some(value) = self.prevents_idle {
            window.behavior.prevents_idle = value;
        }

        if let Some(value) = self.monitor {
            window.behavior.monitor = value;
        }
    }
}

impl AppearanceActions {
    fn apply(&self, window: &mut Window) {
        if let Some(value) = self.opacity {
            window.appearance.opacity = value;
        }

        if let Some(value) = &self.decoration {
            window.appearance.decoration = match value {
                DecorationMode::None =>
                    DecorationMode::None,

                DecorationMode::Server =>
                    DecorationMode::Server,

                DecorationMode::Client =>
                    DecorationMode::Client,
            };
        }

        if let Some(action) = &self.background {
            action.apply(&mut window.appearance.background);
        }

        if let Some(action) = &self.border {
            action.apply(&mut window.appearance.border);
        }

        if let Some(action) = &self.shadow {
            action.apply(&mut window.appearance.shadow);
        }

        if let Some(action) = &self.blur {
            action.apply(&mut window.appearance.blur);
        }

        if let Some(action) = &self.glow {
            action.apply(&mut window.appearance.glow);
        }

        if let Some(action) = &self.effects {
            action.apply(&mut window.appearance.effects);
        }

        if let Some(action) = &self.rounding {
            action.apply(&mut window.appearance.rounding);
        }

        if let Some(action) = &self.clipping {
            action.apply(&mut window.appearance.clipping);
        }

        if let Some(value) = self.dim_when_unfocused {
            window.appearance.dim_when_unfocused = value;
        }

        if let Some(value) = self.focused_opacity {
            window.appearance.focused_opacity = value;
        }

        if let Some(value) = self.unfocused_opacity {
            window.appearance.unfocused_opacity = value;
        }
    }
}

impl BackgroundActions {
    fn apply(&self, background: &mut BackgroundConfig) {
        if let Some(value) = self.enabled {
            background.enabled = value;
        }

        if let Some(value) = self.color {
            background.color = value;
        }

        if let Some(value) = self.opacity {
            background.opacity = value;
        }

        if let Some(value) = &self.gradient {
            background.gradient = value.clone();
        }
    }
}


impl BorderActions {
    fn apply(&self, border: &mut BorderConfig) {
        if let Some(value) = self.enabled {
            border.enabled = value;
        }

        if let Some(value) = self.width {
            border.width = value;
        }

        if let Some(value) = self.opacity {
            border.opacity = value;
        }

        if let Some(value) = self.radius {
            border.radius = value;
        }

        if let Some(value) = self.vignette {
            border.vignette = value;
        }

        if let Some(value) = self.focused_width {
            border.focused_width = value;
        }

        if let Some(value) = self.unfocused_width {
            border.unfocused_width = value;
        }

        if let Some(value) = self.focused_color {
            border.focused_color = value;
        }

        if let Some(value) = self.unfocused_color {
            border.unfocused_color = value;
        }
    }
}

impl ShadowActions {
    fn apply(&self, shadow: &mut ShadowConfig) {
        if let Some(value) = self.enabled {
            shadow.enabled = value;
        }

        if let Some(value) = self.radius {
            shadow.radius = value;
        }

        if let Some(value) = self.spread {
            shadow.spread = value;
        }

        if let Some(value) = self.offset {
            shadow.offset = value;
        }

        if let Some(value) = self.quality {
            shadow.quality = value;
        }

        if let Some(value) = self.color {
            shadow.color = value;
        }

        if let Some(value) = self.opacity {
            shadow.opacity = value;
        }

        if let Some(value) = self.monitor {
            shadow.monitor = value;
        }

        if let Some(value) = &self.gradient {
            shadow.gradient = value.clone();
        }
    }
}


impl BlurActions {
    fn apply(&self, blur: &mut BlurConfig) {
        if let Some(value) = self.enabled {
            blur.enabled = value;
        }

        if let Some(value) = self.radius {
            blur.radius = value;
        }

        if let Some(value) = self.passes {
            blur.passes = value;
        }

        if let Some(value) = self.saturation {
            blur.saturation = value;
        }

        if let Some(value) = self.brightness {
            blur.brightness = value;
        }

        if let Some(value) = self.behind {
            blur.behind = value;
        }

        if let Some(value) = self.opacity {
            blur.opacity = value;
        }
    }
}

impl GlowActions {
    fn apply(&self, glow: &mut GlowConfig) {
        if let Some(value) = self.enabled {
            glow.enabled = value;
        }

        if let Some(value) = self.radius {
            glow.radius = value;
        }

        if let Some(value) = self.spread {
            glow.spread = value;
        }

        if let Some(value) = self.opacity {
            glow.opacity = value;
        }
        
        if let Some(value) = self.color {
            glow.color = value;
        }
    }
}

impl EffectsActions {
    fn apply(&self, effects: &mut WindowEffects) {
        if let Some(value) = self.saturation {
            effects.saturation = value;
        }

        if let Some(value) = self.brightness {
            effects.brightness = value;
        }

        if let Some(value) = self.contrast {
            effects.contrast = value;
        }

        if let Some(value) = self.hue {
            effects.hue = value;
        }

        if let Some(value) = self.gamma {
            effects.gamma = value;
        }

        if let Some(value) = self.grayscale {
            effects.grayscale = value;
        }

        if let Some(value) = self.tint {
            effects.tint = value;
        }

        if let Some(value) = self.invert {
            effects.invert = value;
        }

        if let Some(value) = self.sepia {
            effects.sepia = value;
        }
    }
}

impl RoundingActions {
    fn apply(&self, rounding: &mut RoundingConfig) {
        if let Some(value) = self.enabled {
            rounding.enabled = value;
        }

        if let Some(value) = self.top_left {
            rounding.top_left = value;
        }
        
        if let Some(value) = self.top_right {
            rounding.top_right = value;
        }

        if let Some(value) = self.bottom_left {
            rounding.bottom_left = value;
        }

        if let Some(value) = self.bottom_right {
            rounding.bottom_right = value;
        }
    }
}

impl ClippingActions {
    fn apply(&self, clipping: &mut ClippingConfig) {
        if let Some(value) = self.enabled {
            clipping.enabled = value;
        }

        if let Some(value) = self.clip_content {
            clipping.clip_content = value;
        }

        if let Some(value) = self.clip_background {
            clipping.clip_background = value;
        }

        if let Some(value) = self.clip_shadow {
            clipping.clip_shadow = value;
        }
    }
}

impl AnimationActions {
    fn apply(&self, window: &mut Window) {
        if let Some(value) = self.enabled {
            window.animation.enabled = value;
        }

        if let Some(action) = &self.open {
            action.apply(&mut window.animation.open);
        }

        if let Some(action) = &self.close {
            action.apply(&mut window.animation.close);
        }

        if let Some(action) = &self.move_ {
            action.apply(&mut window.animation.move_);
        }

        if let Some(action) = &self.resize {
            action.apply(&mut window.animation.resize);
        }

        if let Some(action) = &self.focus {
            action.apply(&mut window.animation.focus);
        }
    }
}

impl AnimationConfigActions {
    fn apply(&self, animation: &mut AnimationConfig) {
        if let Some(value) = self.enabled {
            animation.enabled = value;
        }

        if let Some(value) = self.duration_ms {
            animation.duration_ms = value;
        }

        if let Some(value) = &self.easing {
            animation.easing = match value {
                Easing::Linear => Easing::Linear,
                
                Easing::EaseIn => Easing::EaseIn,
                Easing::EaseOut => Easing::EaseOut,
                Easing::EaseInOut => Easing::EaseInOut,

                Easing::EaseInCubic => 
                    Easing::EaseInCubic,

                Easing::EaseOutCubic =>
                    Easing::EaseOutCubic,

                Easing::EaseInOutCubic =>
                    Easing::EaseInOutCubic,

                Easing::EaseInQuart =>
                    Easing::EaseInQuart,

                Easing::EaseOutQuart =>
                    Easing::EaseOutQuart,

                Easing::EaseInOutQuart =>
                    Easing::EaseInOutQuart,

                Easing::Spring {
                    stiffness,
                    damping,
                    mass,
                } => Easing::Spring {
                    stiffness: *stiffness,
                    damping: *damping,
                    mass: *mass
                },
            };
        }

        if let Some(value) = self.opacity {
            animation.opacity = value;
        }

        if let Some(value) = self.scale {
            animation.scale = value;
        }

        if let Some(value) = self.translation {
            animation.translation = value;
        }

        if let Some(value) = self.rotation {
            animation.rotation = value;
        }
    }
}

