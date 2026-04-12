use std::sync::{OnceLock, RwLock};

use crate::{I18nError, I18nRuntime};

#[derive(Clone, Default)]
struct GlobalState {
    locale: String,
    runtime: Option<I18nRuntime>,
}

fn global_state() -> &'static RwLock<GlobalState> {
    static STATE: OnceLock<RwLock<GlobalState>> = OnceLock::new();

    STATE.get_or_init(|| RwLock::new(GlobalState::default()))
}

pub(crate) fn install_global_runtime(runtime: I18nRuntime) {
    let mut state = global_state().write().unwrap();

    state.locale = runtime.initial_locale().to_string();
    state.runtime = Some(runtime);
}

fn unwrap_global_translation(key: &str, result: Result<String, I18nError>) -> String {
    result.unwrap_or_else(|error| panic!("Failed to translate {key} globally: {error}"))
}

fn with_global_runtime<T>(
    f: impl FnOnce(&I18nRuntime, &str) -> Result<T, I18nError>,
) -> Result<T, I18nError> {
    let state = global_state().read().unwrap();

    let Some(runtime) = state.runtime.as_ref() else {
        return Err(I18nError::GlobalRuntimeNotInitialized);
    };

    f(runtime, &state.locale)
}

/// Returns the explicit non-reactive global locale mirror.
///
/// This is useful outside a Dioxus render tree. Inside components, prefer
/// [`crate::use_i18n`] and [`crate::I18nSession::language`].
#[must_use]
pub fn global_locale() -> String {
    global_state().read().unwrap().locale.clone()
}

/// Sets the explicit non-reactive global locale mirror.
///
/// This does **not** force existing Dioxus trees to rerender. To update UI,
/// call [`crate::I18nSession::set_language`] from inside the active render
/// tree.
pub fn set_global_locale(locale: impl Into<String>) {
    global_state().write().unwrap().locale = locale.into();
}

/// Tries to translate a key using the registered global runtime.
pub fn try_translate_global(key: &str) -> Result<String, I18nError> {
    with_global_runtime(|runtime, locale| runtime.translate(locale, key))
}

/// Translates a key using the registered global runtime, panicking on failure.
#[must_use]
pub fn translate_global(key: &str) -> String {
    unwrap_global_translation(key, try_translate_global(key))
}

/// Tries to translate a key with interpolation arguments using the global
/// runtime.
pub fn try_translate_global_with_args(
    key: &str,
    args: &[(&str, String)],
) -> Result<String, I18nError> {
    with_global_runtime(|runtime, locale| runtime.translate_with_kv_args(locale, key, args))
}

/// Translates a key with interpolation arguments using the global runtime,
/// panicking on failure.
#[must_use]
pub fn translate_global_with_args(key: &str, args: &[(&str, String)]) -> String {
    unwrap_global_translation(key, try_translate_global_with_args(key, args))
}
