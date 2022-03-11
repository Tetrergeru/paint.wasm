use yew::{html, Component, Context, NodeRef};

mod components;
mod shaders;

use components::{
    draganddrop_container::DraganddropContainer,
    color_picker::ColorPicker,
};

pub struct App {
    my_input: NodeRef,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            my_input: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        html! {
            <div ref={self.my_input.clone()} style={"height: 100vh"}>
                <DraganddropContainer event_target={self.my_input.clone()}>
                    <ColorPicker />
                </DraganddropContainer>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
