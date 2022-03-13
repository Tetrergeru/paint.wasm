mod color;
mod components;
mod shaders;
mod vector;
mod virtual_context;
mod layer_manager;
mod app;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("main");
    yew::start_app::<app::App>();
}
