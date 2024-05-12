use dioxus::prelude::*;
use dioxus_web_component::web_component;

fn main() {
}

#[web_component]
fn MyWebComponent(
    #[attribute(value= "name")]
    attr: String,
) -> Element {
    None
}
