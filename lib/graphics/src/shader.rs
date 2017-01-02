extern crate gl;

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use gl::types::*;

use shader_source;
use vertex;

pub struct Program {
    addr: GLuint,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

impl Program {
    pub fn new(vs: Shader, fs: Shader) -> Program {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs.addr);
            gl::AttachShader(program, fs.addr);
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
            return Program {
                addr: program,
                vertex_shader: vs,
                fragment_shader: fs,
            };
        }
    }

    pub fn define_vertex_attribute_layout(&self,
                                          vbs: &vertex::VertexBuffers,
                                          vertex_attrs: Vec<shader_source::VertexAttribute>) {
        unsafe {
            gl::UseProgram(self.addr);
            gl::BindFragDataLocation(self.addr, 0, CString::new("out_color").unwrap().as_ptr());
        }
        for vertex_attr in vertex_attrs.iter() {

            self.define_single_vertex_attribute(vbs, vertex_attr);
        }
    }

    pub fn define_single_vertex_attribute(&self,
                                          vbs: &vertex::VertexBuffers,
                                          vertex_attr: &shader_source::VertexAttribute) {
        unsafe {
            // Specify the layout of the vertex data
            let attr = gl::GetAttribLocation(self.addr,
                                             CString::new(vertex_attr.var_name).unwrap().as_ptr());
            gl::EnableVertexAttribArray(attr as GLuint);
            gl::VertexAttribPointer(attr as GLuint, vertex_attr.stride, gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    ((vbs.vertex_width as GLsizei) * (mem::size_of::<GLfloat>() as GLsizei)) as i32,
                                    (vertex_attr.offset * mem::size_of::<GLfloat>()) as *const _);
        }
    }

    pub fn close(&self) {
        self.vertex_shader.close();
        self.fragment_shader.close();
        unsafe {
            gl::DeleteProgram(self.addr);
        }
    }
}

pub struct Shader {
    addr: GLuint,
}

impl Shader {
    pub fn new(src: &'static str, shader_ty: shader_source::GLShaderEnum) -> Shader {
        let ty = shader_ty.to_glenum();
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);
            // Attempt to compile the shader
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(shader,
                                     len,
                                     ptr::null_mut(),
                                     buf.as_mut_ptr() as *mut GLchar);
                // TODO don't panic here and return an error that we handle
                panic!("{}",
                       str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
            }
        }
        return Shader { addr: shader };
    }

    pub fn close(&self) {
        unsafe {
            gl::DeleteShader(self.addr);
        }
    }
}
