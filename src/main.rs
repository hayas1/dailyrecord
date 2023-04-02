use dailyrecord::component::route::Router;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("Hello, this is {}", env!("CARGO_PKG_NAME"));

    yew::Renderer::<Router>::new().render();
}
