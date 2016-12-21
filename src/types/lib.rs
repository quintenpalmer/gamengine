extern crate rustc_serialize;

use rustc_serialize::Decodable;
use rustc_serialize::Decoder;

#[derive(Clone, Copy)]
pub enum MathFunc {
    Linear,
}

impl Decodable for MathFunc {
    fn decode<D: Decoder>(d: &mut D) -> Result<MathFunc, D::Error> {
        let s = try!(d.read_str());
        match s.as_str() {
            "lin" => Ok(MathFunc::Linear),
            _ => Err(d.error("invalid path")),
        }
    }
}

impl MathFunc {
    pub fn operate(&self, x: f32) -> f32 {
        match self {
            &MathFunc::Linear => x,
        }
    }
}
