use ray_tracing_utility::Vec3;

fn main() {
    let (width, height) = (256, 256);
    println!("P3\n{} {}\n255", width, height);
    for j in (0..height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..width {
            Vec3::new(
                i as f64 / (width - 1) as f64,
                j as f64 / (height - 1) as f64,
                0.25,
            )
            .write_color_string();
        }
    }
    eprintln!("Done.");
}
