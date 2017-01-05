extern crate glutin;
extern crate gl;

mod app;
mod window;
mod vertex;
mod shader;
mod shader_source;
mod shapes;

pub use app::App;
pub use app::RenderingSource;
pub use shapes::SimpleRect;
pub use shapes::SimpleTriangle;
pub use shapes::Updateable;
pub use vertex::VertexSpecable;
pub use vertex::VertexSpecification;
pub use vertex::Vertex;
pub use vertex::ElementTriangle;
