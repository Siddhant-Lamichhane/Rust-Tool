use num_complex::Complex;
use std::f64::consts::PI;

/// Computes the Discrete Fourier Transform (DFT) of the input slice.
/// The input is a slice of real numbers (assumed to have zero imaginary parts).
pub fn dft(input: &[f64]) -> Vec<Complex<f64>> {
    let n = input.len();
    let mut output = Vec::with_capacity(n);
    // Convert input values into complex numbers.
    let input_complex: Vec<Complex<f64>> = input
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (i, &x) in input_complex.iter().enumerate() { //i is the index, n is the length
            let angle = -2.0 * PI * (k as f64) * (i as f64) / (n as f64);
            let twiddle = Complex::new(angle.cos(), angle.sin());
            sum += x * twiddle;
        }
        output.push(sum);
    }
    
    output
}