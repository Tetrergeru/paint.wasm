use crate::{
    color::{Color, Palette},
    components::{
        color_picker::ColorPicker, draganddrop_container::DraganddropContainer,
        layers_widget::LayersWidget,
    },
    layer_manager::{LayerManager, RcLayerManager},
    virtual_context::VirtualContext,
};
use web_sys::{HtmlCanvasElement, MouseEvent};
use yew::{html, Component, Context, NodeRef};

pub struct App {
    my_input: NodeRef,
    palette: Palette,
    layer_manager: RcLayerManager,

    canvas_ref: NodeRef,
    context: Option<VirtualContext>,
}

impl App {
    fn draw(&self) {
        let context = self.context.as_ref().unwrap();
        context.checkerboard(10, Color::new(191, 191, 191, 255), Color::WHITE);
        for layer in self.layer_manager.borrow().iter_layers() {
            context.draw_image(layer.get_canvas());
        }
    }
}

pub enum Msg {
    ColorPicked(Palette),
    MouseDown(MouseEvent),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        log::debug!("Component create");
        let layer_manager: RcLayerManager = LayerManager::new(1000, 500).into();
        layer_manager.borrow_mut().push_layer();
        layer_manager.borrow_mut().push_layer();
        layer_manager.borrow_mut().push_layer();
        layer_manager.borrow_mut().push_layer();
        layer_manager.borrow_mut().push_layer();
        Self {
            my_input: NodeRef::default(),
            palette: Palette::default(),
            layer_manager,
            canvas_ref: NodeRef::default(),
            context: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ColorPicked(palette) => {
                self.palette = palette;
                false
            }
            Msg::MouseDown(e) => {
                self.layer_manager.borrow().draw_in_context(|context| {
                    context.fill_circle(
                        e.offset_x() as f64,
                        e.offset_y() as f64,
                        50.0,
                        self.palette.main,
                    );
                });

                self.draw();

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <div ref={self.my_input.clone()} class="main-container" style={"height: 100vh"}>
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
                <div>
                    <canvas
                        ref={self.canvas_ref.clone()}
                        width=1000
                        height=500
                        style="background-color: white;"
                        onmousedown={ctx.link().callback(Msg::MouseDown)}
                    />
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            // log::debug!("{:?}", canvas);
            self.context = Some(VirtualContext::new(canvas, 1000, 500));
        }

        self.draw();
    }
}
