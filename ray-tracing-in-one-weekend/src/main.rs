use ray_tracing_utility::*;
use std::env;
use std::fs;
use std::process::Command;
use std::rc::Rc;

fn main() -> std::io::Result<()> {
    // Image
    let path = env::current_dir()?.join("out");
    let img_pmm = path.join("ray-tracing-in-one-weekend.ppm");
    let img_png = path.join("ray-tracing-in-one-weekend.png");
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World
    let sp_1 = Sphere::new(Point3::from((0, 0, -1)), 0.5);
    let sp_2 = Sphere::new(Point3::from((0.0, -100.5, 0.0)), 100.0);
    let mut world = HittableList::default();
    world.add(Rc::new(sp_1));
    world.add(Rc::new(sp_2));

    // Camera
    let camera = Camera::new();

    let mut img_content = vec![format!("P3\n{} {}\n255", width, height)];
    for j in (0..height).rev() {
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..width {
            let mut pixel_color = Color::from((0, 0, 0));
            for _ in 0..samples_per_pixel {
                let u = i as f64 / width as f64;
                let v = j as f64 / height as f64;
                pixel_color += camera.get_ray(u, v).calc_color(&world)
            }
            img_content.push(pixel_color.to_color_string(samples_per_pixel))
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
