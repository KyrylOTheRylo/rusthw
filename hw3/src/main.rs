extern crate image;
extern crate rustfft;


use image::{GrayImage, Luma};
use rustfft::{FftPlanner, num_complex::Complex, FftDirection};



fn median_filter(input: &GrayImage, kernel_size: u32) -> GrayImage {
    let mut output = input.clone();

    let pad = kernel_size as i32 / 2;
    let (width, height) = input.dimensions();

    for y in pad..(height as i32 - pad) {
        for x in pad..(width as i32 - pad) {
            let mut neighbors = Vec::new();

            // Collect the values in the kernel
            for ky in -pad..=pad {
                for kx in -pad..=pad {
                    let pixel = input.get_pixel((x + kx) as u32, (y + ky) as u32).0[0];
                    neighbors.push(pixel);
                }
            }

            // Sort and find the median value
            neighbors.sort_unstable();
            let median = neighbors[neighbors.len() / 2];
            output.put_pixel(x as u32, y as u32, Luma([median]));
        }
    }

    output
}

fn compute_cepstrum(input: &GrayImage, a: f32, b: f32) -> GrayImage {
    let (width, height) = input.dimensions();
    let mut planner = FftPlanner::new();

    // Prepare data for FFT
    let mut data: Vec<Complex<f32>> = input
        .pixels()
        .map(|p| Complex::new(p.0[0] as f32, 0.0))
        .collect();

    // Perform Fourier Transform
    let fft = planner.plan_fft_forward(data.len());
    fft.process(&mut data);

    // Compute the cepstrum
    let cepstrum_data: Vec<Complex<f32>> = data
        .iter()
        .map(|&x| {
            let magnitude = x.norm();
            let phase = x.arg();
            Complex::from_polar((b * magnitude + a).ln(), phase) // Updated line
        })
        .collect();

    // Perform Inverse Fourier Transform
    let ifft = planner.plan_fft_inverse(cepstrum_data.len());
    let mut inverse_data = cepstrum_data.clone();
    ifft.process(&mut inverse_data);

    // Normalize and convert back to image format
    let max_val = inverse_data.iter().map(|x| x.norm()).fold(0.0_f32, f32::max);
    let min_val = inverse_data.iter().map(|x| x.norm()).fold(f32::MAX, f32::min);
    let mut output = GrayImage::new(width, height);
    for (i, &pixel) in inverse_data.iter().enumerate() {
        let normalized_val = ((pixel.norm() - min_val) / (max_val - min_val) * 255.0).max(0.0) as u8;
        output.put_pixel((i % width as usize) as u32, (i / width as usize) as u32, Luma([normalized_val]));
    }

    output
}
fn main() {
    // Load your image
    let  img = image::open("src/saturn3.GIF").unwrap().to_luma8();


    let filtered_img = median_filter(&img, 3);

    filtered_img.save("src/output.png").unwrap();

    let mut img2 = image::open("src/moon.tif").unwrap().to_luma8();
    let cepstrum = compute_cepstrum(&img2, 1.0, 0.1);
    cepstrum.save("src/output2.png").unwrap();
}