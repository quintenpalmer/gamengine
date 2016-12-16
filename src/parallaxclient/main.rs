extern crate graphics;

fn main() {
    println!("hello from a client");
    let r_app = graphics::App::new(600,
                                   600,
                                   "Parallax Client Demo",
                                   graphics::SIMPLE_VERTEX_SOURCE,
                                   graphics::SIMPLE_FRAGMENT_SOURCE,
                                   0.0,
                                   0.0,
                                   0.25,
                                   0.25);
    match r_app {
        Ok(mut app) => {
            loop {
                match app.draw() {
                    Ok(_) => {}
                    Err(err) => {
                        println!("error: {}", err);
                        break;
                    }
                };
                if app.window.handle_events() {
                    break;
                }
            }
            app.close();
        }
        Err(err) => {
            println!("could not create window: {}", err);
        }
    }
    println!("exiting");
}
