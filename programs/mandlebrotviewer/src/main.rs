extern crate graphics;
extern crate mandelbrot;

use std::env;
use std::error;
use std::fmt;
use std::path;

#[derive(Debug)]
struct ArgError {}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "mandelbrotviewer command line error, must supplier one of app or img, and a frame \
                file to read if image")
    }
}

impl error::Error for ArgError {
    fn description(&self) -> &str {
        return "usage: mandelbrotviewer [app|img frames/default.csv]";
    }

    fn cause(&self) -> Option<&error::Error> {
        return None;
    }
}

#[derive(Debug)]
struct FilepathParsingError {}

impl fmt::Display for FilepathParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected error parsing the filepath")
    }
}

impl error::Error for FilepathParsingError {
    fn description(&self) -> &str {
        return "could not split filepath as expected";
    }

    fn cause(&self) -> Option<&error::Error> {
        return None;
    }
}

fn main() {
    println!("plotting mandlebrot");
    match run_main() {
        Ok(_) => println!("ran successfully"),
        Err(err) => println!("error: {} (reason: {})", err, err.description()),
    };
    println!("exiting");
}

fn run_main() -> Result<(), Box<error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(Box::new(ArgError {}));
    }
    match args[1].as_str() {
        "img" => {
            match args.len() {
                3 => gen_photo(args[2].as_str()),
                _ => Err(Box::new(ArgError {})),
            }
        }
        "app" => explore_mandelbrot(),
        _ => Err(Box::new(ArgError {})),
    }
}

fn gen_photo(filename: &str) -> Result<(), Box<error::Error>> {
    println!("reading: {}", filename);
    let frame = try!(mandelbrot::parse_frame(filename));

    let stem: &path::Path =
        try!(path::Path::new(filename).file_stem().ok_or(FilepathParsingError {})).as_ref();

    let prefix = path::Path::new("generated").join(stem).as_path().to_owned();

    let prefix_path = try!(prefix.to_str()
        .ok_or(FilepathParsingError {}));

    println!("writing with prefix: {}", prefix_path);

    return mandelbrot::write_png(prefix_path,
                                 mandelbrot::Frame::new(frame.screen_width,
                                                        frame.screen_height,
                                                        frame.x_min,
                                                        frame.x_max,
                                                        frame.y_min,
                                                        frame.y_max),
                                 frame.iterations);

}

fn build_mandelbrot_tex_def() -> graphics::TextureSetupDefinition {
    let width: u32 = 900;
    let height: u32 = 720;

    let frame = mandelbrot::Frame::new(width, height, -2.3, 1.2, -1.4, 1.4);

    let mut data: Vec<u8> = Vec::new();
    let iterations = 25;
    for raw_y in 0..height {
        for raw_x in 0..width {
            let pixel = mandelbrot::get_pixel_values(&frame, raw_x, raw_y, iterations);
            data.append(&mut pixel.to_vec());
        }
    }

    return graphics::TextureSetupDefinition {
        width: width,
        height: height,
        data: data,
    };

}

fn explore_mandelbrot() -> Result<(), Box<error::Error>> {
    let mandelbrot_tex_def = build_mandelbrot_tex_def();
    let mut app = try!(graphics::App::new(900,
                                          720,
                                          "Parallax Client Demo",
                                          graphics::RenderingSource::TextureRenderingSource {
                                              tex_def: mandelbrot_tex_def,
                                          }));
    let rects: Vec<Box<graphics::VertexSpecable>> =
        vec![Box::new(graphics::TexRect::new(-1.0, 1.0, -1.0, 1.0))];

    loop {
        match app.handle_events() {
            Some(graphics::Action::Closed) => break,
            Some(_) => (),
            None => (),
        }

        app.draw(&rects);
    }
    app.close();
    return Ok(());
}
