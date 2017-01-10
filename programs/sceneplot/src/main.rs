extern crate graphics;
extern crate sceneplotlib;

use sceneplotlib::fileformat;
use sceneplotlib::types;

#[derive(Debug)]
struct ArgError {}

impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parallax command line argument error")
    }
}

impl std::error::Error for ArgError {
    fn description(&self) -> &str {
        return "must supply the filename to read from";
    }

    fn cause(&self) -> Option<&std::error::Error> {
        return None;
    }
}

trait UpdateSpecable: graphics::VertexSpecable + graphics::Updateable {}
impl<T> UpdateSpecable for T where T: graphics::VertexSpecable + graphics::Updateable {}

fn main() {
    println!("hello from a client");
    match run_app() {
        Ok(_) => println!("ran successfully"),
        Err(err) => println!("error: {} (reason: {})", err, err.description()),
    };
    println!("exiting");
}

fn run_app() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let filename: &str = try!(match args.len() {
        2 => Ok(args[1].as_str()),
        _ => Err(Box::new(ArgError {})),
    });

    println!("reading: {}", filename);
    let shape_sources = try!(fileformat::parse_shape_source(filename));


    let mut rects: Vec<Box<UpdateSpecable>> = std::vec::Vec::new();
    for shape_source in shape_sources.iter() {
        match shape_source.shape {
            fileformat::ShapeType::Rect => {
                rects.push(Box::new(graphics::SimpleRect::new(shape_source.x,
                                                              shape_source.y,
                                                              shape_source.width,
                                                              shape_source.height,
                                                              shape_source.red,
                                                              shape_source.green,
                                                              shape_source.blue)))
            }
            fileformat::ShapeType::Triangle => {
                rects.push(Box::new(graphics::SimpleTriangle::new(shape_source.x,
                                                                  shape_source.y,
                                                                  shape_source.width,
                                                                  shape_source.height,
                                                                  shape_source.red,
                                                                  shape_source.green,
                                                                  shape_source.blue)))
            }
        }
    }

    let app = try!(graphics::App::new(600,
                                      600,
                                      "Parallax Client Demo",
                                      graphics::RenderingSource::ColorRenderingSource));
    let mut iteration = 0;
    loop {
        for i in 0..shape_sources.len() {
            let ref s = shape_sources[i];

            let new_x = s.x_scale * operate(s.x_func, iteration + s.x_offset, s.x_cycle_size);
            let new_y = s.y_scale * operate(s.y_func, iteration + s.y_offset, s.y_cycle_size);

            rects[i].update_offset(new_x, new_y);
        }

        match app.handle_events() {
            Some(graphics::Action::Closed) => break,
            Some(_) => (),
            None => (),
        }

        iteration += 1;

        try!(app.draw(&rects));
    }
    app.close();
    return Ok(());
}

pub fn operate(func: types::MathFunc, iteration: u16, u_cycle_size: u16) -> f32 {
    let cycle_size = f32::from(u_cycle_size);
    let tick = f32::from(iteration) % cycle_size;

    return func.plot_with_max(tick, cycle_size);
}
