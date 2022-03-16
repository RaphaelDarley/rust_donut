use rust_donut::donut_utils::*;
use rust_donut::*;
use std::{thread, time};

fn main() {
    // let config = CameraConfig::new(2.0, 100, 50, 10.0, 10.0);
    let config = CameraConfig::new(2.0, 20, 10, 10.0, 10.0);

    for i in 0..10 {
        draw(circle_gen(0.0, 0.0, i as f64, 10.0, 500), &config).unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn circle_gen(centre_x: f64, centre_y: f64, centre_z: f64, radius: f64, res: usize) -> Vec<Point> {
    let mut acc = vec![];
    let mut theta: f64 = 0.0;
    let theta_step = (2f64 * std::f64::consts::PI) / res as f64;

    for _ in 0..res {
        acc.push(Point::new(
            theta.sin() * radius + centre_x,
            theta.cos() * radius + centre_y,
            centre_z,
        ));
        theta += theta_step;
    }

    acc
}
