use smithay::utils::{
    Physical,
    Point,
    Size,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonitorId(pub usize);


#[derive(Debug, Clone)]
pub struct Monitor {
    pub id: MonitorId,
    pub name: String,

    pub physical_size: Size<i32, Physical>,

    pub resolution: Size<i32, Physical>,
    pub refresh: i32,

    pub position: Point<i32, Physical>,

    pub scale: f64,
    pub gaps: i32,

    pub enabled: bool,
}

impl Monitor {
    pub fn new(
        id: MonitorId,
        name: String,
        physical_size: Size<i32, Physical>,
        resolution: Size<i32, Physical>,
        refresh: i32,
    ) -> Self {
        Self {
            id,
            name,
            physical_size,
            resolution,
            refresh,
            position: Point::from((0, 0)),
            scale: 1.0,
            gaps: 0,
            enabled: true,
        }
    }

    pub fn set(
        &mut self,
        scale: f64,
        gaps: i32,
        position: Point<i32, Physical>,
        resolution: Size<i32, Physical>,
        refresh: i32,
        enabled: bool,
    ) {
        self.scale = scale;
        self.gaps = gaps;
        self.position = position;
        self.resolution = resolution;
        self.refresh = refresh;
        self.enabled = enabled;
    }
}