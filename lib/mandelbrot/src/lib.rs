extern crate num;
extern crate image;

mod image_create;
mod eq;

pub use image_create::gen_png;
pub use image_create::write_png;
pub use image_create::Frame;
pub use eq::mandelbrot_divergence;
