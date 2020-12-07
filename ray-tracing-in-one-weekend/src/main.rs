fn main() {
    let (width, height) = (256, 256);
    println!("P3\n{} {}\n255\n", width, height);
    for j in (0..height).rev() {
        for i in 0..width {
            println!(
                "{}",
                vec![
                    i as f64 / (width - 1) as f64,
                    j as f64 / (height - 1) as f64,
                    0.25,
                ]
                .iter()
                .map(|n| ((256f64 * n) as i32).to_string())
                .collect::<Vec<_>>()
                .join(" ")
            );
        }
    }
}
