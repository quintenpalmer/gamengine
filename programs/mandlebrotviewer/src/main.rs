extern crate mandelbrot;

use std::error;
use std::env;

#[derive(Debug)]
struct ArgError {}

impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "mandelbrotviewer requires a filename to read frame from")
    }
}

impl std::error::Error for ArgError {
    fn description(&self) -> &str {
        return "must supply the filename to read the frame from";
    }

    fn cause(&self) -> Option<&std::error::Error> {
        return None;
    }
}

fn main() {
    println!("plotting mandlebrot");
    match gen_photo() {
        Ok(_) => println!("ran successfully"),
        Err(err) => println!("error: {} (reason: {})", err, err.description()),
    };
    println!("exiting");
}

fn gen_photo() -> Result<(), Box<error::Error>> {
    let args: Vec<String> = env::args().collect();
    let filename: &str = try!(match args.len() {
        2 => Ok(args[1].as_str()),
        _ => Err(Box::new(ArgError {})),
    });

    println!("reading: {}", filename);
    let frame = try!(mandelbrot::parse_frame(filename));
    return mandelbrot::write_png(mandelbrot::Frame::new(frame.screen_width,
                                                        frame.screen_height,
                                                        frame.x_min,
                                                        frame.x_max,
                                                        frame.y_min,
                                                        frame.y_max));

}
