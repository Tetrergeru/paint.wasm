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

const FS_SOURCE: &str = "#version 300 es
precision mediump float;

in vec2 fragCoord;

out vec4 color;

uniform float radius;

vec3 hsvToRgb(float hue, float s, float v) {
    float h = hue / 60.0;

    float c = v * s;

    float x = c * (1.0 - abs(mod(h, 2.0) - 1.0));

    vec3 rgb;

    if (0.0 <= h && h < 1.0) rgb = vec3(c, x, 0.0);
    else if (1.0 <= h && h < 2.0) rgb = vec3(x, c, 0.0);
    else if (2.0 <= h && h < 3.0) rgb = vec3(0.0, c, x);
    else if (3.0 <= h && h < 4.0) rgb = vec3(0.0, x, c);
    else if (4.0 <= h && h < 5.0) rgb = vec3(x, 0.0, c);
    else if (5.0 <= h && h < 6.0) rgb = vec3(c, 0.0, x);

    float m = v - c;

    return rgb + vec3(m, m, m);
}

const float PIx2 = 2.0 * 3.1415;

void main() {
    float dist = sqrt(dot(fragCoord, fragCoord));

    vec2 norm = fragCoord / dist;

    float angle;
    if (norm.y < 0.0)
        angle = acos(norm.x) * 360.0 / PIx2;
    else
        angle = (PIx2 - acos(norm.x)) * 360.0 / PIx2;

    if (dist > 1.0)
        discard;

    vec3 targetColor = hsvToRgb(angle, dist, 1.0);

    float border = 1.5 / radius;

    if (dist > 1.0 - border)
        color = vec4(targetColor, -(dist - 1.0) / border);
    else
        color = vec4(targetColor, 1.0);
}
";
