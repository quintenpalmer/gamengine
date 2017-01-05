extern crate graphics;

fn main() {
    println!("hello from a client");
    let rects = vec![graphics::Rect::new(0.0, 0.0, 0.2, 0.6),
                     graphics::Rect::new(0.0, 0.0, 0.6, 0.2)];
    let r_app = graphics::App::new(600,
                                   600,
                                   "Parallax Client Demo",
                                   graphics::SIMPLE_VERTEX_SOURCE,
                                   graphics::SIMPLE_FRAGMENT_SOURCE,
                                   rects);
    match r_app {
        Ok(app) => {
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
