use crate::{
    color::Palette,
    components::{
        color_picker::ColorPicker, draganddrop_container::DraganddropContainer,
        layers_widget::LayersWidget,
    },
    layer_manager::{LayerManager, RcLayerManager},
};
use yew::{html, Component, Context, NodeRef};

pub struct App {
    my_input: NodeRef,
    palette: Palette,
    layer_manager: RcLayerManager,
}

pub enum Msg {
    ColorPicked(Palette),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        log::debug!("Component create");
        let layer_manager: RcLayerManager = LayerManager::new(1000, 500).into();
        layer_manager.borrow_mut().push_layer();
        layer_manager.borrow_mut().push_layer();
        Self {
            my_input: NodeRef::default(),
            palette: Palette::default(),
            layer_manager,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ColorPicked(palette) => {
                self.palette = palette;
                false
            }
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
                <DraganddropContainer
                    text="Layers"
                    event_target={self.my_input.clone()}
                    key="layers"
                    left=500
                >
                    <LayersWidget
                        manager={self.layer_manager.clone()}
                    />
                </DraganddropContainer>
                <div class="layers">
                    <canvas class="canvas-layer" id="layer-canvas-1"></canvas>
                    <canvas class="canvas-layer" id="layer-canvas-2"></canvas>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            log::info!("App first_render");
            self.layer_manager.borrow().draw_in_context(0, |context| {
                context.line(10.0, 10.0, 200.0, 200.0, 20.0);
            })
        }
    }
}
