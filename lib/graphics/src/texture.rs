use gl;

use std::ffi::CString;
use std::mem;

use gl::types::*;

pub struct TextureSetupDefinition {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn texture_load(program: GLuint, tex_def: TextureSetupDefinition) {
    unsafe {
        let mut texture = mem::uninitialized();
        gl::GenTextures(1, &mut texture);

        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGBA as GLint, // GLint internalFormat,
                       tex_def.width as i32,
                       tex_def.height as i32,
                       0,
                       gl::RGBA as GLenum, // GLenum format,
                       gl::UNSIGNED_BYTE, // GLenum type,
                       tex_def.data.as_slice().as_ptr() as *const _);
        gl::Uniform1i(gl::GetUniformLocation(program,
                                             CString::new("tex_sample").unwrap().as_ptr()),
                      0);

        gl::TexParameteri(gl::TEXTURE_2D,
                          gl::TEXTURE_WRAP_S,
                          gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D,
                          gl::TEXTURE_WRAP_T,
                          gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }
}
