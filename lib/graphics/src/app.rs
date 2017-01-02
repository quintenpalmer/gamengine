extern crate glutin;
extern crate gl;

use std::ptr;
use std::error;

use window;
use shader;
use vertex;

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
                Renderer::new(shader::SIMPLE_VERTEX_SOURCE,
                              shader::SIMPLE_FRAGMENT_SOURCE,
                              5 /* this is the width of a ColorVertex: x, y, red, green, blue */)
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
    fn new(vertex_source: shader::ShaderSource,
           fragment_source: shader::ShaderSource,
           vertex_width: u8)
           -> Renderer {


        let vertex_shader = shader::Shader::new(vertex_source, shader::GLShaderEnum::VertexShader);
        let fragment_shader = shader::Shader::new(fragment_source,
                                                  shader::GLShaderEnum::FragmentShader);
        let program = shader::Program::new(vertex_shader, fragment_shader);

        let vertex_data = vertex::VertexBuffers::new(vertex_width);

        program.link_vertex(&vertex_data);

        return Renderer {
            program: program,
            vertices: vertex_data,
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
