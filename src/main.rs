pub mod color;
pub mod components;
pub mod layer_manager;
pub mod shaders;
pub mod vector;
pub mod virtual_context;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<components::app::App>();
}
