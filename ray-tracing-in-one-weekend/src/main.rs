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
    let aspect_ratio = 3. / 2.;
    let width = 1200;
    let height = (width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 100;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::new(0., 0., 0.);

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0., 1., 0.),
        20.0,
        aspect_ratio,
        0.1,
        10.,
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

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    world.add(Arc::new(Sphere {
        center: Point3::new(0., -1000., 0.),
        radius: 1000.0,
        material: Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }
        .as_ref(),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let center = Point3::new(
                a as f64 + 0.9 * rand_f64(),
                0.2,
                b as f64 + 0.9 * rand_f64(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            let material: Arc<dyn Material> = match true {
                _ if choose_mat < 0.8 => {
                    // diffuse
                    Lambertian {
                        albedo: Color::random() * Color::random(),
                    }
                    .as_ref()
                }
                _ if choose_mat < 0.95 => {
                    // metal
                    Metal {
                        albedo: Color::random_in_range(0.5, 1.),
                        fuzz: rand_f64_in_range(0., 0.5),
                    }
                    .as_ref()
                }
                _ => {
                    // glass
                    Dielectric::new(1.5).as_ref()
                }
            };
            world.add(Arc::new(Sphere {
                center,
                radius: 0.2,
                material,
            }));
        }
    }

    world.add(Arc::new(Sphere {
        center: Point3::new(0., 1., 0.),
        radius: 1.0,
        material: Dielectric::new(1.5).as_ref(),
    }));

    world.add(Arc::new(Sphere {
        center: Point3::new(-4., 1., 0.),
        radius: 1.0,
        material: Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }
        .as_ref(),
    }));

    world.add(Arc::new(Sphere {
        center: Point3::new(4., 1., 0.),
        radius: 1.0,
        material: Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }
        .as_ref(),
    }));

    world
}
