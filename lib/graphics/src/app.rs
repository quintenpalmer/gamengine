extern crate glutin;
extern crate gl;

use std::ptr;
use std::error;

use window;
use shader;
use vertex;

pub struct App {
    pub window: window::Window,
    program: shader::Program,
    vertices: vertex::VertexBuffers,
}

impl App {
    pub fn new<T: Into<String>>(width: u32,
                                height: u32,
                                title: T,
                                vertex_source: shader::ShaderSource,
                                fragment_source: shader::ShaderSource,
                                rects: &Vec<Box<vertex::VertexSpecable>>)
                                -> Result<App, Box<error::Error>> {

        let window = try!(window::Window::new(width, height, title));

        try!(window.make_main());

        let vertex_shader = shader::Shader::new(vertex_source, shader::GLShaderEnum::VertexShader);
        let fragment_shader = shader::Shader::new(fragment_source,
                                                  shader::GLShaderEnum::FragmentShader);
        let program = shader::Program::new(vertex_shader, fragment_shader);

        let vertex_data = vertex::VertexBuffers::new(rects);

        program.link_vertex(&vertex_data);

        return Ok(App {
            window: window,
            program: program,
            vertices: vertex_data,
        });
    }

    pub fn draw(&self,
                rects: &Vec<Box<vertex::VertexSpecable>>)
                -> Result<(), glutin::ContextError> {
        // build and copy the vertex data
        let element_count = self.vertices.gen_vertex_buffers(rects);
        unsafe {
            // Clear the screen to red
            gl::ClearColor(0.9, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(gl::TRIANGLES, element_count, gl::UNSIGNED_INT, ptr::null());

        }

        try!(self.window.swap_buffers());
        return Ok(());
    }

    pub fn close(&self) {
        self.vertices.close();
        self.program.close();
    }
}