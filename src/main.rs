fn main() {
    // let point1 = Point{x: 0.0, y:5.0, z: 10.0,};
    // println!("{:?} projects to pixel {:?}", point1, point1.project(2.0));
    // draw(vec![point1.project(2.0)])
    draw(vec![Point{x: 0.0, y:5.0, z: 10.0,}], 2.0);
}


fn draw(scene: Vec<Point>, screen_dist: f32) {

    let screen = scene.into_iter().map(|point|{point.project(screen_dist)});

    let x_resolution = 20;
    let y_resolution = 10;
    let x_fov = 10.0;
    let y_fov = 10.0;
    let x_step = x_fov/x_resolution as f32;
    let y_step = y_fov/y_resolution as f32;

    let mut disp_acc = vec![vec![0;x_resolution];y_resolution];
    
    // println!("{:?}", disp_acc);
    
    println!("{:?}", screen);

    // println!("len: {}", screen.len());

    for pxl in screen {
        if pxl.x.abs() >= x_fov/2.0 {continue;}
        if pxl.y.abs() >= y_fov/2.0 {continue;}

        println!("x pos from right:{}", (pxl.x + x_fov/2.0)       );
        println!("y pos from top:{}", (y_fov/2.0 - pxl.y)       );

        disp_acc[((y_fov/2.0 - pxl.y) /y_step).floor() as usize][((pxl.x + x_fov/2.0) /x_step).floor() as usize] +=1;

    }

    // println!("{:?}", disp_acc);

    for row in disp_acc{
        for p in row{print!("{}", p)}
        print!("\n")
    }
    
}



#[derive(Debug)]
struct Pixel {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Point{
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn project(&self, screen_dist: f32) -> Pixel {
        let scale = screen_dist / self.z;
        Pixel{x: self.x * scale, y: self.y * scale}
    }
}