extern crate gl;

use std::ptr;
use std::str;
use std::mem;
use std::ffi::CString;

use gl::types::*;

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

    pub fn link_vertex(&self, vd: &vertex::VertexBuffers) {
        unsafe {
            gl::UseProgram(self.addr);
        }
        self.vertex_shader.link_to_program(self, vd);
    }

    pub fn close(&self) {
        self.vertex_shader.close();
        self.fragment_shader.close();
        unsafe {
            gl::DeleteProgram(self.addr);
        }
    }
}

pub enum GLShaderEnum {
    VertexShader,
    FragmentShader,
}

impl GLShaderEnum {
    fn to_glenum(&self) -> GLenum {
        match self {
            &GLShaderEnum::VertexShader => gl::VERTEX_SHADER,
            &GLShaderEnum::FragmentShader => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct Shader {
    addr: GLuint,
    var_name: &'static str,
}

impl Shader {
    pub fn new(shader_source: ShaderSource, shader_ty: GLShaderEnum) -> Shader {
        let src = shader_source.source_glsl;
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
        return Shader {
            addr: shader,
            var_name: shader_source.var_name,
        };
    }


    pub fn link_to_program(&self, program: &Program, vd: &vertex::VertexBuffers) {
        unsafe {
            // Specify the layout of the vertex data
            let attr = gl::GetAttribLocation(program.addr,
                                             CString::new(self.var_name).unwrap().as_ptr());
            gl::EnableVertexAttribArray(attr as GLuint);
            gl::VertexAttribPointer(attr as GLuint, 2, gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    ((vd.vertex_width as GLsizei) * (mem::size_of::<GLfloat>() as GLsizei)) as i32,
                                    ptr::null());
        }
    }

    pub fn close(&self) {
        unsafe {
            gl::DeleteShader(self.addr);
        }
    }
}

pub struct ShaderSource {
    pub source_glsl: &'static str,
    pub var_name: &'static str,
}

const VS_SRC: &'static str = r#"#version 150
    in vec2 position;
    void main() {
       gl_Position = vec4(position, 0.0, 1.0);
    }"#;

pub const SIMPLE_VERTEX_SOURCE: ShaderSource = ShaderSource {
    source_glsl: VS_SRC,
    var_name: "position",
};

const FS_SRC: &'static str = r#"#version 150
    out vec4 out_color;
    void main() {
       out_color = vec4(1.0, 1.0, 1.0, 1.0);
    }"#;

pub const SIMPLE_FRAGMENT_SOURCE: ShaderSource = ShaderSource {
    source_glsl: FS_SRC,
    var_name: "out_color",
};
