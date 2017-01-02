extern crate gl;
extern crate glutin;

use std::error;
use std::ptr;

use shader;
use shader_source;
use vertex;
use window;

pub struct App {
    pub window: window::Window,
    renderer: Renderer,
}

impl App {
    pub fn new<T: Into<String>>(width: u32,
                                height: u32,
                                title: T,
                                source: RenderingSource)
                                -> Result<App, Box<error::Error>> {

        let window = try!(window::Window::new(width, height, title));

        try!(window.make_main());

        let renderer = match source {
            RenderingSource::ColorRenderingSource => {
                Renderer::new(shader_source::color_pipeline_source())
            }
        };

        return Ok(App {
            window: window,
            renderer: renderer,
        });
    }

    pub fn draw<V: vertex::VertexSpecable + ?Sized>(&self,
                                                    rects: &Vec<Box<V>>)
                                                    -> Result<(), glutin::ContextError> {
        unsafe {
            // Clear the screen to red
            gl::ClearColor(0.9, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.renderer.draw(rects);

        try!(self.window.swap_buffers());
        return Ok(());
    }

    pub fn close(&self) {
        self.renderer.close();
    }
}

pub enum RenderingSource {
    ColorRenderingSource,
}

struct Renderer {
    program: shader::Program,
    vertices: vertex::VertexBuffers,
}

impl Renderer {
    fn new(p_src: shader_source::RenderingPipelineSource) -> Renderer {
        let vertex_shader = shader::Shader::new(p_src.vertex_glsl,
                                                shader_source::GLShaderEnum::VertexShader);
        let fragment_shader = shader::Shader::new(p_src.fragment_glsl,
                                                  shader_source::GLShaderEnum::FragmentShader);
        let program = shader::Program::new(vertex_shader, fragment_shader);

        let vertex_buffers = vertex::VertexBuffers::new(p_src.vertex_width);

        program.define_vertex_attribute_layout(&vertex_buffers, p_src.all_vertex_attrs);

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
