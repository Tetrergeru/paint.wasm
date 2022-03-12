use std::rc::Rc;

use web_sys::{WebGl2RenderingContext as Gl, WebGlBuffer, WebGlProgram};

use super::{init_shader_program, make_f32_buffer};

pub struct HsvCircle {
    gl: Rc<Gl>,
    program: WebGlProgram,
    buffer: WebGlBuffer,
    buffer_length: i32,
    vertex_position: u32,
    viewport: (i32, i32, i32, i32),
}

impl HsvCircle {
    pub fn new(gl: Rc<Gl>, viewport: (i32, i32, i32, i32)) -> Self {
        let program = init_shader_program(&gl, VS_SOURCE, FS_SOURCE);

        let buffer = make_f32_buffer(
            &gl,
            &[
                -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
            ],
        );
        let buffer_length = 6;
        let vertex_position = gl.get_attrib_location(&program, "vertexPosition") as u32;
        Self {
            gl,
            program,
            buffer,
            buffer_length,
            vertex_position,
            viewport,
        }
    }

    pub fn draw(&self) {
        self.gl.viewport(self.viewport.0, self.viewport.1, self.viewport.2, self.viewport.3);

        self.gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&self.buffer));
        self.gl
            .vertex_attrib_pointer_with_i32(self.vertex_position, 2, Gl::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(self.vertex_position);

        self.gl.use_program(Some(&self.program));

        self.gl.enable(Gl::BLEND);
        self.gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);

        self.gl.draw_arrays(Gl::TRIANGLES, 0, self.buffer_length);
        self.gl.disable(Gl::BLEND);
    }
}

const VS_SOURCE: &str = "#version 300 es

in vec2 vertexPosition;

out vec2 fragCoord;

void main() {
    gl_Position = vec4(vertexPosition, 0.0, 1.0);
    fragCoord = vertexPosition;
}
";

const FS_SOURCE: &str = "#version 300 es
precision mediump float;

in vec2 fragCoord;

out vec4 color;

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
    else
        color = vec4(hsvToRgb(angle, pow(dist, 1.7), 1.0), 1.0);// - vec4(hsvToRgb(angle, sqrt(dist), 1.0), 1.0);
}
";
