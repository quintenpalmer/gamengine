extern crate graphics;
extern crate fileformat;
extern crate types;

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
    let rect_sources = try!(fileformat::parse_rect_source(filename));


    let mut rects = std::vec::Vec::new();
    for rect_source in rect_sources.iter() {
        rects.push(graphics::Rect::new(rect_source.x,
                                       rect_source.y,
                                       rect_source.width,
                                       rect_source.height,
                                       rect_source.red,
                                       rect_source.green,
                                       rect_source.blue));
    }
    let mut app = try!(graphics::App::new(600,
                                          600,
                                          "Parallax Client Demo",
                                          graphics::SIMPLE_VERTEX_SOURCE,
                                          graphics::SIMPLE_FRAGMENT_SOURCE,
                                          rects));
    let mut iteration = 0;
    loop {
        for x in 0..rect_sources.len() {
            let ref s = rect_sources[x];

            let new_x = s.x_scale * operate(s.x_func, iteration + s.x_offset, s.x_cycle_size);
            let new_y = s.y_scale * operate(s.y_func, iteration + s.y_offset, s.y_cycle_size);

            try!(app.update_rect(x, new_x, new_y));
        }

        if app.window.handle_events() {
            break;
        }
        iteration += 1;

        try!(app.draw());
    }
    app.close();
    return Ok(());
}

pub fn operate(func: types::MathFunc, iteration: u16, u_cycle_size: u16) -> f32 {
    let cycle_size = f32::from(u_cycle_size);
    let tick = f32::from(iteration) % cycle_size;

    return func.plot_with_max(tick, cycle_size);
}
