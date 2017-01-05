extern crate num;

use num::rational::Ratio;

pub struct SimpleColor {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn yellow_to_blue(value: u8) -> [u8; 4] {
    return pixel_color(SimpleColor {
                           red: 255,
                           green: 255,
                           blue: 0,
                       },
                       SimpleColor {
                           red: 0,
                           green: 0,
                           blue: 255,
                       },
                       value);
}

pub fn pixel_color(start: SimpleColor, end: SimpleColor, value: u8) -> [u8; 4] {
    return [lin_interp(start.red, end.red, value),
            lin_interp(start.green, end.green, value),
            lin_interp(start.blue, end.blue, value),
            255];
}

pub fn lin_interp(start: u8, end: u8, val: u8) -> u8 {
    let start_weight = Ratio::new(255 - (val as u32), 255) * Ratio::from_integer(start as u32);
    let end_weight = Ratio::new(val as u32, 255) * Ratio::from_integer(end as u32);
    return (start_weight + end_weight).to_integer() as u8;
}
