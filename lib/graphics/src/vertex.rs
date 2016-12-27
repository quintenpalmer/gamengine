extern crate gl;

use gl::types::*;

use std::vec;
use std::mem;

pub struct VertexBuffers {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    pub vertex_width: u8,
    pub rects: Vec<Box<VertexSpecable>>,
}

pub trait VertexSpecable {
    fn get_vertex_specification(&self) -> VertexSpecification;
    fn update_offset(&mut self, x: f32, y: f32);
}

pub struct VertexSpecification {
    pub vertices: Vec<GLfloat>,
    pub elements: Vec<GLint>,
}

impl VertexBuffers {
    pub fn new(rects: Vec<Box<VertexSpecable>>) -> VertexBuffers {
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

        let v = VertexBuffers {
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            rects: rects,
            vertex_width: 5,
        };

        v.gen_vertex_buffers();
        return v;
    }

    fn full_vertex_spec(&self) -> VertexSpecification {
        let mut vertices = vec::Vec::new();
        let mut elements = vec::Vec::new();
        let mut vertex_count_offset = 0;
        for rect in self.rects.iter() {
            let mut vert_spec = rect.get_vertex_specification();

            let vertex_count = (vert_spec.vertices.len() / (self.vertex_width as usize)) as i32;

            vertices.append(&mut vert_spec.vertices);
            elements.append(&mut vert_spec.elements.iter().map(|&x| x + vertex_count_offset).collect());

            vertex_count_offset += vertex_count;
        }

        return VertexSpecification {
            vertices: vertices,
            elements: elements,
        };
    }

    pub fn gen_vertex_buffers(&self) -> GLsizei {
        let vertex_spec = self.full_vertex_spec();
        let vertices = vertex_spec.vertices;
        let elements = vertex_spec.elements;

        let elem_count = elements.len() as GLsizei;

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

        return elem_count;
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
