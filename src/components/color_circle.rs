use web_sys::{HtmlCanvasElement, MouseEvent};
use yew::{html, Callback, Component, Context, NodeRef, Properties};

use crate::{color::Color, vector::Vector2, virtual_context::VirtualContext};

pub enum Msg {
    Down(MouseEvent),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub color_pick: Callback<Color>,
}

pub struct ColorCircle {
    canvas_ref: NodeRef,

    virtual_context: Option<VirtualContext>,

    width: i32,
    height: i32,
}

impl Component for ColorCircle {
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
                let color = self.draw(e.offset_x() as f64, e.offset_y() as f64);

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

            self.draw(self.width as f64 / 2.0, self.height as f64 / 2.0);
        }
    }
}

impl ColorCircle {
    fn draw(&self, x0: f64, y0: f64) -> Color {
        let context = self.virtual_context.as_ref().unwrap();
        context.clear(Color::new(0, 255, 255, 0));

        let x = (self.width / 2) as f64;
        let y = (self.height / 2) as f64;
        let r = (self.width / 2 - 5) as f64;

        context.hsv_circle(x, y, r);

        let point = Vector2::new((x0 - x) / r, (y0 - y) / r);

        let (color, point) = point_hsv_to_rgb(point);

        let (x, y) = (x as f64 + point.x * r, y as f64 + point.y * r);

        context.fill_circle(x, y, 5.0, color);

        context.draw_circle(x, y, 5.0, 1.0);

        color
    }
}

fn point_hsv_to_rgb(point: Vector2) -> (Color, Vector2) {
    if point.x == 0.0 && point.y == 0.0 {
        return (Color::WHITE, point);
    }

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
