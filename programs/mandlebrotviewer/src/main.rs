extern crate mandelbrot;

use std::error;
use std::env;
use std::path;

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

#[derive(Debug)]
struct FilepathParsingError {}

impl std::fmt::Display for FilepathParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unexpected error parsing the filepath")
    }
}

impl std::error::Error for FilepathParsingError {
    fn description(&self) -> &str {
        return "could not split filepath as expected";
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
                                 250);

}
