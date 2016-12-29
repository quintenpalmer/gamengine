extern crate num;

use num::complex::Complex32;

pub fn mandelbrot_divergence(x: f32, y: f32) -> f32 {
    let divergence = mandelbrot_value(Complex32 { re: 0.0, im: 0.0 }, Complex32::new(x, y), 25)
        .norm();
    if divergence < 2.0 {
        return 0.0;
    } else {
        return divergence;
    }
}

fn mandelbrot_value(prev: Complex32, point: Complex32, iteration: u16) -> Complex32 {
    if iteration <= 0 {
        return prev;
    } else if prev.norm() > 2.0 {
        return prev;
    } else {
        return mandelbrot_value((prev * prev) + point, point, iteration - 1);
    }
}
