use num::complex::Complex64;

pub fn mandelbrot_divergence(x: f64, y: f64, iterations: u32) -> Result<(), u32> {
    return mandelbrot_value(Complex64 { re: 0.0, im: 0.0 },
                            Complex64::new(x, y),
                            iterations);
}

fn mandelbrot_value(prev: Complex64, point: Complex64, iteration: u32) -> Result<(), u32> {
    if iteration <= 0 {
        return Ok(());
    } else if prev.norm() > 2.0 {
        return Err(iteration);
    } else {
        return mandelbrot_value((prev * prev) + point, point, iteration - 1);
    }
}
