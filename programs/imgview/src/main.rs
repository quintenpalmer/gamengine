extern crate image;
extern crate num;

extern crate graphics;

use image::ImageDecoder;

use std::fmt;
use std::env;
use std::error;
use std::fs::File;

fn main() {
    println!("editing an image");
    match run_app() {
        Ok(()) => println!("ran successfully"),
        Err(err) => println!("error: {} (reason: {})", err, err.description()),
    }
    println!("done editing");
}


fn run_app() -> Result<(), Box<error::Error>> {

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(Box::new(Error::ArgError {}));
    }
    let filename = args[1].as_str();

    let f = try!(File::open(filename));
    let mut decoder = image::png::PNGDecoder::new(f);
    let (width, height) = try!(decoder.dimensions());
    let im = try!(decoder.read_image());

    let u8image = try!(match im {
        image::DecodingResult::U8(u8im) => Ok(u8im),
        image::DecodingResult::U16(_) => Err(Error::U8OnlyError {}),
    });

    let tex_def = graphics::TextureSetupDefinition {
        width: width,
        height: height,
        data: u8image,
    };

    let mut screen_width = 600;
    let mut screen_height = 600;

    let mut app = try!(graphics::App::new(screen_width,
                                          screen_height,
                                          "Picture Viewer",
                                          graphics::RenderingSource::TextureRenderingSource {
                                              tex_def: tex_def,
                                          }));

    loop {
        match app.handle_events() {
            Some(graphics::Action::Closed) => break,
            Some(graphics::Action::Resized(w, h)) => {
                screen_width = w;
                screen_height = h;
            }
            None => (),
        }

        let (lower_x, upper_x, lower_y, upper_y) =
            calc_position(width, height, screen_width, screen_height);
        let rects: Vec<Box<graphics::VertexSpecable>> =
            vec![Box::new(graphics::TexRect::new(lower_x, upper_x, lower_y, upper_y))];

        app.draw(&rects);
    }
    app.close();
    return Ok(());
}

fn calc_position(image_w: u32, image_h: u32, screen_w: u32, screen_h: u32) -> (f32, f32, f32, f32) {
    let f_image_h = f32::from(image_h as u16);
    let f_image_w = f32::from(image_w as u16);
    let f_screen_h = f32::from(screen_h as u16);
    let f_screen_w = f32::from(screen_w as u16);

    let image_ratio = f_image_w / f_image_h;
    let screen_ratio = f_screen_w / f_screen_h;

    if image_ratio > screen_ratio {
        let half = (f_screen_h - ((f_screen_w / f_image_w) * f_image_h)) / (f_screen_h);
        return (-1.0, 1.0, -1.0 + half, 1.0 - half);
    } else {
        let half = (f_screen_w - ((f_screen_h / f_image_h) * f_image_w)) / (f_screen_w);
        return (-1.0 + half, 1.0 - half, -1.0, 1.0);
    }
}


#[derive(Debug)]
enum Error {
    U8OnlyError,
    ArgError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::U8OnlyError => write!(f, "can only accept u8 for the image"),
            Error::ArgError => write!(f, "must supply png to load"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::U8OnlyError => "can only accept u8 for the image",
            Error::ArgError => "must supply png to load",
        }
    }
}
