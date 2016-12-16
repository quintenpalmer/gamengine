extern crate glutin;

use window;

pub struct App {
    pub window: window::Window,
}

impl App {
    pub fn new<T: Into<String>>(
            width: u32,
            height: u32,
            title: T) -> Result<App, glutin::CreationError> {

        let window = try!(window::Window::new(width, height, title));

        return Ok(App{
            window: window,
        });
    }
}
