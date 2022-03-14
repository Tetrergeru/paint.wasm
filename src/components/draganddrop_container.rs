use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};
use yew::{html, Children, Component, Context, NodeRef, Properties};

use gloo::events::EventListener;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub left: i32,
    #[prop_or_default]
    pub top: i32,
    pub event_target: NodeRef,
}

pub enum Msg {
    Move(MouseEvent),
    Down(MouseEvent),
    Up,
}

pub struct DraganddropContainer {
    left: i32,
    top: i32,

    mouse_prev: Option<(i32, i32)>,

    _move_listener: EventListener,
    _up_listener: EventListener,
}

impl Component for DraganddropContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let document: HtmlElement = ctx.props().event_target.cast::<HtmlElement>().unwrap();

        let on_mouse_move = ctx.link().callback(Msg::Move);
        let move_listener = EventListener::new(&document, "mousemove", move |e| {
            let e = e.clone().unchecked_into::<MouseEvent>();
            on_mouse_move.emit(e);
        });

        let on_mouse_up = ctx.link().callback(|_| Msg::Up);
        let up_listener = EventListener::new(&document, "mouseup", move |e| {
            let e = e.clone().unchecked_into::<MouseEvent>();
            on_mouse_up.emit(e);
        });

        Self {
            left: ctx.props().left,
            top: ctx.props().top,
            mouse_prev: None,
            _move_listener: move_listener,
            _up_listener: up_listener,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        html! {
            <div style={format!("
                    position: absolute;
                    left: {}px;
                    top: {}px;
                    background-color: white;
                    border: 1px solid black;
                    border-radius: 5px;
                ", self.left, self.top)}>
                <div
                    style={"
                        height: 20px;
                        background-color: white;
                        border-bottom: 1px solid black;
                        border-top-right-radius: 5px;
                        border-top-left-radius: 5px;"
                    }
                    onmousedown={ctx.link().callback(Msg::Down)}
                >{ctx.props().text.clone()}</div>
                <div class="draganddrop__child-container">
                    { for ctx.props().children.iter() }
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Move(e) => match self.mouse_prev {
                None => false,
                Some((x, y)) => {
                    self.left = e.client_x() - x;
                    self.top = e.client_y() - y;
                    true
                }
            },
            Msg::Down(e) => {
                self.mouse_prev = Some((e.offset_x(), e.offset_y()));
                false
            }
            Msg::Up => {
                self.mouse_prev = None;
                false
            }
        }
    }
}
