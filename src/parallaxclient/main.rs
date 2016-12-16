extern crate graphics;

fn main() {
    println!("hello from a client");
    let r_app = graphics::App::new(600, 600, "Parallax Client Demo");
    match r_app {
        Ok(app) => {
            loop {
                if app.window.handle_events() {
                    break;
                }
            }
        },
        Err(err) => {
            println!("could not create window: {}", err);
        },
    }
    println!("exiting");
}
