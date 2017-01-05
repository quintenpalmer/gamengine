extern crate glutin;
extern crate gl;

mod app;
mod window;
mod vertex;
mod shader;

pub use app::App;
pub use shader::SIMPLE_VERTEX_SOURCE;
pub use shader::SIMPLE_FRAGMENT_SOURCE;
