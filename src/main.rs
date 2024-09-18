pub mod ppm_output;
pub mod vec3;

fn main() {
    let n = 255;
    ppm_output::out_pixels(&[n+1, n+1], n);
}