extern crate gl;

use gl::types::*;

pub trait GLShader {
    fn to_glenum(&self) -> GLenum;
    fn get_glsl(&self) -> &'static str;
}

pub struct GLVertexShader {
    glsl: &'static str,
}

impl GLShader for GLVertexShader {
    fn to_glenum(&self) -> GLenum {
        return gl::VERTEX_SHADER;
    }

    fn get_glsl(&self) -> &'static str {
        return self.glsl;
    }
}

pub struct GLFragmentShader {
    glsl: &'static str,
}

impl GLShader for GLFragmentShader {
    fn to_glenum(&self) -> GLenum {
        return gl::FRAGMENT_SHADER;
    }

    fn get_glsl(&self) -> &'static str {
        return self.glsl;
    }
}

pub struct RenderingPipelineSource {
    pub vertex_glsl: GLVertexShader,
    pub fragment_glsl: GLFragmentShader,
    pub all_vertex_attrs: Vec<VertexAttribute>,
    pub vertex_width: u8,
}

pub struct VertexAttribute {
    pub var_name: &'static str,
    pub stride: GLsizei,
}

pub fn color_pipeline_source() -> RenderingPipelineSource {
    return RenderingPipelineSource {
        vertex_glsl: GLVertexShader { glsl: COLOR_VS_GLSL },
        fragment_glsl: GLFragmentShader { glsl: COLOR_FS_GLSL },
        all_vertex_attrs: vec![VertexAttribute {
                                   var_name: "position",
                                   stride: 2,
                               },
                               VertexAttribute {
                                   var_name: "color",
                                   stride: 3,
                               }],
        vertex_width: 5, // this is the width of a ColorVertex: x, y, red, green, blue
    };
}

const COLOR_VS_GLSL: &'static str = r#"#version 150
    in vec2 position;
    in vec3 color;
    out vec3 attr_color;
    void main() {
       attr_color = color;
       gl_Position = vec4(position, 0.0, 1.0);
    }"#;

const COLOR_FS_GLSL: &'static str = r#"#version 150
    in vec3 attr_color;
    out vec4 out_color;
    void main() {
       out_color = vec4(attr_color, 1.0);
    }"#;
