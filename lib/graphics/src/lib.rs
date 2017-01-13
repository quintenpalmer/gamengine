extern crate gl;
extern crate glutin;

mod app;
mod program;
mod shader;
mod shader_source;
mod color_shapes;
mod texture;
mod texture_shapes;
mod vertex;
mod window;

pub use app::App;
pub use app::RenderingSource;
pub use color_shapes::SimpleRect;
pub use color_shapes::SimpleTriangle;
pub use color_shapes::Updateable;
pub use texture::TextureSetupDefinition;
pub use texture_shapes::TexRect;
pub use vertex::VertexSpecable;
pub use vertex::VertexSpecification;
pub use vertex::Vertex;
pub use vertex::ColorVertex;
pub use vertex::TextureVertex;
pub use vertex::ElementTriangle;
