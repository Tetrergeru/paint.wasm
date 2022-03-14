use web_sys::{WebGl2RenderingContext as Gl, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

use super::{init_shader_program, make_f32_buffer, VS_SOURCE};

pub struct HsvCircleShader {
    program: WebGlProgram,
    buffer: WebGlBuffer,
    buffer_length: i32,

    width: i32,
    height: i32,

    vertex_location: u32,
    radius_location: WebGlUniformLocation,
}

const FS_SOURCE: &str = include_str!("src/hsv_circle.frag");

impl HsvCircleShader {
    pub fn new(gl: &Gl, width: i32, height: i32) -> Self {
        let program = init_shader_program(gl, VS_SOURCE, FS_SOURCE);

        let buffer = make_f32_buffer(
            gl,
            &[
                -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, //
                -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
            ],
        );
        let buffer_length = 6;
        let vertex_location = gl.get_attrib_location(&program, "vertexPosition") as u32;
        let radius_location = gl.get_uniform_location(&program, "radius").unwrap();
        Self {
            program,
            buffer,
            buffer_length,
            width,
            height,
            vertex_location,
            radius_location,
        }
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
    }

    pub fn draw(&self, gl: &Gl, x: i32, y: i32, radius: i32) {
        gl.viewport(
            x - radius,
            (self.height - y) - radius,
            2 * radius,
            2 * radius,
        );

        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(self.vertex_location, 2, Gl::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.vertex_location);

        gl.use_program(Some(&self.program));

        gl.uniform1f(Some(&self.radius_location), radius as f32);

        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);

        gl.draw_arrays(Gl::TRIANGLES, 0, self.buffer_length);
        gl.disable(Gl::BLEND);

        gl.viewport(0, 0, self.width, self.height);
    }
}
