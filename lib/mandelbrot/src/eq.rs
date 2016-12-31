extern crate num;

use num::complex::Complex64;

pub fn mandelbrot_divergence(x: f64, y: f64) -> f64 {
    let divergence = mandelbrot_value(Complex64 { re: 0.0, im: 0.0 }, Complex64::new(x, y), 25)
        .norm();
    if divergence < 2.0 {
        return 0.0;
    } else {
        return divergence;
    }
}

fn mandelbrot_value(prev: Complex64, point: Complex64, iteration: u16) -> Complex64 {
    if iteration <= 0 {
        return prev;
    } else if prev.norm() > 2.0 {
        return prev;
    } else {
        return mandelbrot_value((prev * prev) + point, point, iteration - 1);
    }
}
