use js_sys::Float32Array;
use web_sys::{WebGlProgram, WebGl2RenderingContext as Gl, WebGlShader, WebGlBuffer};

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
        // gl.deleteShader(shader);
        // return null;
    }

    shader
}