use thiserror::Error;

/// Errors produced by low-level translation APIs.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum I18nError {
    /// A key was not found after backend lookup finished.
    #[error("message id `{key}` was not found for locale `{locale}`")]
    MessageIdNotFound {
        /// Locale requested by the caller.
        locale: String,
        /// Translation key that was not found.
        key: String,
    },

    /// A global helper was called before any global runtime had been
    /// registered.
    #[error("global i18n runtime is not initialized")]
    GlobalRuntimeNotInitialized,
}
