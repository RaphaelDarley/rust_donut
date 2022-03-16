use rust_donut::donut_utils::*;
use rust_donut::draw;

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
