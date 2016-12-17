extern crate gl;

use gl::types::*;

use std::mem;

pub struct VertexBuffers {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    rect: Rect,
    pub vertex_width: u8,
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn new(width: f32, height: f32, xloc: f32, yloc: f32) -> Rect {
        return Rect {
            x: xloc,
            y: yloc,
            width: width,
            height: height,
        };
    }

    fn calc_corners(&self) -> (f32, f32, f32, f32) {
        let top = self.y + (self.height / 2.0);
        let bottom = self.y - (self.height / 2.0);
        let right = self.x + (self.width / 2.0);
        let left = self.x - (self.width / 2.0);
        return (top, bottom, left, right);
    }

    fn vertex_vector(&self) -> Vec<GLfloat> {
        let (top, bottom, left, right) = self.calc_corners();
        // top-left, top-right, bottom-left, bottom-right
        return vec![left, top, right, top, right, bottom, left, bottom];
    }
}

impl VertexBuffers {
    pub fn new(rect: Rect) -> VertexBuffers {
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

        let mut v = VertexBuffers {
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            rect: rect,
            vertex_width: 2,
        };

        v.gen_vertex_buffers();
        return v;
    }

    pub fn gen_vertex_buffers(&mut self) {
        let vertices: Vec<GLfloat> = self.rect.vertex_vector();

        // the elements each point to what 3 points make up a single triangle
        // given the elements below and the vertex data, we see the triangles
        // are as follows:
        //
        // triangle one | triangle two
        //  o--o        |    o
        //  | /         |   /|
        //  |/          |  / |
        //  o           | o--o
        let elements: Vec<GLint> = vec![0, 1, 2, 2, 3, 0];

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
