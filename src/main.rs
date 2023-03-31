use dailyrecord::component::app::App;

fn main() {
    // https://yew.rs/ja/docs/migration-guides/yew/from-0_19_0-to-0_20_0#yew-renderer
    yew::Renderer::<App>::new().render();
}
