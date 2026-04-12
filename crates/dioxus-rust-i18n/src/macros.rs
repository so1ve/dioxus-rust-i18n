/// Tries to translate a key inside a Dioxus render context.
///
/// This macro reads the current [`crate::I18nSession`] from context, so it is
/// only valid during Dioxus render execution.
///
/// Supported forms:
///
/// ```ignore
/// te!("welcome")
/// te!("welcome", name = "Dioxus")
/// te!("welcome", locale = "zh-CN")
/// te!("welcome", locale = "zh-CN", name = "Dioxus")
/// ```
#[macro_export]
macro_rules! te {
    ($id:expr $(,)?) => {{
        let __i18n = $crate::use_i18n();

        __i18n.try_translate($id)
    }};
    ($id:expr, locale = $locale:expr $(,)?) => {{
        let __i18n = $crate::use_i18n();

        __i18n.try_translate_in($locale, $id)
    }};
    ($id:expr, locale = $locale:expr, $($name:ident = $value:expr),+ $(,)?) => {{
        let __i18n = $crate::use_i18n();
        let __args = [$( (stringify!($name), ::std::string::ToString::to_string(&$value)) ),+];

        __i18n.try_translate_kv_args_in($locale, $id, &__args)
    }};
    ($id:expr, $($name:ident = $value:expr),+ $(,)?) => {{
        let __i18n = $crate::use_i18n();
        let __args = [$( (stringify!($name), ::std::string::ToString::to_string(&$value)) ),+];

        __i18n.try_translate_kv_args($id, &__args)
    }};
}

/// Translates a key inside a Dioxus render context, panicking on failure.
#[macro_export]
macro_rules! t {
    ($($all:tt)*) => {{
        $crate::te!($($all)*).unwrap()
    }};
}

/// Translates a key inside a Dioxus render context and falls back to the key
/// itself on failure.
#[macro_export]
macro_rules! tid {
    ($id:expr $(,)?) => {{
        $crate::te!($id).unwrap_or_else(|_| ::std::string::ToString::to_string(&$id))
    }};
    ($id:expr, locale = $locale:expr $(,)?) => {{
        $crate::te!($id, locale = $locale)
            .unwrap_or_else(|_| ::std::string::ToString::to_string(&$id))
    }};
    ($id:expr, locale = $locale:expr, $($name:ident = $value:expr),+ $(,)?) => {{
        $crate::te!($id, locale = $locale, $($name = $value),+)
            .unwrap_or_else(|_| ::std::string::ToString::to_string(&$id))
    }};
    ($id:expr, $($name:ident = $value:expr),+ $(,)?) => {{
        $crate::te!($id, $($name = $value),+)
            .unwrap_or_else(|_| ::std::string::ToString::to_string(&$id))
    }};
}

#[cfg(test)]
mod tests {
    use dioxus::dioxus_core::Runtime;
    use dioxus::prelude::*;

    use crate::I18nSession;
    use crate::runtime::test_runtime;

    #[test]
    fn t_macro_reads_context_and_interpolates_named_arguments() {
        let runtime = test_runtime();

        let mut dom = VirtualDom::new(|| rsx! { div {} });
        let _ = dom.rebuild_to_vec();

        dom.in_scope(ScopeId::APP, || {
            let i18n = I18nSession::new(runtime);
            Runtime::current().provide_context(ScopeId::APP, i18n);

            assert_eq!(crate::t!("welcome", name = "Dioxus"), "Hello, Dioxus!");
        });
    }
}
