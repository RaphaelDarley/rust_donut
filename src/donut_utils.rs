pub struct CameraConfig {
    pub screen_dist: f64,
    pub x_res: usize,
    pub y_res: usize,
    pub x_fov: f64,
    pub y_fov: f64,
    pub x_step: f64,
    pub y_step: f64,
}

impl CameraConfig {
    pub fn new(
        screen_dist: f64,
        x_res: usize,
        y_res: usize,
        x_fov: f64,
        y_fov: f64,
    ) -> CameraConfig {
        CameraConfig {
            screen_dist,
            x_res,
            y_res,
            x_fov,
            y_fov,
            x_step: x_fov / x_res as f64,
            y_step: y_fov / y_res as f64,
        }
    }
}

#[derive(Debug)]
pub struct Projection {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn project(&self, screen_dist: f64) -> Projection {
        let scale = screen_dist / self.z;
        Projection {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}
