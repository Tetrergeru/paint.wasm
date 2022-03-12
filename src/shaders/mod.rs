use js_sys::Float32Array;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGlProgram, WebGl2RenderingContext as Gl, WebGlShader, WebGlBuffer, WebGlTexture, HtmlCanvasElement, WebGlUniformLocation};

pub mod hsv_circle;
pub mod copy_image;

pub fn make_f32_buffer(gl: &Gl, array: &[f32]) -> WebGlBuffer {
    let buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&buffer));

    gl.buffer_data_with_array_buffer_view(
        Gl::ARRAY_BUFFER,
        &Float32Array::from(array),
        Gl::STATIC_DRAW,
    );

    gl.bind_buffer(Gl::ARRAY_BUFFER, None);

    buffer
}

pub fn init_shader_program(gl: &Gl, vs_source: &str, fs_source: &str) -> WebGlProgram {
    let vs = load_shader(gl, Gl::VERTEX_SHADER, vs_source);
    let fs = load_shader(gl, Gl::FRAGMENT_SHADER, fs_source);

    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, &vs);
    gl.attach_shader(&program, &fs);
    gl.link_program(&program);

    if !gl.get_program_parameter(&program, Gl::LINK_STATUS) {
        panic!("Unable to initialize the shader program: {:?}", gl.get_program_info_log(&program));
    }

    program
}

fn load_shader(gl: &Gl, typ: u32, source: &str) -> WebGlShader {
    let shader = gl.create_shader(typ).unwrap();

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if !gl.get_shader_parameter(&shader, Gl::COMPILE_STATUS) {
        panic!(
            "An error occurred compiling the shaders: {:?}",
            gl.get_shader_info_log(&shader)
        );
    }

    shader
}

pub fn load_texture_from_canvas(gl: &Gl, texture: &WebGlTexture, image: &HtmlCanvasElement) {
    gl.bind_texture(Gl::TEXTURE_2D, Some(texture));
    gl.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
        Gl::TEXTURE_2D,
        0,
        Gl::RGBA as i32,
        Gl::RGBA,
        Gl::UNSIGNED_BYTE,
        image
    ).unwrap();
    gl.generate_mipmap(Gl::TEXTURE_2D);
}


#[wasm_bindgen(module = "/src/shaders/helpers.js")]
extern "C" {
    #[wasm_bindgen(js_name = "uniformTexture")]
    pub fn uniform_texture(gl: &Gl, location: &WebGlUniformLocation, texture: &WebGlTexture);

    #[wasm_bindgen(js_name = "debugCanvases")]
    pub fn debug_canvases(c1: &HtmlCanvasElement, c2: &HtmlCanvasElement);
}

const VS_SOURCE: &str = "#version 300 es

in vec2 vertexPosition;

out vec2 fragCoord;

void main() {
    gl_Position = vec4(vertexPosition, 0.0, 1.0);
    fragCoord = vertexPosition;
}
";