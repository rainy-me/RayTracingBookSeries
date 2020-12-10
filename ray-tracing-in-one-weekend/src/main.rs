use ray_tracing_utility::*;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::process::Command;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {
    // Image
    let path = env::current_dir()?.join("out");
    let img_pmm = path.join("ray-tracing-in-one-weekend.ppm");
    let img_png = path.join("ray-tracing-in-one-weekend.png");
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let sp_1 = Sphere::new(Point3::from((0, 0, -1)), 0.5);
    let sp_2 = Sphere::new(Point3::from((0.0, -100.5, 0.0)), 100.0);
    let mut world = HittableList::default();
    world.add(Arc::new(sp_1));
    world.add(Arc::new(sp_2));

    // Camera
    let camera = Camera::new();
    let all = (width * height) as f64 / 100.0;
    let count = Arc::new(Mutex::new(0.0));
    let mut img_str = format!("P3\n{} {}\n255\n", width, height);
    let img_content = (0..height)
        .rev()
        .flat_map(|j| (0..width).map(|i| (i, j)).collect::<Vec<(i32, i32)>>())
        .collect::<Vec<(i32, i32)>>()
        .par_iter()
        .map(|&(i, j)| {
            let mut pixel_color = Color::from((0, 0, 0));
            for _ in 0..samples_per_pixel {
                let u = i as f64 / width as f64;
                let v = j as f64 / height as f64;
                pixel_color += camera.get_ray(u, v).calc_color(&world, max_depth)
            }
            let mut c = count.lock().unwrap();
            *c += 1.0;
            println!("{:.2}%", *c / all);
            pixel_color.to_color_string(samples_per_pixel)
        })
        .collect::<Vec<String>>()
        .join("\n");
    img_str.push_str(&img_content);
    img_str.push_str("\n");

    fs::write(&img_pmm, &img_str).expect("Unable to write file");
    Command::new("convert")
        .arg(img_pmm)
        .arg(img_png)
        .status()
        .expect("failed to execute process");
    eprintln!("Done.");
    Ok(())
}
