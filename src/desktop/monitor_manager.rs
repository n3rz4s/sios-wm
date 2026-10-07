use super::monitor::{
    MonitorId,
    Monitor,
};


pub struct MonitorManager {
    pub monitors: Vec<Monitor>,
}


impl MonitorManager {
    pub fn new() -> Self {
        Self {
            monitors: Vec::new(),
        }
    }

    pub fn add_monitor(&mut self, monitor: Monitor) {
        self.monitors.push(monitor);
    }

    pub fn remove_monitor(&mut self, id: MonitorId) {
        self.monitors.retain(|monitor| monitor.id != id);
    }

    pub fn get_monitor(&self, id: MonitorId) -> Option<&Monitor> {
        self.monitors.iter().find(|monitor| monitor.id == id)
    }

    pub fn get_monitor_mut(&mut self, id: MonitorId) -> Option<&mut Monitor> {
        self.monitors.iter_mut().find(|monitor| monitor.id == id)
    }

    pub fn get_monitor_by_name(&self, name: &str) -> Option<&Monitor> {
        self.monitors
            .iter()
            .find(|monitor| monitor.name == name)
    }

    pub fn get_monitor_by_name_mut(
        &mut self,
        name: &str
    ) -> Option<&mut Monitor> {
        self.monitors
            .iter_mut()
            .find(|monitor| monitor.name == name)
    }
}