use vertex;

pub struct TexRect {
    lower_x: f32,
    upper_x: f32,
    lower_y: f32,
    upper_y: f32,
}

impl TexRect {
    pub fn new(lower_x: f32, upper_x: f32, lower_y: f32, upper_y: f32) -> TexRect {
        return TexRect {
            lower_x: lower_x,
            upper_x: upper_x,
            lower_y: lower_y,
            upper_y: upper_y,
        };
    }
}

impl vertex::VertexSpecable for TexRect {
    fn get_vertex_specification(&self) -> vertex::VertexSpecification {
        return vertex::VertexSpecification {
            vertices: vec![Box::new(vertex::TextureVertex {
                               x: self.lower_x,
                               y: self.upper_y,
                               tex_x: 0.0,
                               tex_y: 0.0,
                           }), // Top-left
                           Box::new(vertex::TextureVertex {
                               x: self.upper_x,
                               y: self.upper_y,
                               tex_x: 1.0,
                               tex_y: 0.0,
                           }), // Top-right
                           Box::new(vertex::TextureVertex {
                               x: self.upper_x,
                               y: self.lower_y,
                               tex_x: 1.0,
                               tex_y: 1.0,
                           }), // Bottom-right
                           Box::new(vertex::TextureVertex {
                               x: self.lower_x,
                               y: self.lower_y,
                               tex_x: 0.0,
                               tex_y: 1.0,
                           }) /* Bottom-left */],
            elements: vec![vertex::ElementTriangle {
                               p1: 0,
                               p2: 1,
                               p3: 2,
                           },
                           vertex::ElementTriangle {
                               p1: 2,
                               p2: 3,
                               p3: 0,
                           }],
        };
    }
}
