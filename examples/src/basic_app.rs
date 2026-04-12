use dioxus::dioxus_core::NoOpMutations;
use dioxus::prelude::*;
use dioxus_rust_i18n::{i18n, t};

i18n!("locales", fallback = ["en"]);

#[component]
fn App() -> Element {
    let mut i18n = use_init_i18n();

    rsx! {
        button {
            onclick: move |_| i18n.set_language("zh-CN"),
            "Switch"
        }
        p { { t!("welcome") } }
        p { { t!("hello_user", name = "Dioxus") } }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut dom = VirtualDom::new(App);
    dom.rebuild_in_place();
    dom.render_immediate(&mut NoOpMutations);
}
