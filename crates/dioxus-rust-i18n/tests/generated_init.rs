use std::sync::{Arc, Mutex};

use dioxus::prelude::*;

dioxus_rust_i18n::i18n!("tests/fixtures/basic-locales", fallback = ["en"]);

#[derive(Clone, Props)]
struct OutputProps {
    output: Arc<Mutex<String>>,
}

impl PartialEq for OutputProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.output, &other.output)
    }
}

fn render_output(app: fn(OutputProps) -> Element) -> String {
    let output = Arc::new(Mutex::new(String::new()));
    let mut dom = VirtualDom::new_with_props(
        app,
        OutputProps {
            output: output.clone(),
        },
    );
    let _ = dom.rebuild_to_vec();

    output.lock().unwrap().clone()
}

#[allow(non_snake_case)]
fn App(props: OutputProps) -> Element {
    let _ = use_init_i18n();
    let message = dioxus_rust_i18n::t!("welcome");
    *props.output.lock().unwrap() = message.clone();

    rsx! {
        div { "{message}" }
    }
}

#[test]
fn generated_macro_initializes_i18n_for_render_tree() {
    assert_eq!(render_output(App), "Hello");
}

#[allow(non_snake_case)]
fn OverrideApp(props: OutputProps) -> Element {
    let _ = use_init_i18n();
    let message = dioxus_rust_i18n::t!("welcome", locale = "zh-CN");
    *props.output.lock().unwrap() = message.clone();

    rsx! {
        div { "{message}" }
    }
}

#[test]
fn generated_macro_supports_locale_override_and_global_translation() {
    assert_eq!(render_output(OverrideApp), "你好");
    assert_eq!(dioxus_rust_i18n::global_locale(), "en");

    dioxus_rust_i18n::set_global_locale("zh-CN");

    assert_eq!(dioxus_rust_i18n::translate_global("welcome"), "你好");
}

#[allow(non_snake_case)]
fn FallbackApp(props: OutputProps) -> Element {
    let _ = use_init_i18n();
    let message = dioxus_rust_i18n::t!("welcome", locale = "fr");
    *props.output.lock().unwrap() = message.clone();

    rsx! {
        div { "{message}" }
    }
}

#[test]
fn generated_macro_keeps_upstream_fallback_behavior() {
    assert_eq!(render_output(FallbackApp), "Hello");
}

mod backend_override_case {
    use std::borrow::Cow;

    use dioxus::prelude::*;
    use rust_i18n::Backend;

    use super::{OutputProps, render_output};

    struct OverrideBackend;

    impl Backend for OverrideBackend {
        fn available_locales(&self) -> Vec<Cow<'_, str>> {
            vec![Cow::Borrowed("en")]
        }

        fn translate(&self, locale: &str, key: &str) -> Option<Cow<'_, str>> {
            match (locale, key) {
                ("en", "welcome") => Some(Cow::Borrowed("Howdy")),
                _ => None,
            }
        }

        fn messages_for_locale(&self, locale: &str) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
            match locale {
                "en" => Some(vec![(Cow::Borrowed("welcome"), Cow::Borrowed("Howdy"))]),
                _ => None,
            }
        }
    }

    dioxus_rust_i18n::i18n!(
        "tests/fixtures/basic-locales",
        fallback = ["en"],
        backend = OverrideBackend
    );

    #[allow(non_snake_case)]
    fn App(props: OutputProps) -> Element {
        let _ = use_init_i18n();
        let message = dioxus_rust_i18n::t!("welcome");
        *props.output.lock().unwrap() = message.clone();

        rsx! { div { "{message}" } }
    }

    #[test]
    fn generated_macro_preserves_backend_override_behavior() {
        assert_eq!(render_output(App), "Howdy");
    }
}
