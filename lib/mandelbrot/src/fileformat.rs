extern crate rustc_serialize;
extern crate csv;

use std::fmt;
use std::error;

#[derive(Debug)]
struct MultiFrameError {}

impl fmt::Display for MultiFrameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "error that signifies multiple frames were present in file parsing")
    }
}

impl error::Error for MultiFrameError {
    fn description(&self) -> &str {
        return "must supply only one frame in file";
    }

    fn cause(&self) -> Option<&error::Error> {
        return None;
    }
}

#[derive(RustcDecodable, Copy, Clone)]
pub struct Frame {
    pub screen_width: u32,
    pub screen_height: u32,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

pub fn parse_frame(filename: &str) -> Result<Frame, Box<error::Error>> {
    let mut reader = try!(csv::Reader::from_file(filename));
    let mut frames: Vec<Frame> = Vec::new();
    for record in reader.decode() {
        let shape_source: Frame = try!(record);
        frames.push(shape_source);
    }
    return match frames.len() {
        1 => Ok(frames[0]),
        _ => Err(Box::new(MultiFrameError {})),
    };
}
