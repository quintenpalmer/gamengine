extern crate graphics;

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
