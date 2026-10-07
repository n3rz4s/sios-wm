use smithay::{
    utils::{
        Logical,
        Point,
        Size,
    },
    wayland::shell::{
        wlr_layer::LayerSurface,
        xdg::ToplevelSurface,
    },
};

use super::{
    monitor::{
        Monitor,
        MonitorId,
    },
    monitor_manager::MonitorManager,
    renderer::DesktopRenderer,
    window::{
        Window,
        WindowId,
    },
    window_rules::WindowRules,
};

use crate::backend::drm::DrmOutput;

use sios_wm_api::MonitorConfig;

pub struct Desktop {
    pub monitors: MonitorManager,
    pub renderer: DesktopRenderer,

    pub windows: Vec<Window>,
    pub layers: Vec<LayerSurface>,

    pub window_rules: WindowRules,

    next_window_id: u64,
}

impl Desktop {
    pub fn new() -> Self {
        Self {
            monitors: MonitorManager::new(),
            renderer: DesktopRenderer::new(),

            windows: Vec::new(),
            layers: Vec::new(),

            window_rules: WindowRules::new(),

            next_window_id: 1,
        }
    }

    pub fn add_monitors(
        &mut self,
        outputs: &[DrmOutput],
    ) {
        for (index, output) in outputs.iter().enumerate() {
            let resolution = Size::from((
                output.mode.size().0 as i32,
                output.mode.size().1 as i32,
            ));

            let monitor = Monitor::new(
                MonitorId(index),
                output.name.clone(),
                output.physical_size.into(),
                resolution,
                output.mode.vrefresh() as i32,
            );

            eprintln!(
                "SIOS: Monitor added: {} {}x{} @ {} Hz, physical={}x{} mm",
                monitor.name,
                monitor.resolution.w,
                monitor.resolution.h,
                monitor.refresh,
                monitor.physical_size.w,
                monitor.physical_size.h,
            );

            self.add_monitor(monitor);
        }
    }

    pub fn apply_monitor_config(
        &mut self,
        config: &MonitorConfig
    ) {
        for settings in &config.monitors {
            let Some(monitor) = self.monitors
                .get_monitor_by_name_mut(&settings.name)
            else {
                println!(
                    "SIOS: Monitor '{}' not found.",
                    settings.name,
                );
                continue;
            };

            monitor.set(
                settings.scale,
                settings.gaps,
                monitor.position,
                Size::from((
                    settings.width,
                    settings.height,
                )),
                settings.refresh,
                settings.enabled,
            );

            eprintln!(
                "SIOS: Applied config to {}: scale={}, gaps={}, resolution={}x{}, refresh={}, enabled={:?}",
                monitor.name,
                monitor.scale,
                monitor.gaps,
                monitor.resolution.w,
                monitor.resolution.h,
                monitor.refresh,
                monitor.enabled,
            );
        }
    }

    pub fn add_monitor(&mut self, monitor: Monitor) {
        self.monitors.add_monitor(monitor);
    }

    pub fn remove_monitor(&mut self, id: MonitorId) {
        self.monitors.remove_monitor(id);
    }

    pub fn add_window(
        &mut self,
        surface: ToplevelSurface,
    ) -> WindowId {
        let id = WindowId(self.next_window_id);

        self.next_window_id += 1;

        let mut window = Window::new(
            id,
            surface,
        );

        eprintln!(
            "SIOS: Window {:?} surface = {:?}",
            id,
            window.surface().wl_surface(),
        );

        self.window_rules.apply(&mut window);

        window.set_position(
            Point::from((
                100,
                100,
            )),
        );

        eprintln!(
            "SIOS: Window {:?}: mapped={}, border={}, width={}",
            window.id,
            window.state.mapped,
            window.appearance.border.enabled,
            window.appearance.border.width,
        );

        window.map();

        eprintln!(
            "SIOS: mapped={}",
            window.state.mapped,
        );

        if self.windows.is_empty() {
            window.focus();
        }

        eprintln!(
            "SIOS: Window Added: id={:?}",
            id,
        );

        self.windows.push(window);

        id
    }

    pub fn remove_window(
        &mut self,
        id: WindowId,
    ) -> Option<Window> {
        let index = self.windows
            .iter()
            .position(|window| window.id() == id)?;

        let window = self.windows.remove(index);

        eprintln!(
            "SIOS: Window removed: id={:?}",
            id,
        );

        Some(window)
    }

    pub fn set_window_rules(
        &mut self,
        window_rules: WindowRules,
    ) {
        self.window_rules = window_rules;
    }

    pub fn window(
        &self,
        id: WindowId
    ) -> Option<&Window> {
        self.windows
            .iter()
            .find(|window| window.id() == id)
    }

    pub fn window_mut(
        &mut self,
        id: WindowId
    ) -> Option<&mut Window> {
        self.windows
            .iter_mut()
            .find(|window| window.id() == id)
    }

    pub fn window_count(
        &self,
    ) -> usize {
        self.windows.len()
    }

    pub fn mapped_windows(
        &self
    ) -> impl Iterator<Item = &Window> {
        self.windows
            .iter()
            .filter(|window| {
                window.is_mapped()
                    && window.is_visible()
            })
    }

    pub fn focused_window(
        &self,
    ) -> Option<WindowId> {
        self.windows
            .iter()
            .find(|window| window.is_focused())
            .map(|window| window.id())
    }

    pub fn focus_window(
        &mut self,
        id: WindowId,
    ) -> bool {
        let Some(index) = self.windows
            .iter()
            .position(|window| window.id() == id)
        else {
            return false;
        };

        if !self.windows[index].is_focusable() {
            return false;
        }

        for window in &mut self.windows {
            if window.id() != id {
                window.unfocus()
            }
        }

        self.windows[index].focus();

        eprintln!(
            "SIOS: Window focused: id={:?}",
            id,
        );

        true
    }

    pub fn raise_window(
        &mut self,
        id: WindowId,
    ) -> bool {
        let Some(index) = self.windows
            .iter()
            .position(|window| window.id() == id)
        else {
            return false;
        };

        if index == self.windows.len() - 1 {
            return true;
        }

        let window = self.windows.remove(index);

        self.windows.push(window);

        true
    }

    pub fn unfocus(
        &mut self,
    ) -> bool {
        let mut found = false;

        for window in &mut self.windows {
            if window.is_focused() {
                window.unfocus();
                found = true;
            }
        }

        found
    }

    pub fn window_at(
        &self,
        position: Point<f64, Logical>,
    ) -> Option<WindowId> {
        self.windows
            .iter()
            .rev()
            .find(|window| {
                if !window.is_mapped()
                    || !window.is_visible()
                {
                    return false;
                }

                let window_position =
                    window.position();

                let window_size =
                    window.size();

                let x = position.x;
                let y = position.y;

                let left =
                    window_position.x as f64;
                
                let top =
                    window_position.y as f64;

                let right =
                    left + window_size.w as f64;

                let bottom =
                    top + window_size.h as f64;

                x >= left
                    && x < right
                    && y >= top
                    && y < bottom
            })
            .map(|window| window.id())
    }

    pub fn clear_focus(
        &mut self,
    ) {
        for window in &mut self.windows {
            window.unfocus();
        }
    }

    pub fn add_layer(
        &mut self,
        surface: LayerSurface,
    ) {
        eprintln!(
            "SIOS: Layer surface added.",
        );

        self.layers.push(surface);
    }

    pub fn remove_layer(
        &mut self,
        surface: &LayerSurface
    ) {
        self.layers.retain(|layer| {
            layer.wl_surface() != surface.wl_surface()
        });

        eprintln!(
            "SIOS: Layer surface removed."
        );
    }

    pub fn layers(&self) -> impl Iterator<Item = &LayerSurface> {
        self.layers.iter()
    }

    pub fn place_window(
        &mut self,
        id: WindowId,
        position: Point<i32, Logical>,
    ) -> bool {
        let Some(window) = self.window_mut(id) else {
            return false;
        };

        window.set_position(position);

        true
    }

    pub fn resize_window(
        &mut self,
        id: WindowId,
        size: Size<i32, Logical>,
    ) -> bool {
        let Some(window) = self.window_mut(id) else {
            return false;
        };

        window.set_size(size);

        true
    }

    pub fn window_rules(
        &self
    ) -> &WindowRules {
        &self.window_rules
    }

    pub fn window_rules_mut(
        &mut self,
    ) -> &mut WindowRules {
        &mut self.window_rules
    }

    pub fn monitor_for_window(
        &self,
        id: WindowId,
    ) -> Option<&Monitor> {
        let window = self.window(id)?;

        let monitor_id = window.monitor()?;

        self.monitors.get_monitor(monitor_id)
    }
}