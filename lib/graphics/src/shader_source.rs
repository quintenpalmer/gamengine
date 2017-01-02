extern crate gl;

use gl::types::*;

pub enum GLShaderEnum {
    VertexShader,
    FragmentShader,
}

impl GLShaderEnum {
    pub fn to_glenum(&self) -> GLenum {
        match self {
            &GLShaderEnum::VertexShader => gl::VERTEX_SHADER,
            &GLShaderEnum::FragmentShader => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct RenderingPipelineSource {
    pub vertex_glsl: &'static str,
    pub fragment_glsl: &'static str,
    pub all_vertex_attrs: Vec<VertexAttribute>,
    pub vertex_width: u8,
}

pub struct VertexAttribute {
    pub var_name: &'static str,
    pub stride: GLsizei,
    pub offset: usize,
}

pub fn color_pipeline_source() -> RenderingPipelineSource {
    return RenderingPipelineSource {
        vertex_glsl: COLOR_VS_GLSL,
        fragment_glsl: COLOR_FS_GLSL,
        all_vertex_attrs: vec![POSITION_VERTEX_ATTR, COLOR_VERTEX_ATTR],
        vertex_width: 5, // this is the width of a ColorVertex: x, y, red, green, blue
    };
}

const POSITION_VERTEX_ATTR: VertexAttribute = VertexAttribute {
    var_name: "position",
    stride: 2,
    offset: 0,
};

const COLOR_VERTEX_ATTR: VertexAttribute = VertexAttribute {
    var_name: "color",
    stride: 3,
    offset: 2,
};

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
