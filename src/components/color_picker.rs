use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as Gl};
use yew::{html, Component, Context, NodeRef};

use crate::shaders::{init_shader_program, make_f32_buffer};

pub enum Msg {}

pub struct ColorPicker {
    canvas_ref: NodeRef,
}

impl Component for ColorPicker {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
        }
    }

    // fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    //     false
    // }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        html! {
            <canvas ref={self.canvas_ref.clone()} width=200 height=200/>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        // if !_first_render {
        //     return;
        // }

        log::info!("Rerendered color picker");

        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<Gl>()
            .unwrap();

        gl.viewport(0, 0, canvas.client_width(), canvas.client_height());

        let program = init_shader_program(&gl, VS_SOURCE, FS_SOURCE);
        let buffer = make_f32_buffer(
            &gl,
            &[
                -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
            ],
        );
        let buffer_length = 6;
        let vertex_position = gl.get_attrib_location(&program, "vertexPosition") as u32;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);
        gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);

        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&buffer));
        gl.vertex_attrib_pointer_with_i32(vertex_position, 2, Gl::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(vertex_position);

        gl.use_program(Some(&program));

        gl.draw_arrays(Gl::TRIANGLES, 0, buffer_length);
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
        color = vec4(1.0, 1.0, 1.0, 1.0);
    else
        color = vec4(hsvToRgb(angle, dist, 1.0), 1.0);
}
";
