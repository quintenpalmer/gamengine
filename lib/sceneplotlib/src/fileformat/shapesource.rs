extern crate rustc_serialize;
extern crate csv;

use types;

use rustc_serialize::Decodable;
use rustc_serialize::Decoder;

pub enum ShapeType {
    Rect,
    Triangle,
}

impl Decodable for ShapeType {
    fn decode<D: Decoder>(d: &mut D) -> Result<ShapeType, D::Error> {
        let s = try!(d.read_str());
        match s.as_str() {
            "rect" => Ok(ShapeType::Rect),
            "tri" => Ok(ShapeType::Triangle),
            _ => Err(d.error("invalid math function")),
        }
    }
}

#[derive(RustcDecodable)]
pub struct ShapeSource {
    pub shape: ShapeType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub x_func: types::MathFunc,
    pub y_func: types::MathFunc,
    pub x_scale: f32,
    pub y_scale: f32,
    pub x_cycle_size: u16,
    pub y_cycle_size: u16,
    pub x_offset: u16,
    pub y_offset: u16,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn parse_shape_source(filename: &str) -> Result<Vec<ShapeSource>, csv::Error> {
    let mut reader = try!(csv::Reader::from_file(filename));
    let mut shape_sources: Vec<ShapeSource> = Vec::new();
    for record in reader.decode() {
        let shape_source: ShapeSource = try!(record);
        shape_sources.push(shape_source);
    }
    return Ok(shape_sources);
}
