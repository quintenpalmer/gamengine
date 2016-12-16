extern crate glutin;

pub struct Window {
    window: glutin::Window,
}

impl Window {
    pub fn new<T: Into<String>>(
            width: u32,
            height: u32,
            title: T) -> Result<Window, glutin::CreationError> {

        let window = try!(glutin::WindowBuilder::new()
            .with_dimensions(width, height)
            .with_title(title)
            .with_vsync()
            .build());

        return Ok(Window{
            window: window,
        });
    }

    pub fn handle_events(&self) -> bool {
        for ev in self.window.poll_events() {
            match ev {
                glutin::Event::Closed => {
                    return true
                },
                glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Q)) => {
                    return true
                },
                _ => (),
            }
        };
        return false
    }
}
