use gl;

use std::error;
use std::ptr;

use program;
use shader_source;
use texture;
use vertex;
use window;

pub struct App {
    window: window::Window,
    renderer: Renderer,
}

impl App {
    pub fn new(width: u32,
               height: u32,
               title: &str,
               source: RenderingSource)
               -> Result<App, Box<error::Error>> {

        let mut window = try!(window::Window::new(width, height, title));

        window.make_main();

        let renderer = match source {
            RenderingSource::ColorRenderingSource => {
                Renderer::new(shader_source::color_pipeline_source())
            }
            RenderingSource::TextureRenderingSource { tex_def } => {
                let r = Renderer::new(shader_source::texture_pipeline_source());
                texture::texture_load(r.program.get_addr(), tex_def);
                r
            }
        };

        return Ok(App {
            window: window,
            renderer: renderer,
        });
    }

    pub fn draw<V: vertex::VertexSpecable + ?Sized>(&mut self, rects: &Vec<Box<V>>) {
        unsafe {
            // Clear the screen to red
            gl::ClearColor(0.9, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.renderer.draw(rects);

        self.window.swap_buffers();
    }

    pub fn handle_events(&mut self) -> Option<window::Action> {
        return self.window.handle_events();
    }

    pub fn close(&self) {
        self.renderer.close();
    }
}

pub enum RenderingSource {
    ColorRenderingSource,
    TextureRenderingSource { tex_def: texture::TextureSetupDefinition, },
}

struct Renderer {
    program: program::Program,
    vertices: vertex::VertexBuffers,
}

impl Renderer {
    fn new(p_src: shader_source::RenderingPipelineSource) -> Renderer {
        let vertex_buffers = vertex::VertexBuffers::new(p_src.vertex_width);

        let program = program::Program::new(p_src.vertex_glsl,
                                            p_src.fragment_glsl,
                                            p_src.all_vertex_attrs,
                                            &vertex_buffers);

        return Renderer {
            program: program,
            vertices: vertex_buffers,
        };
    }

    fn draw<V: vertex::VertexSpecable + ?Sized>(&self, rects: &Vec<Box<V>>) {
        // build and copy the vertex data
        let element_count = self.vertices.gen_vertex_buffers(rects);
        unsafe {
            gl::DrawElements(gl::TRIANGLES, element_count, gl::UNSIGNED_INT, ptr::null());
        }
    }

    pub fn close(&self) {
        self.vertices.close();
        self.program.close();
    }
}
