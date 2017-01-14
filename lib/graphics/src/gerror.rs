use glfw;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    WindowCreationError(GLFWError),
}

pub fn new_init_error(e: glfw::InitError) -> Error {
    Error::WindowCreationError(GLFWError::GLFWInitError(e))
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "graphics error")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::WindowCreationError(_) => "window creation error",
        }
    }
}

#[derive(Debug)]
pub enum GLFWError {
    GLFWInitError(glfw::InitError),
    GLFWFromOption,
}
