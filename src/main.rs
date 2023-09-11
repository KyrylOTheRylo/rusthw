use rustfft::{FftPlanner, num_complex::Complex};
use std::f64::consts::PI;
use rustfft::FftDirection; 
use std::fs::File;
use std::io::{BufReader, BufRead};
use plotters::prelude::*;

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
    //let size = data.len();
    //let mut planner = FftPlanner::new();
    //let fft = planner.plan_fft(size, FftDirection::Forward); 

    
    let mut complex_data: Vec<Complex<f64>> = data.iter().map(|&x| Complex::new(x, 0.0)).collect();

    //fft.process(&mut complex_data);
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

fn calculate_frequency( sampling_rate: f64, fft_size: usize, peak_bin_index: usize) -> f64 {
    
    let frequency_resolution = sampling_rate  / fft_size as f64;

    
    let actual_frequency = peak_bin_index as f64 * frequency_resolution;

    actual_frequency
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

fn approximate_signal(
    signal: &Vec<f64>,
    t_min: f64,
    t_max: f64,
    num_points: usize,
) -> Result<Vec<f64>, ()> {
    let delta_t = (t_max - t_min) / num_points as f64;
    let n = signal.len();
    let mut approximation = vec![0.0; num_points];

    for i in 0..num_points {
        let t = t_min + i as f64 * delta_t;
        let mut sum = 0.0;

        for k in 0..n {
            
            let x_k = signal[k];
            let mut  sinc_term = 1.0;
            if i != 0 {
                sinc_term = (PI * t *n as f64/ t_max 
                    - PI * k as f64)
                    .sin()
                    / (PI * t *n as f64/ t_max 
                        - PI * k as f64); 
            }
           
            sum += x_k * sinc_term;
        }

        approximation[i] = sum;
    }

    Ok(approximation)
}
fn main() {
    let file_path: &str = "src/f11.txt";
    println!("In file {}", file_path);

    let data = load_from_file(file_path);
    plot("plot.png", &data, -150.0 , 150.0, 100.0);

    let magnitude = fft(&data);




    //let actual_frequency = calculate_frequency(sampling_rate, fft_size, peak_bin_index);
    // Print or visualize the magnitude values (e.g., using plotters)
    //println!("{actual_frequency}");
    //println!("{:?}", magnitude);
    for (n,x) in magnitude[0..magnitude.len()/2].iter().enumerate(){
        if *x > 5.0 && *x < 8.0{
        
            println!("{}", n * 100/500);
        }
    }
    plot_fft("plot2.png", &magnitude, 0.0 , 10.0, 100.0, 500);
    let file_path2: &str = "src/test.txt";
    let data2 = load_from_file(file_path2);
    plot("plot3.png", &data2, -15.0 , 15.0, 100.0);
    let answer = approximate_signal(&data2, 0.0, 2.0, 400).unwrap();
    plot("plot4.png", &answer, -15.0 , 15.0, 200.0);
    let magnitude2 = fft(&answer);
    plot_fft("plot5.png", &magnitude2, 0.0 , 10.0, 200.0, 200 );
    for (n,x) in magnitude2[0..magnitude2.len()/2].iter().enumerate(){
        if *x > 1.5 && *x < 5.0
        {
        
            println!("\n{}", n );
        }
    }

}
