use ray_tracing_utility::{HittableList, Point3, Ray, Sphere, Vec3};
use std::env;
use std::fs;
use std::process::Command;
use std::rc::Rc;

fn main() -> std::io::Result<()> {
    // Image
    let path = env::current_dir()?.join("out");
    let img_pmm = path.join("img.ppm");
    let img_png = path.join("img.png");
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;

    // World
    let sp_1 = Sphere::new(Point3::from((0, 0, -1)), 0.5);
    let sp_2 = Sphere::new(Point3::from((0.0, -100.5, 0.0)), 100.0);
    let mut world = HittableList::default();
    world.add(Rc::new(sp_1));
    world.add(Rc::new(sp_2));

    // Camera
    let viewport_height = 2f64;
    let viewport_width = aspect_ratio * viewport_height as f64;
    let focal_length = 1;

    let origin = Point3::from((0, 0, 0));
    let horizontal = Vec3::from((viewport_width, 0.0, 0.0));
    let vertical = Vec3::from((0.0, viewport_height, 0.0));
    let lower_left_corner =
        origin - horizontal / 2f64 - vertical / 2f64 - Vec3::from((0, 0, focal_length));

    let mut img_content = vec![format!("P3\n{} {}\n255", width, height)];
    for j in (0..height).rev() {
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;
            img_content.push(
                Ray {
                    origin,
                    direction: lower_left_corner + horizontal * u + vertical * v - origin,
                }
                .calc_color(&world)
                .to_color_string(),
            )
        }
    }
    img_content.push("\n".to_string());

    fs::write(&img_pmm, &img_content.join("\n")).expect("Unable to write file");
    Command::new("convert")
        .arg(img_pmm)
        .arg(img_png)
        .status()
        .expect("failed to execute process");
    eprintln!("Done.");
    Ok(())
}
