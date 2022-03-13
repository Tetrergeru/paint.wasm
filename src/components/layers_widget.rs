use web_sys::HtmlCanvasElement;
use yew::{html, Component, Context, Html, NodeRef, Properties};

use crate::{
    color::Color,
    layer_manager::{Notification, RcLayerManager},
    vector::Rectangle,
    virtual_context::VirtualContext,
};

pub struct LayersWidget {
    canvas_refs: Vec<NodeRef>,
    contexts: Vec<VirtualContext>,
    manager: RcLayerManager,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub manager: RcLayerManager,
}

pub enum Msg {
    LayerChanged,
}

impl Component for LayersWidget {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let manager = &ctx.props().manager;

        let on_notifiication = ctx.link().callback(|m| m);

        manager.borrow_mut().subscribe(Box::new(move |n| {
            log::info!("LayersWidget receive notification {:?}", n);
            on_notifiication.emit(match n {
                Notification::Change { .. } => Msg::LayerChanged,
            });
        }));

        Self {
            canvas_refs: manager
                .borrow()
                .iter_layers()
                .map(|_| NodeRef::default())
                .collect(),
            manager: manager.clone(),
            contexts: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LayerChanged => {
                log::info!("LayersWidget update");
                let manager = self.manager.borrow();
                for (idx, ctx) in self.contexts.iter().enumerate() {
                    log::info!("onchanged flush {}", idx);
                    ctx.draw_image_bounded(
                        manager.get_layer(idx).unwrap().get_canvas(),
                        Rectangle::new(0.0, 0.0, 200.0, 100.0),
                    );
                }
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="layers__container">
                {
                    for self.manager.borrow()
                        .iter_layers().enumerate()
                        .map(|(idx, layer)| html!{
                            <canvas
                                class="layers__one-layer-canvas"
                                width=200
                                height=100
                                key={layer.get_id()}
                                ref={self.canvas_refs[idx].clone()}
                            />
                    }
                )
                }
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            log::info!("LayersWidget first_render");
            for (_i, canvas_ref) in self.canvas_refs.iter().enumerate() {
                let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();

                let context = VirtualContext::new(canvas, 200, 100);

                context.checkerboard(10, Color::new(200, 200, 200, 200), Color::WHITE);

                self.contexts.push(context);
            }
        }
    }
}
