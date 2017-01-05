extern crate glutin;
extern crate gl;

pub struct Window {
    window: glutin::Window,
}

impl Window {
    pub fn new<T: Into<String>>(width: u32,
                                height: u32,
                                title: T)
                                -> Result<Window, glutin::CreationError> {

        let window = try!(glutin::WindowBuilder::new()
            .with_dimensions(width, height)
            .with_title(title)
            .with_vsync()
            .build());

        return Ok(Window { window: window });
    }

    pub fn make_main(&self) -> Result<(), glutin::ContextError> {
        // It is essential to make the context current before calling `gl::load_with`.
        unsafe { try!(self.window.make_current()) };

        // Load the OpenGL function pointers
        // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        return Ok(());
    }

    pub fn swap_buffers(&self) -> Result<(), glutin::ContextError> {
        return self.window.swap_buffers();
    }

    pub fn handle_events(&self) -> bool {
        for ev in self.window.poll_events() {
            match ev {
                glutin::Event::Closed => return true,
                glutin::Event::KeyboardInput(glutin::ElementState::Released,
                                             _,
                                             Some(glutin::VirtualKeyCode::Q)) => return true,
                _ => (),
            }
        }
        return false;
    }
}
