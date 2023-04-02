use dailyrecord::component::route::Router;

fn main() {
    gloo::console::log!(format!("hello, this is {} (version {})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));

    yew::Renderer::<Router>::new().render();
}
