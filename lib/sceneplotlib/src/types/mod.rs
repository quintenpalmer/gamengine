use rustc_serialize::Decodable;
use rustc_serialize::Decoder;

use std::f32;
use std::fmt;
use std::error;

#[derive(Clone, Copy)]
pub enum MathFunc {
    Linear,
    Parabolic,
    Sin,
}

impl Decodable for MathFunc {
    fn decode<D: Decoder>(d: &mut D) -> Result<MathFunc, D::Error> {
        let s = try!(d.read_str());
        match s.as_str() {
            "lin" => Ok(MathFunc::Linear),
            "par" => Ok(MathFunc::Parabolic),
            "sin" => Ok(MathFunc::Sin),
            _ => Err(d.error("invalid math function")),
        }
    }
}

impl MathFunc {
    pub fn operate(&self, x: f32) -> f32 {
        match self {
            &MathFunc::Linear => x,
            &MathFunc::Parabolic => x.powi(2),
            &MathFunc::Sin => x.sin(),
        }
    }

    pub fn plot_with_max(&self, x: f32, max: f32) -> f32 {
        match self {
            &MathFunc::Linear => (x - max / 2.0) / max,
            // y = -(x - n/2)^2 + (n/2)^2
            &MathFunc::Parabolic => {
                let half_max = max / 2.0;
                let non_minimized = -((x - half_max).powi(2)) + half_max.powi(2);
                non_minimized / half_max.powi(2)
            }
            &MathFunc::Sin => ((x / max) * f32::consts::PI * 2.0).sin() / 2.5,
        }
    }
}

#[derive(Debug)]
pub struct OutOfBoundsError {}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "out of bounds error")
    }
}

impl error::Error for OutOfBoundsError {
    fn description(&self) -> &str {
        return "request to index outside bounds of vector";
    }

    fn cause(&self) -> Option<&error::Error> {
        return None;
    }
}
