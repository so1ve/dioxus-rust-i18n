dioxus_rust_i18n::i18n!("tests/fixtures/v2-locales", fallback = ["en"]);

#[test]
fn wrapped_rust_i18n_macro_supports_format_v2_resources() {
    let runtime = __dioxus_rust_i18n_generated::compiled_runtime();

    assert_eq!(runtime.translate("en", "welcome").unwrap(), "Hello from v2");
    assert_eq!(
        runtime.translate("zh-CN", "welcome").unwrap(),
        "来自 v2 的你好"
    );
    assert_eq!(
        runtime
            .translate_with_kv_args("zh-CN", "welcome_user", &[("name", String::from("Dioxus"))])
            .unwrap(),
        "你好，Dioxus！"
    );
}
