extern crate glutin;
extern crate gl;

mod app;
mod window;
mod vertex;
mod shader;
mod shapes;

pub use app::App;
pub use shader::SIMPLE_VERTEX_SOURCE;
pub use shader::SIMPLE_FRAGMENT_SOURCE;
pub use shapes::SimpleRect;
pub use shapes::SimpleTriangle;
pub use vertex::VertexSpecable;
