use std::borrow::Cow;
use std::sync::Arc;

use rust_i18n::Backend;

use crate::I18nError;

/// Immutable translation runtime used by both manual integrations and Dioxus
/// sessions.
#[derive(Clone)]
pub struct I18nRuntime {
    backend: Arc<dyn Backend>,
    initial_locale: String,
    available_locales: Vec<String>,
}

impl I18nRuntime {
    /// Builds a runtime from an arbitrary backend implementation.
    #[must_use]
    pub fn from_backend<B>(initial_locale: impl Into<String>, backend: B) -> Self
    where
        B: Backend,
    {
        let initial_locale = initial_locale.into();
        let available_locales = backend
            .available_locales()
            .into_iter()
            .map(Cow::into_owned)
            .collect();

        Self {
            backend: Arc::new(backend),
            initial_locale,
            available_locales,
        }
    }

    /// Returns the locale used when a fresh session starts.
    #[must_use]
    pub fn initial_locale(&self) -> &str {
        &self.initial_locale
    }

    /// Returns every locale visible to the runtime after backend assembly.
    #[must_use]
    pub fn available_locales(&self) -> &[String] {
        &self.available_locales
    }

    /// Translates a key for a locale using the backend's own lookup semantics.
    pub fn translate(&self, locale: &str, key: &str) -> Result<String, I18nError> {
        self.backend
            .translate(locale, key)
            .map(Cow::into_owned)
            .ok_or_else(|| I18nError::MessageIdNotFound {
                locale: locale.to_string(),
                key: key.to_string(),
            })
    }

    /// Translates a key and performs `%{name}` interpolation with positional
    /// slices.
    pub fn translate_with_args(
        &self,
        locale: &str,
        key: &str,
        patterns: &[&str],
        values: &[String],
    ) -> Result<String, I18nError> {
        let translation = self.translate(locale, key)?;

        Ok(rust_i18n::replace_patterns(&translation, patterns, values))
    }

    /// Translates a key and performs `%{name}` interpolation with key/value
    /// pairs.
    pub fn translate_with_kv_args(
        &self,
        locale: &str,
        key: &str,
        args: &[(&str, String)],
    ) -> Result<String, I18nError> {
        let patterns = args.iter().map(|(pattern, _)| *pattern).collect::<Vec<_>>();
        let values = args
            .iter()
            .map(|(_, value)| value.clone())
            .collect::<Vec<_>>();

        self.translate_with_args(locale, key, &patterns, &values)
    }
}

#[cfg(test)]
pub(crate) fn test_runtime() -> I18nRuntime {
    use rust_i18n::SimpleBackend;

    let mut backend = SimpleBackend::new();
    backend.add_translations(
        "en".into(),
        [
            ("greeting".into(), "Hello".into()),
            ("welcome".into(), "Hello, %{name}!".into()),
        ]
        .into_iter()
        .collect(),
    );
    backend.add_translations(
        "zh-CN".into(),
        [
            ("greeting".into(), "你好".into()),
            ("welcome".into(), "你好，%{name}！".into()),
        ]
        .into_iter()
        .collect(),
    );

    I18nRuntime::from_backend("en", backend)
}

#[cfg(test)]
mod tests {
    use crate::runtime::test_runtime;

    #[test]
    fn translates_from_backend() {
        let runtime = test_runtime();

        assert_eq!(runtime.translate("en", "greeting").unwrap(), "Hello");
        assert_eq!(runtime.translate("zh-CN", "greeting").unwrap(), "你好");
    }

    #[test]
    fn plain_backends_do_not_get_runtime_managed_fallbacks() {
        let runtime = test_runtime();

        assert_eq!(
            runtime.translate("zh-Hans-CN", "greeting"),
            Err(crate::I18nError::MessageIdNotFound {
                locale: String::from("zh-Hans-CN"),
                key: String::from("greeting"),
            })
        );
        assert_eq!(
            runtime.translate("fr", "greeting"),
            Err(crate::I18nError::MessageIdNotFound {
                locale: String::from("fr"),
                key: String::from("greeting"),
            })
        );
    }

    #[test]
    fn interpolates_named_arguments_with_rust_i18n_style_patterns() {
        let runtime = test_runtime();

        assert_eq!(
            runtime
                .translate_with_kv_args("en", "welcome", &[("name", String::from("Dioxus"))])
                .unwrap(),
            "Hello, Dioxus!",
        );
    }
}
