use gloo::utils::document;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture};

use crate::{
    color::Color,
    shaders::{
        checkerboard::CheckerboardShader, copy_image::CopyImageShader, hsv_circle::HsvCircleShader,
        load_texture_from_canvas,
    },
    vector::{Rectangle, Vector2},
};

pub struct VirtualContext {
    canvas_2d: HtmlCanvasElement,
    context_2d: CanvasRenderingContext2d,
    canvas_gl: HtmlCanvasElement,
    context_gl: WebGl2RenderingContext,

    hsv_circle: HsvCircleShader,
    copy_image: CopyImageShader,
    checkerboard: CheckerboardShader,

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
            checkerboard: CheckerboardShader::new(
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

    pub fn new_independent(width: u32, height: u32) -> Self {
        let canvas: HtmlCanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .unchecked_into();
        Self::new(canvas, width, height)
    }

    pub fn hsv_circle(&self, x: f64, y: f64, r: f64) {
        self.hsv_circle
            .draw(&self.context_gl, x as i32, y as i32, r as i32);
        self.flush_gl_to_2d();
    }

    pub fn checkerboard(&self, cell_size: f64, color_a: Color, color_b: Color) {
        self.checkerboard.draw(
            &self.context_gl,
            Vector2::new(cell_size as f64, cell_size as f64),
            color_a,
            color_b,
        );
        self.flush_gl_to_2d();
    }

    pub fn line(&self, x0: f64, y0: f64, x1: f64, y1: f64, width: f64, color: Color) {
        self.context_2d.begin_path();
        self.context_2d
            .set_stroke_style(&JsValue::from_str(&color.to_style()));
        self.context_2d.set_line_width(width);
        self.context_2d.move_to(x0, y0);
        self.context_2d.line_to(x1, y1);
        self.context_2d.stroke();
        self.context_2d.close_path();
        self.flush_2d_to_gl();
    }

    pub fn clear(&self, color: Color) {
        self.context_2d.save();
        self.context_2d
            .set_global_composite_operation("copy")
            .unwrap();
        self.context_2d
            .set_fill_style(&JsValue::from_str(&color.to_style()));
        self.context_2d.fill_rect(
            0.0,
            0.0,
            self.canvas_2d.width() as f64,
            self.canvas_2d.height() as f64,
        );
        self.context_2d.restore();
        self.flush_2d_to_gl();
    }

    pub fn fill_circle(&self, x0: f64, y0: f64, r: f64, color: Color) {
        self.context_2d.begin_path();
        self.context_2d
            .set_fill_style(&JsValue::from_str(&color.to_style()));
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

    pub fn draw_image(&self, image: &HtmlCanvasElement) {
        self.context_2d
            .draw_image_with_html_canvas_element(image, 0.0, 0.0)
            .unwrap();
        self.flush_2d_to_gl();
    }

    pub fn draw_image_bounded(&self, image: &HtmlCanvasElement, bounds: Rectangle) {
        self.context_2d
            .draw_image_with_html_canvas_element_and_dw_and_dh(
                image,
                bounds.coord.x,
                bounds.coord.y,
                bounds.size.x,
                bounds.size.y,
            )
            .unwrap();
        self.flush_2d_to_gl();
    }

    pub fn get_canvas(&self) -> &'_ HtmlCanvasElement {
        &self.canvas_2d
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.canvas_2d.set_width(width);
        self.canvas_gl.set_width(width);
        self.canvas_2d.set_height(height);
        self.canvas_gl.set_height(height);
        self.checkerboard.set_size(width as i32, height as i32);
        self.hsv_circle.set_size(width as i32, height as i32);
        self.copy_image.set_size(width as i32, height as i32);
    }

    pub fn width(&self) -> u32 {
        self.get_canvas().width()
    }

    pub fn height(&self) -> u32 {
        self.get_canvas().height()
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
    }

    fn flush_gl_to_2d(&self) {
        self.context_2d
            .draw_image_with_html_canvas_element(&self.canvas_gl, 0.0, 0.0)
            .unwrap();
    }
}
