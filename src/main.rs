use color::{Color, Palette};
use yew::{html, Component, Context, NodeRef};

mod components;
mod shaders;
mod virtual_context;
mod color;
mod vector;

use components::{color_picker::ColorPicker, draganddrop_container::DraganddropContainer};

pub struct App {
    my_input: NodeRef,
    palette: Palette,
}

pub enum Msg {
    ColorPicked(Palette),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            my_input: NodeRef::default(),
            palette: Palette::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ColorPicked(palette) => {
                self.palette = palette;
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <div ref={self.my_input.clone()} style={"height: 100vh"}>
                <DraganddropContainer 
                    text="Pick color" 
                    event_target={self.my_input.clone()}
                    key="colorpicker"
                >
                    <ColorPicker
                        color_pick={ctx.link().callback(Msg::ColorPicked)}
                    />
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
