use vertex;

pub trait Updateable {
    fn update_offset(&mut self, x: f32, y: f32);
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn get_color_floats(&self) -> (f32, f32, f32) {
        let red = f32::from(self.red) / 255.0;
        let green = f32::from(self.green) / 255.0;
        let blue = f32::from(self.blue) / 255.0;
        return (red, green, blue);
    }
}

struct LocInfo {
    x: f32,
    y: f32,
    orig_x: f32,
    orig_y: f32,
}

impl LocInfo {
    fn update_offset(&mut self, x_offset: f32, y_offset: f32) {
        self.x = self.orig_x + x_offset;
        self.y = self.orig_y + y_offset;
    }
}

pub struct SimpleRect {
    loc: LocInfo,
    width: f32,
    height: f32,
    color: Color,
}

impl SimpleRect {
    pub fn new(xloc: f32,
               yloc: f32,
               width: f32,
               height: f32,
               red: u8,
               green: u8,
               blue: u8)
               -> SimpleRect {
        return SimpleRect {
            loc: LocInfo {
                x: xloc,
                y: yloc,
                orig_x: xloc,
                orig_y: yloc,
            },
            width: width,
            height: height,
            color: Color {
                red: red,
                green: green,
                blue: blue,
            },
        };
    }

    fn calc_corners(&self) -> (f32, f32, f32, f32) {
        let top = self.loc.y + (self.height / 2.0);
        let bottom = self.loc.y - (self.height / 2.0);
        let right = self.loc.x + (self.width / 2.0);
        let left = self.loc.x - (self.width / 2.0);
        return (top, bottom, left, right);
    }
}

impl Updateable for SimpleRect {
    fn update_offset(&mut self, x_offset: f32, y_offset: f32) {
        self.loc.update_offset(x_offset, y_offset)
    }
}

impl vertex::VertexSpecable for SimpleRect {
    fn get_vertex_specification(&self) -> vertex::VertexSpecification {
        let (top, bottom, left, right) = self.calc_corners();
        let (red, green, blue) = self.color.get_color_floats();
        // top-left, top-right, bottom-left, bottom-right
        let vertices: Vec<Box<vertex::Vertex>> = vec![Box::new(vertex::ColorVertex {
                                                          x: left,
                                                          y: top,
                                                          red: red,
                                                          green: green,
                                                          blue: blue,
                                                      }),
                                                      Box::new(vertex::ColorVertex {
                                                          x: right,
                                                          y: top,
                                                          red: red,
                                                          green: green,
                                                          blue: blue,
                                                      }),
                                                      Box::new(vertex::ColorVertex {
                                                          x: right,
                                                          y: bottom,
                                                          red: red,
                                                          green: green,
                                                          blue: blue,
                                                      }),
                                                      Box::new(vertex::ColorVertex {
                                                          x: left,
                                                          y: bottom,
                                                          red: red,
                                                          green: green,
                                                          blue: blue,
                                                      })];

        // the elements each point to what 3 points make up a single triangle
        // given the elements below and the vertex data, we see the triangles
        // are as follows:
        //
        // triangle one | triangle two
        //  o--o        |    o
        //  | /         |   /|
        //  |/          |  / |
        //  o           | o--o
        let elements = vec![vertex::ElementTriangle {
                                p1: 0,
                                p2: 1,
                                p3: 2,
                            },
                            vertex::ElementTriangle {
                                p1: 2,
                                p2: 3,
                                p3: 0,
                            }];

        return vertex::VertexSpecification {
            vertices: vertices,
            elements: elements,
        };
    }
}

pub struct SimpleTriangle {
    loc: LocInfo,
    width: f32,
    height: f32,
    color: Color,
}

impl SimpleTriangle {
    pub fn new(xloc: f32,
               yloc: f32,
               width: f32,
               height: f32,
               red: u8,
               green: u8,
               blue: u8)
               -> SimpleTriangle {
        return SimpleTriangle {
            loc: LocInfo {
                x: xloc,
                y: yloc,
                orig_x: xloc,
                orig_y: yloc,
            },
            width: width,
            height: height,
            color: Color {
                red: red,
                green: green,
                blue: blue,
            },
        };
    }

    fn calc_points(&self) -> (f32, f32, f32, f32, f32) {
        let top = self.loc.y + (self.height / 2.0);
        let bottom = self.loc.y - (self.height / 2.0);
        let right = self.loc.x + (self.width / 2.0);
        let left = self.loc.x - (self.width / 2.0);
        let middle = self.loc.x;
        return (top, bottom, left, right, middle);
    }
}

impl Updateable for SimpleTriangle {
    fn update_offset(&mut self, x_offset: f32, y_offset: f32) {
        self.loc.update_offset(x_offset, y_offset)
    }
}

impl vertex::VertexSpecable for SimpleTriangle {
    fn get_vertex_specification(&self) -> vertex::VertexSpecification {
        let (top, bottom, left, right, middle) = self.calc_points();
        let (red, green, blue) = self.color.get_color_floats();
        // top-middle, bottom-right, bottom-left
        let vertices: Vec<Box<vertex::Vertex>> = vec![Box::new(vertex::ColorVertex {
                                                          x: middle,
                                                          y: top,
                                                          red: red,
                                                          green: green,
                                                          blue: blue,
                                                      }),
                                                      Box::new(vertex::ColorVertex {
                                                          x: right,
                                                          y: bottom,
                                                          red: red,
                                                          green: green,
                                                          blue: blue,
                                                      }),
                                                      Box::new(vertex::ColorVertex {
                                                          x: left,
                                                          y: bottom,
                                                          red: red,
                                                          green: green,
                                                          blue: blue,
                                                      })];

        // the elements each point to what 3 points make up a single triangle
        // given the elements below and the vertex data, we see the triangle
        // is as follows:
        //
        // triangle
        //  o--o
        //  | /
        //  |/
        //  o
        let elements = vec![vertex::ElementTriangle {
                                p1: 0,
                                p2: 1,
                                p3: 2,
                            }];

        return vertex::VertexSpecification {
            vertices: vertices,
            elements: elements,
        };
    }
}
