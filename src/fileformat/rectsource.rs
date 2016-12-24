extern crate rustc_serialize;
extern crate csv;

extern crate types;

#[derive(RustcDecodable)]
pub struct RectSource {
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
}

pub fn parse_rect_source(filename: &str) -> Result<Vec<RectSource>, csv::Error> {
    let mut reader = try!(csv::Reader::from_file(filename));
    let mut rect_sources: Vec<RectSource> = Vec::new();
    for record in reader.decode() {
        let rect_source: RectSource = try!(record);
        rect_sources.push(rect_source);
    }
    return Ok(rect_sources);
}
