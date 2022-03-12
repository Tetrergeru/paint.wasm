use web_sys::{
    WebGl2RenderingContext as Gl, WebGlBuffer, WebGlProgram, WebGlTexture, WebGlUniformLocation,
};

use super::{init_shader_program, make_f32_buffer, uniform_texture, VS_SOURCE};

pub struct CopyImageShader {
    program: WebGlProgram,
    buffer: WebGlBuffer,
    buffer_length: i32,

    width: i32,
    height: i32,

    vertex_location: u32,
    image_location: WebGlUniformLocation,
}

impl CopyImageShader {
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
        let image_location = gl.get_uniform_location(&program, "image").unwrap();
        Self {
            program,
            buffer,
            buffer_length,
            width,
            height,
            vertex_location,
            image_location,
        }
    }

    pub fn draw(&self, gl: &Gl, x: i32, y: i32, width: i32, height: i32, image: &WebGlTexture) {
        gl.viewport(x, self.height - y - height, width, height);

        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(self.vertex_location, 2, Gl::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.vertex_location);

        gl.use_program(Some(&self.program));

        // gl.uniform1i(Some(&self.image_location), image.deref());
        uniform_texture(gl, &self.image_location, image);

        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);

        gl.draw_arrays(Gl::TRIANGLES, 0, self.buffer_length);
        gl.disable(Gl::BLEND);

        gl.viewport(0, 0, self.width, self.height);
    }
}

const FS_SOURCE: &str = "#version 300 es
precision mediump float;

uniform sampler2D image;

in vec2 fragCoord;

out vec4 color;

void main() {
    vec2 pos = vec2(fragCoord.x, -fragCoord.y);
    color = texture(image, vec2(0.5, 0.5) + pos * 0.5);
}
";
