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
    let aspect_ratio = 16. / 9.;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 100;

    // World
    let r: f64 = (std::f64::consts::PI / 4.).cos();
    let mut world = HittableList::default();

    let material_ground = Lambertian::new(220, 220, 220).as_ref();
    let material_left = Metal::new(8, 217, 214, 0.6).as_ref();
    let material_center = Metal::new(220, 220, 220, 0.3).as_ref();
    let material_right = Lambertian::new(255, 46, 99).as_ref();

    world.add(Arc::new(Sphere {
        center: Point3::new(0., -100.5, -1.),
        radius: 100.,
        material: material_ground,
    }));
    world.add(Arc::new(Sphere {
        center: Point3::new(0., 0., -1.),
        radius: 0.5,
        material: material_center,
    }));
    world.add(Arc::new(Sphere {
        center: Point3::new(-r, 0., -1.),
        radius: -0.5,
        material: material_left,
    }));
    world.add(Arc::new(Sphere {
        center: Point3::new(r, 0., -1.),
        radius: 0.5,
        material: material_right,
    }));

    let look_from = Point3::new(3., 3., 2.);
    let look_at = Point3::new(0., 0., -1.);

    // Camera
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0., 1., 0.),
        25.0,
        aspect_ratio,
        2.0,
        (look_from - look_at).length(),
    );

    let all = (width * height) as f64 / 100.;
    let count = Arc::new(Mutex::new(0.));
    let mut img_str = format!("P3\n{} {}\n255\n", width, height);
    let img_content = (0..height)
        .rev()
        .flat_map(|j| (0..width).map(|i| (i, j)).collect::<Vec<_>>())
        .collect::<Vec<(i32, i32)>>()
        .par_iter()
        .map(|&(i, j)| {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = i as f64 / width as f64;
                let v = j as f64 / height as f64;
                pixel_color += camera.get_ray(u, v).calc_color(&world, max_depth)
            }
            let mut c = count.lock().unwrap();
            *c += 1.;
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
