use gl;

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use gl::types::*;

use shader;
use shader_source;
use vertex;

pub struct Program {
    addr: GLuint,
    vertex_shader: shader::Shader,
    fragment_shader: shader::Shader,
}

impl Program {
    pub fn new(vertex_glsl: shader_source::GLVertexShader,
               fragment_glsl: shader_source::GLFragmentShader,
               all_vertex_attrs: Vec<shader_source::VertexAttribute>,
               vbs: &vertex::VertexBuffers)
               -> Program {
        unsafe {
            let program = gl::CreateProgram();
            let vs = shader::Shader::new(vertex_glsl);
            let fs = shader::Shader::new(fragment_glsl);
            gl::AttachShader(program, vs.get_addr());
            gl::AttachShader(program, fs.get_addr());
            gl::LinkProgram(program);
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(program,
                                      len,
                                      ptr::null_mut(),
                                      buf.as_mut_ptr() as *mut GLchar);
                // TODO don't panic here and return an error that we handle
                panic!("{}",
                       str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
            }
            let p = Program {
                addr: program,
                vertex_shader: vs,
                fragment_shader: fs,
            };
            p.define_vertex_attribute_layout(vbs, all_vertex_attrs);
            return p;
        }
    }

    fn define_vertex_attribute_layout(&self,
                                      vbs: &vertex::VertexBuffers,
                                      vertex_attrs: Vec<shader_source::VertexAttribute>) {
        unsafe {
            gl::UseProgram(self.addr);
            gl::BindFragDataLocation(self.addr, 0, CString::new("out_color").unwrap().as_ptr());
        }
        let mut total_offset = 0;
        for vertex_attr in vertex_attrs.iter() {
            let offset = self.define_single_vertex_attribute(vbs, vertex_attr, total_offset);
            total_offset += offset;
        }
    }

    fn define_single_vertex_attribute(&self,
                                      vbs: &vertex::VertexBuffers,
                                      vertex_attr: &shader_source::VertexAttribute,
                                      offset: usize)
                                      -> usize {
        unsafe {
            // Specify the layout of the vertex data
            let attr = gl::GetAttribLocation(self.addr,
                                             CString::new(vertex_attr.var_name).unwrap().as_ptr());
            gl::EnableVertexAttribArray(attr as GLuint);
            gl::VertexAttribPointer(attr as GLuint, vertex_attr.stride, gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    ((vbs.vertex_width as GLsizei) * (mem::size_of::<GLfloat>() as GLsizei)) as i32,
                                    (offset * mem::size_of::<GLfloat>()) as *const _);
        }
        return vertex_attr.stride as usize;
    }

    pub fn get_addr(&self) -> GLuint {
        return self.addr;
    }

    pub fn close(&self) {
        self.vertex_shader.close();
        self.fragment_shader.close();
        unsafe {
            gl::DeleteProgram(self.addr);
        }
    }
}
