pub mod donut_utils;

use donut_utils::*;

pub fn draw(scene: Vec<Point>, config: CameraConfig) {
    let screen = scene
        .into_iter()
        .map(|point| point.project(config.screen_dist));

    let mut disp_acc = vec![vec![0; config.x_res]; config.y_res];

    for pxl in screen {
        if pxl.x.abs() >= config.x_fov / 2.0 {
            continue;
        }
        if pxl.y.abs() >= config.y_fov / 2.0 {
            continue;
        }

        println!("x:{}, y:{}", pxl.x, pxl.y);
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
