use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufReader, BufRead};
use plotters::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};
extern crate nalgebra as na;
pub use na::{Vector3, Rotation3};
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

fn apply_filter(fft_result: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut filtered_result = Vec::with_capacity(fft_result.len());

    for (k, value) in fft_result.iter().enumerate() {
        let a: f64 = 0.001;
        let a1 = -2.0;
        let a2 = 1.0;
        let phi = calculate_frequency(100.0, 500, k);

        // Apply the filter Fh(φ)
        let filter = 4.0 * a * a / (a2 + a1 * (-phi).exp() + (-2.0 * -phi).exp());
        filtered_result.push(*value * Complex::new(filter, 0.0));
    }

    filtered_result
}

fn fft_with_filter(data: &[f64]) -> Vec<f64> {
    let mut complex_data: Vec<Complex<f64>> = data.iter().map(|&x| Complex::new(x, 0.0)).collect();

    fft_self(&mut complex_data);

    // Apply the filter to the FFT result
    let filtered_fft = apply_filter(&complex_data);

    let magnitude: Vec<f64> = filtered_fft.iter().map(|&c| c.norm() / 500.0).collect();
    magnitude
}
struct Filter {
    a: [f64; 2], // Coefficients a(1) and a(2)
    b: f64,       // Coefficient b(0)
    x_history: [f64; 2], // x(n-1) and x(n-2) history
}

impl Filter {
    // Initialize the filter with coefficients
    fn new(a1: f64, a2: f64, b0: f64) -> Self {
        Filter {
            a: [a1, a2],
            b: b0,
            x_history: [0.0, 0.0],
        }
    }

    // Apply the filter to an input sample x(n)
    fn filter(&mut self, x_n: f64 ) -> f64 {
        // Calculate the new output y(n) using the difference equation
        let y_n = (self.b * x_n - self.a[0] * self.x_history[0] - self.a[1] * self.x_history[1]);

        // Update the history for the next iteration
        self.x_history[1] = self.x_history[0];
        self.x_history[0] = y_n;

        y_n // Return the filtered output y(n)
    }
}
fn main() {
    let file_path: &str = "src/lab2.txt";
    println!("In file {}", file_path);

    let data = load_from_file(file_path);
    plot("plot.png", &data, -0.0, 100.0, 100.0);

    let magnitude = fft(&data);

    let fft_size = data.len();
    //let mut output_data = vec![Complex::new(0.0, 0.0); fft_size];

    //let frequency_spectrum: Vec<f64> = magnitude.clone();

    //let threshold = 0.5; // Adjust this threshold based on your specific data
    //let mut peaks: Vec<usize> = Vec::new();

    //for i in 1..((fft_size -1) / 2  ) {
    //   if frequency_spectrum[i] > frequency_spectrum[i - 1] && frequency_spectrum[i] > frequency_spectrum[i + 1] {
    //       if frequency_spectrum[i] > threshold {
    //            peaks.push(i);
    //       }
    //   }
    //}

    // Display the peak frequencies
    //for peak_index in peaks {
    //    let frequency = calculate_frequency(100.0, 500 , peak_index);
    //    let peak_value = frequency_spectrum[peak_index];
    //    println!("Peak at frequency: {}", frequency);
    //}



    println!("{:?}", magnitude);
    let a1 = -2.0;
    let a2 = 1.0;
    let b0 = 4.0*0.001* 0.001;
    let mut my_filter = Filter::new(a1, a2, b0);
    let filtered_data: Vec<f64> = data.iter().map(|&x| my_filter.filter(x)).collect();
    println!("{:?}", filtered_data);
    plot("plot5.png", &filtered_data, 0.0, 10.0,100.0);

}