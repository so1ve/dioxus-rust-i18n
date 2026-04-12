# dioxus-rust-i18n

[![Crates.io Version](https://img.shields.io/crates/v/dioxus-rust-i18n)](https://crates.io/crates/dioxus-rust-i18n)
[![docs.rs](https://img.shields.io/docsrs/dioxus-rust-i18n)](https://docs.rs/dioxus-rust-i18n)

Dioxus integration for [`rust-i18n`](https://github.com/longbridge/rust-i18n).

## Installation

```bash
cargo add dioxus-rust-i18n rust-i18n
```

## Quick Start

```rust,ignore
use dioxus::prelude::*;
use dioxus_rust_i18n::{i18n, t};

i18n!("locales", fallback = ["en"]);

#[component]
fn App() -> Element {
    let mut i18n = use_init_i18n();

    rsx! {
        button {
            onclick: move |_| i18n.set_language("zh-CN"),
            "Switch language"
        }
        p { { t!("welcome") } }
        p { { t!("hello_user", name = "Dioxus") } }
    }
}
```

## Documentation

See [docs.rs](https://docs.rs/dioxus-rust-i18n) for full API documentation.

## Crates

| Crate | Description |
|-------|-------------|
| [`dioxus-rust-i18n`](https://crates.io/crates/dioxus-rust-i18n) | Main crate for Dioxus apps |
| [`dioxus-rust-i18n-macro`](https://crates.io/crates/dioxus-rust-i18n-macro) | Procedural macros used by the main crate |

## License

[MIT](../../LICENSE). Made with ❤️ by [Ray](https://github.com/so1ve)
