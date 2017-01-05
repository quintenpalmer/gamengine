extern crate gl;

use gl::types::*;

use std::mem;

pub struct VertexData {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    pub x: f32,
    pub y: f32,
    width: f32,
    height: f32,
    pub vertex_width: u8,
}

impl VertexData {
    pub fn new(width: f32, height: f32, xloc: f32, yloc: f32) -> VertexData {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object
            gl::GenBuffers(1, &mut vbo);

            // Create a Element Buffer Object
            gl::GenBuffers(1, &mut ebo);
        }

        let mut v = VertexData {
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            x: xloc,
            y: yloc,
            width: width,
            height: height,
            vertex_width: 2,
        };

        v.gen_vertex_buffers();
        return v;
    }

    pub fn gen_vertex_buffers(&mut self) {
        let (top, bottom, left, right) = calc_corners(self.x, self.y, self.width, self.height);
        let vertices: Vec<GLfloat> = vertex_vector(top, bottom, left, right);

        let elements: Vec<GLint> = vec!(
            0, 1, 2,
            2, 3, 0,
        );

        unsafe {
            // copy the vertex data to the Vertex Buffer Object
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertices[0]),
                           gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (elements.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                           mem::transmute(&elements[0]),
                           gl::STATIC_DRAW);
        }
    }

    pub fn close(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vao);
            gl::DeleteVertexArrays(1, &self.vao);

            gl::DeleteBuffers(1, &self.vbo);

            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

fn calc_corners(width: f32, height: f32, x: f32, y: f32) -> (f32, f32, f32, f32) {
    let top = y + (height / 2.0);
    let bottom = y - (height / 2.0);
    let right = x + (width / 2.0);
    let left = x - (width / 2.0);
    return (top, bottom, left, right);
}

fn vertex_vector(top: f32, bottom: f32, left: f32, right: f32) -> Vec<GLfloat> {
    return vec!(
         left,    top, // top left
        right,    top, // top right
        right, bottom, // bottom right
         left, bottom  // bottom left
    );
}
