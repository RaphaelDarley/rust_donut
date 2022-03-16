use donut_utils::*;

fn main() {
    draw(
        vec![Point {
            x: 0.0,
            y: 5.0,
            z: 10.0,
        }],
        CameraConfig::new(2.0, 20, 10, 10.0, 10.0),
    );
}

fn draw(scene: Vec<Point>, config: CameraConfig) {
    let screen = scene
        .into_iter()
        .map(|point| point.project(config.screen_dist));

    // let x_resolution = 20;
    // let y_resolution = 10;
    // let x_fov = 10.0;
    // let y_fov = 10.0;
    // let x_step = x_fov / x_resolution as f32;
    // let y_step = y_fov / y_resolution as f32;

    let mut disp_acc = vec![vec![0; config.x_res]; config.y_res];

    for pxl in screen {
        if pxl.x.abs() >= config.x_fov / 2.0 {
            continue;
        }
        if pxl.y.abs() >= config.y_fov / 2.0 {
            continue;
        }

        println!("x pos from right:{}", (pxl.x + config.x_fov / 2.0));
        println!("y pos from top:{}", (config.y_fov / 2.0 - pxl.y));

        disp_acc[((config.y_fov / 2.0 - pxl.y) / config.y_step).floor() as usize]
            [((pxl.x + config.x_fov / 2.0) / config.x_step).floor() as usize] += 1;
    }

    for row in disp_acc {
        for p in row {
            print!("{}", p)
        }

        print!("\n")
    }
}

mod donut_utils {

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
}
