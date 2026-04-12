#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod error;
mod global;
mod hooks;
mod macros;
mod runtime;

pub use dioxus_rust_i18n_macro::i18n;
pub use error::I18nError;
pub use global::{
    global_locale, set_global_locale, translate_global, translate_global_with_args,
    try_translate_global, try_translate_global_with_args,
};
pub use hooks::{I18nSession, use_i18n, use_init_i18n_from};
pub use runtime::I18nRuntime;
