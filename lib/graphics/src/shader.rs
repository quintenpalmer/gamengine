extern crate gl;

use std::ffi::CString;
use std::ptr;
use std::str;

use gl::types::*;

use shader_source;

pub struct Shader {
    addr: GLuint,
}

impl Shader {
    pub fn new<T: shader_source::GLShader>(gl_shader: T) -> Shader {
        let ty = gl_shader.to_glenum();
        let src = gl_shader.get_glsl();
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

    pub fn get_addr(&self) -> GLuint {
        return self.addr;
    }

    pub fn close(&self) {
        unsafe {
            gl::DeleteShader(self.addr);
        }
    }
}
