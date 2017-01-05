extern crate gl;
extern crate glutin;

mod app;
mod program;
mod shader;
mod shader_source;
mod shapes;
mod texture;
mod vertex;
mod window;

pub use app::App;
pub use app::RenderingSource;
pub use shapes::SimpleRect;
pub use shapes::SimpleTriangle;
pub use shapes::Updateable;
pub use texture::TextureSetupDefinition;
pub use vertex::VertexSpecable;
pub use vertex::VertexSpecification;
pub use vertex::Vertex;
pub use vertex::ColorVertex;
pub use vertex::TextureVertex;
pub use vertex::ElementTriangle;
