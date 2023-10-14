use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufReader, BufRead};
use plotters::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};

fn load_from_file(file_path: &str) -> Vec<f64>{
    let file = File::open(file_path).expect("file wasn't found.");
    let reader: BufReader<File> = BufReader::new(file);
    let mut data: Vec<f64> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let numbers: Vec<f64> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<f64>().ok())
                .collect();

            data.extend(numbers);
        }
    }

    data
}

fn fft_self(x: &mut Vec<Complex<f64>>)  {
    let nс = x.len();
    let mut xc: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); nс];

    for k in 0..nс {
        let mut a: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); nс];
        for n in 0..nс {
            a[n] = x[n] * Complex::new(0.0, - 2.0 * PI * k as f64 * n as f64 / nс as f64).exp();
        }
        xc[k] = a.iter().sum() ;
    }
    *x = xc.clone();
}

fn fft(data: &[f64]) -> Vec<f64> {

    let mut complex_data: Vec<Complex<f64>> = data.iter().map(|&x| Complex::new(x, 0.0)).collect();

    fft_self(&mut complex_data);


    let magnitude: Vec<f64> = complex_data.iter().map(|&c| c.norm()/ 500.0).collect() ;
    magnitude

}
fn  plot(outputname: &str,data:&Vec<f64>, min: f64, max: f64, div: f64){
    let root = BitMapBackend::new(outputname, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Data Plot", ("Arial", 30).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..data.len()  as f64 / div, min..max)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(x, y)| ((x as f64/100.0) , *y)),
            &RED,
        ))
        .unwrap();
}
fn  plot_fft(outputname: &str,data:&Vec<f64>, min: f64, max: f64, sampling_rate: f64, fft_size: usize){
    let root = BitMapBackend::new(outputname, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Data Plot", ("Arial", 30).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..50.0, min..max)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            data[0..data.len()/2 ].iter().enumerate().map(|(x, y)| ((calculate_frequency(sampling_rate, fft_size , x)) , *y)),
            &RED,
        ))
        .unwrap();
}

fn calculate_frequency( sampling_rate: f64, fft_size: usize, peak_bin_index: usize) -> f64 {

    let frequency_resolution = sampling_rate  / fft_size as f64;


    let actual_frequency = peak_bin_index as f64 * frequency_resolution;

    actual_frequency
}
fn main() {
    let file_path: &str = "src/lab2.txt";
    println!("In file {}", file_path);

    let data = load_from_file(file_path);
    plot("plot.png", &data, -150.0 , 150.0, 100.0);

    let magnitude = fft(&data);

    let fft_size = data.len();
    //let mut output_data = vec![Complex::new(0.0, 0.0); fft_size];

    let frequency_spectrum: Vec<f64> = magnitude.clone();

    let threshold = 0.5; // Adjust this threshold based on your specific data
    let mut peaks: Vec<usize> = Vec::new();

    for i in 1..((fft_size -1) / 2  ) {
        if frequency_spectrum[i] > frequency_spectrum[i - 1] && frequency_spectrum[i] > frequency_spectrum[i + 1] {
            if frequency_spectrum[i] > threshold {
                peaks.push(i);
            }
        }
    }

    // Display the peak frequencies
    for peak_index in peaks {
        let frequency = calculate_frequency(100.0, 500 , peak_index);
        let peak_value = frequency_spectrum[peak_index];
        println!("Peak at frequency: {} Hz with magnitude: {}", frequency, peak_value);
    }


    plot_fft("plot2.png", &magnitude, 0.0 , 10.0, 100.0, 500);
}
