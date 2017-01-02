extern crate gl;

use gl::types::*;

pub struct ShaderSource {
    pub source_glsl: &'static str,
    pub var_name: &'static str,
    pub stride: GLsizei,
    pub offset: usize,
}

pub const COLOR_VERTEX_SOURCE: ShaderSource = ShaderSource {
    source_glsl: VS_SRC,
    var_name: "position",
    stride: 2,
    offset: 0,
};

const VS_SRC: &'static str = r#"#version 150
    in vec2 position;
    in vec3 color;
    out vec3 attr_color;
    void main() {
       attr_color = color;
       gl_Position = vec4(position, 0.0, 1.0);
    }"#;

pub const COLOR_FRAGMENT_SOURCE: ShaderSource = ShaderSource {
    source_glsl: FS_SRC,
    var_name: "color",
    stride: 3,
    offset: 2,
};

const FS_SRC: &'static str = r#"#version 150
    in vec3 attr_color;
    out vec4 out_color;
    void main() {
       out_color = vec4(attr_color, 1.0);
    }"#;
