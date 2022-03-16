pub struct CameraConfig {
    pub screen_dist: f32,
    pub x_res: usize,
    pub y_res: usize,
    pub x_fov: f32,
    pub y_fov: f32,
    pub x_step: f32,
    pub y_step: f32,
}

impl CameraConfig {
    pub fn new(
        screen_dist: f32,
        x_res: usize,
        y_res: usize,
        x_fov: f32,
        y_fov: f32,
    ) -> CameraConfig {
        CameraConfig {
            screen_dist,
            x_res,
            y_res,
            x_fov,
            y_fov,
            x_step: x_fov / x_res as f32,
            y_step: y_fov / y_res as f32,
        }
    }
}

#[derive(Debug)]
pub struct Projection {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn project(&self, screen_dist: f32) -> Projection {
        let scale = screen_dist / self.z;
        Projection {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}
