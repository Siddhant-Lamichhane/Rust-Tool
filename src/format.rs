use num_complex::Complex;

pub fn format_complex(c: &Complex<f64>) -> String {
    // Format with two decimal places. Adjust precision as needed.
    if c.im < 0.0 {
        format!("{:.2}{:.2}i", c.re, c.im)
    } else {
        format!("{:.2}+{:.2}i", c.re, c.im)
    }
}