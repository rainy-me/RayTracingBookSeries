use ray_tracing_utility::{Point3, Ray, Vec3};
use rayon::prelude::*;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2;
    let viewport_width = (aspect_ratio * viewport_height as f64) as i32;
    let focal_length = 1;

    let origin = Point3::from((0, 0, 0));
    let horizontal = Vec3::from((viewport_width, 0, 0));
    let vertical = Vec3::from((0, viewport_height, 0));
    let lower_left_corner =
        origin - horizontal.div(2f64) - vertical.div(2f64) - Vec3::from((0, 0, focal_length));

    println!("P3\n{} {}\n255", width, height);

    (0..height)
        .rev()
        .flat_map(|j| (0..width).map(|i| (i, j)).collect::<Vec<(i32, i32)>>())
        .collect::<Vec<(i32, i32)>>()
        .par_iter()
        .map(|&(i, j)| {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            Ray {
                origin,
                direction: lower_left_corner + horizontal.mul(u) + vertical.mul(v) - origin,
            }
            .color_vec()
            .to_i32_vec()
        })
        .map(|r| r.print()).collect::<()>();
    eprintln!("Done.");
}
