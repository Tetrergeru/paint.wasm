use gloo::utils::document;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture};

use crate::{shaders::{
    copy_image::CopyImageShader, debug_canvases, hsv_circle::HsvCircleShader,
    load_texture_from_canvas,
}, color::Color};

pub struct VirtualContext {
    canvas_2d: HtmlCanvasElement,
    context_2d: CanvasRenderingContext2d,
    canvas_gl: HtmlCanvasElement,
    context_gl: WebGl2RenderingContext,

    hsv_circle: HsvCircleShader,
    copy_image: CopyImageShader,

    texture_for_swaps: WebGlTexture,
}

impl VirtualContext {
    pub fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> Self {
        canvas.set_width(width);
        canvas.set_height(height);
        let context_2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let canvas_gl: HtmlCanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .unchecked_into();
        canvas_gl.set_width(width);
        canvas_gl.set_height(height);
        let context_gl = canvas_gl
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        Self {
            texture_for_swaps: context_gl.create_texture().unwrap(),
            hsv_circle: HsvCircleShader::new(
                &context_gl,
                canvas_gl.width() as i32,
                canvas_gl.height() as i32,
            ),
            copy_image: CopyImageShader::new(
                &context_gl,
                canvas_gl.width() as i32,
                canvas_gl.height() as i32,
            ),
            canvas_2d: canvas,
            context_2d,
            canvas_gl,
            context_gl,
        }
    }

    pub fn hsv_circle(&self, x: i32, y: i32, r: i32) {
        self.hsv_circle.draw(&self.context_gl, x, y, r);
        self.flush_gl_to_2d();
    }

    pub fn line(&self, x0: f64, y0: f64, x1: f64, y1: f64, width: f64) {
        self.context_2d.begin_path();
        self.context_2d
            .set_stroke_style(&JsValue::from_str("black"));
        self.context_2d.set_line_width(width);
        self.context_2d.move_to(x0, y0);
        self.context_2d.line_to(x1, y1);
        self.context_2d.stroke();
        self.context_2d.close_path();
        self.flush_2d_to_gl();
    }

    pub fn clear(&self, color: Color) {
        self.context_2d.set_fill_style(&JsValue::from_str(&color.to_style()));
        self.context_2d.fill_rect(
            0.0,
            0.0,
            self.canvas_2d.width() as f64,
            self.canvas_2d.height() as f64,
        );
        self.flush_2d_to_gl();
    }

    pub fn fill_circle(&self, x0: f64, y0: f64, r: f64, color: Color) {
        self.context_2d.begin_path();
        self.context_2d.set_fill_style(&JsValue::from_str(&color.to_style()));
        self.context_2d
            .arc(x0, y0, r, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        self.context_2d.fill();
        self.context_2d.close_path();
        self.flush_2d_to_gl();
    }

    pub fn draw_circle(&self, x0: f64, y0: f64, r: f64, width: f64) {
        self.context_2d.begin_path();
        self.context_2d
            .set_stroke_style(&JsValue::from_str("black"));
        self.context_2d.set_line_width(width);
        self.context_2d
            .arc(x0, y0, r, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        self.context_2d.stroke();
        self.context_2d.close_path();
        self.flush_2d_to_gl();
    }

    fn flush_2d_to_gl(&self) {
        load_texture_from_canvas(&self.context_gl, &self.texture_for_swaps, &self.canvas_2d);
        self.copy_image.draw(
            &self.context_gl,
            0,
            0,
            self.canvas_gl.width() as i32,
            self.canvas_gl.height() as i32,
            &self.texture_for_swaps,
        );
        debug_canvases(&self.canvas_2d, &self.canvas_gl);
    }

    fn flush_gl_to_2d(&self) {
        self.context_2d
            .draw_image_with_html_canvas_element(&self.canvas_gl, 0.0, 0.0)
            .unwrap();
        debug_canvases(&self.canvas_2d, &self.canvas_gl);
    }
}
