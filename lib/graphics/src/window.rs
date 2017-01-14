use gl;
use glfw;

use glfw::Context;

use std::sync::mpsc;

use gerror;

pub struct Window {
    inner_glfw: glfw::Glfw,
    window: glfw::Window,
    event_rec: mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

pub enum Action {
    Resized(u32, u32),
    Closed,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Window, gerror::Error> {

        let mut inner_glfw = try!(glfw::init(glfw::FAIL_ON_ERRORS).map_err(gerror::new_init_error));

        inner_glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
        inner_glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        inner_glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (window, events) =
            try!(inner_glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
                .ok_or(gerror::Error::WindowCreationError(gerror::GLFWError::GLFWFromOption)));

        return Ok(Window {
            inner_glfw: inner_glfw,
            window: window,
            event_rec: events,
        });
    }

    pub fn make_main(&mut self) {
        // It is essential to make the context current before calling `gl::load_with`.
        self.window.make_current();

        self.inner_glfw.set_swap_interval(glfw::SwapInterval::Adaptive);

        // register this window as the callback for key polling
        self.window.set_all_polling(true);

        // Load the OpenGL function pointers
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn handle_events(&mut self) -> Option<Action> {
        self.inner_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_rec) {
            println!("an event");
            match event {
                glfw::WindowEvent::Close => return Some(Action::Closed),
                glfw::WindowEvent::Key(glfw::Key::Q, _, glfw::Action::Release, _) => {
                    return Some(Action::Closed)
                }
                glfw::WindowEvent::Size(w, h) => {
                    println!("I'm resizing");
                    return Some(Action::Resized(w as u32, h as u32));
                }
                _ => (),
            }
        }
        return None;
    }
}
