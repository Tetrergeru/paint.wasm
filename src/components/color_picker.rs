use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, WebGl2RenderingContext as Gl};
use yew::{html, Component, Context, NodeRef};

use crate::shaders::hsv_circle::HsvCircle;

pub enum Msg {
    Down(MouseEvent),
}

pub struct ColorPicker {
    canvas_ref: NodeRef,

    gl: Option<Rc<Gl>>,
    hsv_circle: Option<HsvCircle>,

    width: i32,
    heigth: i32,
}

impl Component for ColorPicker {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            gl: None,
            hsv_circle: None,
            width: 200,
            heigth: 200,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Down(e) => {
                self.hsv_circle.as_ref().unwrap().draw();

                let hsv_circle = HsvCircle::new(
                    self.gl.as_ref().unwrap().clone(),
                    (e.layer_x() - 5, self.heigth - e.layer_y() - 5, 10, 10),
                );

                hsv_circle.draw();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <canvas
                ref={self.canvas_ref.clone()} width=200 height=200
                onmousedown={ctx.link().clone().callback(Msg::Down)}
            />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if _first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let gl = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<Gl>()
                .unwrap();
            self.gl = Some(Rc::new(gl));

            let hsv_circle = HsvCircle::new(
                self.gl.as_ref().unwrap().clone(),
                (10, 10, self.width - 20, self.heigth - 20),
            );
            self.hsv_circle = Some(hsv_circle);
        }

        self.hsv_circle.as_ref().unwrap().draw();
    }
}
