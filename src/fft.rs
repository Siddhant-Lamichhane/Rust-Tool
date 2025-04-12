use num_complex::Complex;
use std::f64::consts::PI;

// Panics if the length of `input` is not a power of two.
pub fn fft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    if n <= 1 {
        return input.to_vec();
    }
    if n % 2 != 0 {
        panic!("FFT input length must be a power of two");
    }

    // Divide: split input into even and odd elements.
    let even: Vec<Complex<f64>> = input.iter().step_by(2).cloned().collect();
    let odd: Vec<Complex<f64>> = input.iter().skip(1).step_by(2).cloned().collect();

    // Conquer: recursively compute FFT on even and odd parts.
    let fft_even = fft(&even);
    let fft_odd = fft(&odd);

    // Combine: compute the FFT for the whole array.
    let mut result = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n/2 {
        let theta = -2.0 * PI * k as f64 / n as f64;
        let twiddle = Complex::new(theta.cos(), theta.sin());
        result[k] = fft_even[k] + twiddle * fft_odd[k];
        result[k + n/2] = fft_even[k] - twiddle * fft_odd[k];
    }

    result
}