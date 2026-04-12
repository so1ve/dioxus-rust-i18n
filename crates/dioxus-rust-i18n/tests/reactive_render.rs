use std::sync::{Arc, Mutex};

use dioxus::dioxus_core::NoOpMutations;
use dioxus::prelude::*;
use futures_util::FutureExt;

dioxus_rust_i18n::i18n!("tests/fixtures/basic-locales", fallback = ["en"]);

#[derive(Clone, Props)]
struct ReactiveProps {
    renders: Arc<Mutex<Vec<String>>>,
}

impl PartialEq for ReactiveProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.renders, &other.renders)
    }
}

fn reactive_app(props: ReactiveProps) -> Element {
    let mut i18n = use_init_i18n();
    let mut did_switch = use_signal(|| false);
    let message = dioxus_rust_i18n::t!("welcome");

    props.renders.lock().unwrap().push(message.clone());

    use_effect(move || {
        if !did_switch() {
            did_switch.set(true);
            i18n.set_language("zh-CN");
        }
    });

    rsx! {
        div { "{message}" }
    }
}

#[test]
fn locale_changes_trigger_component_rerender() {
    let renders = Arc::new(Mutex::new(Vec::new()));

    let mut dom = VirtualDom::new_with_props(
        reactive_app,
        ReactiveProps {
            renders: renders.clone(),
        },
    );

    dom.rebuild_in_place();
    dom.render_immediate(&mut NoOpMutations);

    while dom.wait_for_work().now_or_never().is_some() {
        dom.render_immediate(&mut NoOpMutations);
    }

    assert_eq!(
        renders.lock().unwrap().as_slice(),
        [String::from("Hello"), String::from("你好")]
    );
}
