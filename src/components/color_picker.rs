use web_sys::{HtmlCanvasElement, MouseEvent};
use yew::{html, Callback, Component, Context, NodeRef, Properties};

use crate::{color::Color, vector::Vector2, virtual_context::VirtualContext};

pub enum Msg {
    Down(MouseEvent),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub color_pick: Callback<Color>,
}

pub struct ColorPicker {
    canvas_ref: NodeRef,

    virtual_context: Option<VirtualContext>,

    width: i32,
    height: i32,
}

impl Component for ColorPicker {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            virtual_context: None,
            width: 150,
            height: 150,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Down(e) => {
                self.virtual_context
                    .as_ref()
                    .unwrap()
                    .clear(Color::new(255, 255, 255, 255));

                let x = self.width / 2;
                let y = self.height / 2;
                let r = self.width / 2 - 10;

                self.virtual_context.as_ref().unwrap().hsv_circle(x, y, r);

                let point = Vector2::new(
                    (e.layer_x() - x) as f64 / r as f64,
                    (e.layer_y() - y) as f64 / r as f64,
                );

                let (color, point) = point_hsv_to_rgb(point);

                let (x, y) = (
                    x as f64 + point.x * (r - 1) as f64,
                    y as f64 + point.y * (r - 1) as f64,
                );

                self.virtual_context
                    .as_ref()
                    .unwrap()
                    .fill_circle(x, y, 5.0, color);

                self.virtual_context
                    .as_ref()
                    .unwrap()
                    .draw_circle(x, y, 5.0, 1.0);

                ctx.props().color_pick.emit(color);

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <canvas
                ref={self.canvas_ref.clone()} width={self.width.to_string()} height={self.height.to_string()}
                onmousedown={ctx.link().clone().callback(Msg::Down)}
            />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

            self.virtual_context = Some(VirtualContext::new(
                canvas,
                self.width as u32,
                self.height as u32,
            ));

            self.virtual_context
                .as_ref()
                .unwrap()
                .line(50.0, 50.0, 50.1, 50.1, 1.0);

            self.virtual_context.as_ref().unwrap().hsv_circle(
                self.width / 2,
                self.height / 2,
                self.width / 2 - 10,
            );
        }

        log::info!("draw");
    }
}

fn point_hsv_to_rgb(point: Vector2) -> (Color, Vector2) {
    let mut new_point = point;

    let mut dist = point.len();
    let norm = point.norm();

    if dist >= 1.0 {
        dist = 1.0;
        new_point = norm;
    }

    let angle = if norm.y > 0.0 {
        norm.x.acos()
    } else {
        std::f64::consts::TAU - norm.x.acos()
    };

    let color = Color::from_hsv(angle, dist, 1.0);

    (color, new_point)
}
