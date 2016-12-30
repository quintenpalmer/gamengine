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

fn gen_pixel(divergence: f32) -> image::Rgba<u8> {
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
    min_x: f32,
    min_y: f32,
    plot_width: f32,
    plot_height: f32,
}

impl Frame {
    pub fn new(screen_width: u32,
               screen_height: u32,
               min_x: f32,
               max_x: f32,
               min_y: f32,
               max_y: f32)
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

    fn get_coord_for_pixel(&self, screen_x: u32, screen_y: u32) -> (f32, f32) {
        let x_percent = f32::from(screen_x as u16) / f32::from(self.screen_width as u16);
        let y_percent = f32::from(screen_y as u16) / f32::from(self.screen_height as u16);

        let x = self.plot_width * x_percent + self.min_x;
        let y = self.plot_height * y_percent + self.min_y;
        return (x, y);
    }
}
