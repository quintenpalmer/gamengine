extern crate graphics;
extern crate fileformat;

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
                                       rect_source.height));
    }
    let app = try!(graphics::App::new(600,
                                      600,
                                      "Parallax Client Demo",
                                      graphics::SIMPLE_VERTEX_SOURCE,
                                      graphics::SIMPLE_FRAGMENT_SOURCE,
                                      rects));
    loop {
        try!(app.draw());
        if app.window.handle_events() {
            break;
        }
    }
    app.close();
    return Ok(());
}
