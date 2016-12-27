extern crate gl;

use gl::types::*;

use std::vec;
use std::mem;

pub struct VertexBuffers {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    pub vertex_width: u8,
}

pub trait VertexSpecable {
    fn get_vertex_specification(&self) -> VertexSpecification;
}

pub struct Vertex {
    pub x: GLfloat,
    pub y: GLfloat,
    pub red: GLfloat,
    pub green: GLfloat,
    pub blue: GLfloat,
}

impl Vertex {
    fn get_vec(&self) -> Vec<GLfloat> {
        return vec![self.x, self.y, self.red, self.green, self.blue];
    }
}

pub struct ElementTriangle {
    pub p1: GLint,
    pub p2: GLint,
    pub p3: GLint,
}

impl ElementTriangle {
    fn get_vec(&self) -> Vec<GLint> {
        return vec![self.p1, self.p2, self.p3];
    }

    fn add_vertex_offset(&self, vertex_offset: i32) -> ElementTriangle {
        return ElementTriangle {
            p1: self.p1 + vertex_offset,
            p2: self.p2 + vertex_offset,
            p3: self.p3 + vertex_offset,
        };
    }
}

pub struct VertexSpecification {
    pub vertices: Vec<Vertex>,
    pub elements: Vec<ElementTriangle>,
}

impl VertexBuffers {
    pub fn new<V: VertexSpecable + ?Sized>(rects: &Vec<Box<V>>) -> VertexBuffers {
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
            vertex_width: 5, /* this is the width of the current definition of a Vertex, the struct above */
        };

        v.gen_vertex_buffers(rects);
        return v;
    }

    pub fn gen_vertex_buffers<V: VertexSpecable + ?Sized>(&self, rects: &Vec<Box<V>>) -> GLsizei {
        let vertex_spec = full_vertex_spec(rects);
        let vertex_structs = vertex_spec.vertices;
        let element_triangles = vertex_spec.elements;

        let mut vertices: vec::Vec<GLfloat> = vec::Vec::new();
        for vertex in vertex_structs.iter() {
            vertices.append(&mut vertex.get_vec());
        }
        let mut elements: vec::Vec<GLint> = vec::Vec::new();
        for element_triangle in element_triangles.iter() {
            elements.append(&mut element_triangle.get_vec());
        }

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

fn full_vertex_spec<V: VertexSpecable + ?Sized>(rects: &Vec<Box<V>>) -> VertexSpecification {
    let mut vertices = vec::Vec::new();
    let mut elements = vec::Vec::new();
    let mut vertex_count_offset = 0;
    for rect in rects.iter() {
        let mut vert_spec = rect.get_vertex_specification();

        let vertex_count = vert_spec.vertices.len() as i32;

        vertices.append(&mut vert_spec.vertices);
        elements.append(&mut vert_spec.elements
            .iter()
            .map(|x| x.add_vertex_offset(vertex_count_offset))
            .collect());

        vertex_count_offset += vertex_count;
    }

    return VertexSpecification {
        vertices: vertices,
        elements: elements,
    };
}
