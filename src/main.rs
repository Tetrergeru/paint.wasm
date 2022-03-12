use color::Color;
use yew::{html, Component, Context, NodeRef};

mod components;
mod shaders;
mod virtual_context;
mod color;
mod vector;

use components::{color_picker::ColorPicker, draganddrop_container::DraganddropContainer};

pub struct App {
    my_input: NodeRef,
    color: Color,
}

pub enum Msg {
    ColorPicked(Color),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            my_input: NodeRef::default(),
            color: Color::new(0, 0, 0, 255),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ColorPicked(color) => {
                self.color = color;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <div ref={self.my_input.clone()} style={"height: 100vh"}>
                <DraganddropContainer text={self.color.to_style()} event_target={self.my_input.clone()}>
                    <ColorPicker color_pick={ctx.link().callback(Msg::ColorPicked)}/>
                </DraganddropContainer>

                <div class="layers">
                    <canvas class="canvas-layer" id="layer-canvas-1"></canvas>
                    <canvas class="canvas-layer" id="layer-canvas-2"></canvas>
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
