extern crate gl;

use gl::types::*;

use std::vec;
use std::mem;

pub struct VertexBuffers {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    pub vertex_width: u8,
    pub rects: Vec<Rect>,
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    orig_x: f32,
    orig_y: f32,
    width: f32,
    height: f32,
}

struct VertexSpecification {
    vertices: Vec<GLfloat>,
    elements: Vec<GLint>,
}

impl Rect {
    pub fn new(xloc: f32, yloc: f32, width: f32, height: f32) -> Rect {
        return Rect {
            x: xloc,
            y: yloc,
            orig_x: xloc,
            orig_y: yloc,
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

    pub fn update_offset(&mut self, x_offset: f32, y_offset: f32) {
        self.x = self.orig_x + x_offset;
        self.y = self.orig_y + y_offset;
    }

    fn get_vertex_specification(&self) -> VertexSpecification {
        let (top, bottom, left, right) = self.calc_corners();
        // top-left, top-right, bottom-left, bottom-right
        let vertices = vec![left, top, right, top, right, bottom, left, bottom];

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

        return VertexSpecification {
            vertices: vertices,
            elements: elements,
        };
    }
}

impl VertexBuffers {
    pub fn new(rects: Vec<Rect>) -> VertexBuffers {
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
            vertex_width: 2,
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

            let vertex_count = (vert_spec.vertices.len() / 2) as i32;

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
