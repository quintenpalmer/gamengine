extern crate graphics;

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
    let rects = vec![graphics::Rect::new(0.0, 0.0, 0.2, 0.6),
                     graphics::Rect::new(0.0, 0.0, 0.6, 0.2)];

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
