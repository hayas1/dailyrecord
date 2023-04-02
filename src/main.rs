use dailyrecord::component::route::Router;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("Hello, this is {} (version {})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    yew::Renderer::<Router>::new().render();
}
