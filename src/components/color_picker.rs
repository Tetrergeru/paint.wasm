use crate::{
    color::{Color, Palette},
    components::color_circle::ColorCircle,
};
use yew::{html, Component, Context, Html, Properties, Callback};

pub struct ColorPicker {
    palette: Palette,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    palette: Palette,
    #[prop_or_default]
    pub color_pick: Callback<Palette>,
}

pub enum Msg {
    ColorPick(Color),
    SwapColors,
    ToDefault,
}

impl Component for ColorPicker {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            palette: ctx.props().palette.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ColorPick(color) => {
                self.palette.main = color;
                ctx.props().color_pick.emit(self.palette.clone());
                true
            }
            Msg::SwapColors => {
                std::mem::swap(&mut self.palette.main, &mut self.palette.help);
                ctx.props().color_pick.emit(self.palette.clone());
                true
            }
            Msg::ToDefault => {
                self.palette = Palette::default();
                ctx.props().color_pick.emit(self.palette.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="display: flex">
                <div style="position: relative; width: 40px; height: 40px; margin: 5px; z-index: 0;">
                    <div style={format!("
                        position: absolute;
                        z-index: 1;
                        top: 0px;
                        left: 0px;
                        width: 25px;
                        height: 25px;
                        background-color: {};
                        border: 1px solid white;
                        outline: 1px solid black;
                    ", self.palette.main.to_style())}
                    ></div>
                    <div style={format!("
                        position: absolute;
                        z-index: 0;
                        bottom: 0px;
                        right: 0px;
                        width: 25px;
                        height: 25px;
                        background-color: {};
                        border: 1px solid white;
                        outline: 1px solid black;
                    ", self.palette.help.to_style())}
                    ></div>
                    <img style={"
                        position: absolute;
                        top: 0px;
                        right: 0px;
                        width: 10px;
                        height: 10px;
                    "}
                        src="./resources/change-color.svg"
                        onmousedown={ctx.link().callback(|_| Msg::SwapColors)}
                    />
                    <img style={"
                        position: absolute;
                        bottom: 0px;
                        left: 0px;
                        width: 10px;
                        height: 10px;
                    "}
                        src="./resources/colors_default.svg"
                        onmousedown={ctx.link().callback(|_| Msg::ToDefault)}
                    />
                </div>
                <ColorCircle color_pick={ctx.link().callback(Msg::ColorPick)}>
                </ColorCircle>
            </div>
        }
    }
}
