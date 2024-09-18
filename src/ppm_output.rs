fn compute(n: i64, ratio: i64, max: i64) -> i64 {
    (((n as f64 / (ratio - 1) as f64) * (max as f64 + 0.999))) as i64
}
pub fn out_pixels(img_size: &[i64; 2], max: i64) {
    println!("P3\n{} {}\n{}", img_size[0], img_size[1], max);

    for y in 0..img_size[1] {
        eprintln!("Scanlines remaining: {} ", img_size[1] - y);
        // std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
        for x in 0..img_size[0] {
            let r = compute(x, img_size[0], max);
            let g = compute(y, img_size[1], max);
            let b = 0;

            // let ir = (255.999 * r as f64) as i64;
            // let ig = (255.999 * g as f64) as i64;
            // let ib = (255.999 * b as f64) as i64;

            println!("{} {} {}", r, g, 0);

            // std::cout << ir << ' ' << ig << ' ' << ib << '\n'; 
        }
    }
    eprintln!("Done.");
}
