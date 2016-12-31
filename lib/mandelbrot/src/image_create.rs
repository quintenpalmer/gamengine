extern crate image;

use std::fs;
use std::path;
use std::error;

use eq;

pub fn gen_png(frame: Frame) -> image::RgbaImage {
    let mut imagebuf: image::RgbaImage = image::ImageBuffer::new(frame.screen_width,
                                                                 frame.screen_height);
    for (raw_x, raw_y, pixel) in imagebuf.enumerate_pixels_mut() {
        let (x, y) = frame.get_coord_for_pixel(raw_x, raw_y);
        let divergence = eq::mandelbrot_divergence(x, y);
        *pixel = gen_pixel(divergence);
    }
    return imagebuf;
}

pub fn write_png(prefix: &str, frame: Frame) -> Result<(), Box<error::Error>> {
    let ref mut fout = try!(fs::File::create(path::Path::new(&format!("{}_{}x{}_mandelbrot.png",
                                                                      prefix,
                                                                      frame.screen_width,
                                                                      frame.screen_height))));
    let imagebuf = gen_png(frame);
    try!(image::ImageRgba8(imagebuf).save(fout, image::PNG));
    return Ok(());
}

fn gen_pixel(divergence: f64) -> image::Rgba<u8> {
    let div_u8 = divergence as u8;
    if div_u8 == 0 {
        return image::Rgba([0, 0, 0, 255]);
    } else {
        return image::Rgba([0, 255 - divergence as u8, 255 - divergence as u8, 255]);
    }
}

pub struct Frame {
    screen_width: u32,
    screen_height: u32,
    min_x: f64,
    min_y: f64,
    plot_width: f64,
    plot_height: f64,
}

impl Frame {
    pub fn new(screen_width: u32,
               screen_height: u32,
               min_x: f64,
               max_x: f64,
               min_y: f64,
               max_y: f64)
               -> Frame {
        let plot_width = max_x - min_x;
        let plot_height = max_y - min_y;
        return Frame {
            screen_width: screen_width,
            screen_height: screen_height,
            min_x: min_x,
            min_y: min_y,
            plot_width: plot_width,
            plot_height: plot_height,
        };
    }

    fn get_coord_for_pixel(&self, screen_x: u32, screen_y: u32) -> (f64, f64) {
        let x_percent = f64::from(screen_x) / f64::from(self.screen_width);
        let y_percent = f64::from(screen_y) / f64::from(self.screen_height);

        let x = self.plot_width * x_percent + self.min_x;
        let y = self.plot_height * y_percent + self.min_y;
        return (x, y);
    }
}
