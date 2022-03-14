use crate::{
    color::{Color, Palette},
    components::{
        color_picker::ColorPicker, draganddrop_container::DraganddropContainer,
        layers_widget::LayersWidget,
    },
    layer_manager::{LayerManager, RcLayerManager},
    vector::{Rectangle, Vector2},
    virtual_context::VirtualContext,
};
use web_sys::{HtmlCanvasElement, MouseEvent, WheelEvent};
use yew::{html, Component, Context, NodeRef};

pub struct App {
    my_input: NodeRef,
    palette: Palette,
    layer_manager: RcLayerManager,

    canvas_ref: NodeRef,
    context: Option<VirtualContext>,
    previous_point: Option<Vector2>,

    line_width: f64,
    scale: f64,
}

impl App {
    fn draw(&self) {
        let context = self.context.as_ref().unwrap();
        context.checkerboard(self.scaled(), Color::new(191, 191, 191, 255), Color::WHITE);
        for layer in self.layer_manager.borrow().iter_layers() {
            context.draw_image_bounded(layer.get_canvas(), Rectangle::new(0.0, 0.0, 1000.0, 500.0));
        }
    }

    fn scaled(&self) -> f64 {
        (10.0 / self.scale).ceil()
    }
}

pub enum Msg {
    ColorPicked(Palette),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
    Wheel(WheelEvent),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
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
            previous_point: None,
            line_width: 50.0,
            scale: 1.0,
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
                        e.offset_x() as f64 / self.scale,
                        e.offset_y() as f64 / self.scale,
                        self.line_width / 2.0,
                        self.palette.main,
                    );
                });

                self.previous_point = Some(Vector2::new(
                    e.offset_x() as f64 / self.scale,
                    e.offset_y() as f64 / self.scale,
                ));

                self.draw();

                false
            }
            Msg::MouseMove(e) => {
                if let Some(prev) = self.previous_point {
                    self.layer_manager.borrow().draw_in_context(|context| {
                        context.fill_circle(
                            e.offset_x() as f64 / self.scale,
                            e.offset_y() as f64 / self.scale,
                            self.line_width / 2.0,
                            self.palette.main,
                        );
                        context.line(
                            prev.x,
                            prev.y,
                            e.offset_x() as f64 / self.scale,
                            e.offset_y() as f64 / self.scale,
                            self.line_width,
                            self.palette.main,
                        );
                    });

                    self.previous_point = Some(Vector2::new(
                        e.offset_x() as f64 / self.scale,
                        e.offset_y() as f64 / self.scale,
                    ));

                    self.draw();
                }

                false
            }
            Msg::MouseUp(e) => {
                self.layer_manager.borrow().draw_in_context(|context| {
                    context.fill_circle(
                        e.offset_x() as f64 / self.scale,
                        e.offset_y() as f64 / self.scale,
                        self.line_width / 2.0,
                        self.palette.main,
                    );
                });

                self.previous_point = None;

                self.draw();

                false
            }
            Msg::Wheel(e) => {
                if !e.ctrl_key() {
                    return false;
                }

                e.prevent_default();

                if e.delta_y() < 0.0 {
                    self.scale *= 1.05;
                } else {
                    self.scale /= 1.05;
                }

                self.draw();
                true
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
                <div style="
                    height: 100vh;
                    width: 100vw;
                    overflow: scroll;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                ">
                    <div>
                        <canvas
                            style={format!("
                                background-color: white;
                                width: {}px;
                                height: {}px;
                                image-rendering: crisp-edges;
                                image-rendering: pixelated;
                            ", (1000.0 * self.scale) as u32, (500.0 * self.scale) as u32)}
                            ref={self.canvas_ref.clone()}
                            onmousedown={ctx.link().callback(Msg::MouseDown)}
                            onmousemove={ctx.link().callback(Msg::MouseMove)}
                            onmouseup={ctx.link().callback(Msg::MouseUp)}
                            onwheel={ctx.link().callback(Msg::Wheel)}
                        />
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            self.context = Some(VirtualContext::new(canvas, 1000, 500));
        }

        self.draw();
    }
}
