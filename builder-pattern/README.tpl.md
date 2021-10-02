# {{crate}}

[![creates.io](https://img.shields.io/crates/v/builder-pattern?logo=rust)](https://crates.io/crates/builder-pattern)
[![API Docs](https://docs.rs/builder-pattern/badge.svg?logo=docs-rs)](https://docs.rs/builder-pattern/)

[![Build Status (Windows)](https://github.com/SeokminHong/builder-pattern/actions/workflows/build_windows.yml/badge.svg)](https://github.com/SeokminHong/builder-pattern/actions/workflows/build_windows.yml)
[![Build Status (MacOS)](https://github.com/SeokminHong/builder-pattern/actions/workflows/build_macos.yml/badge.svg)](https://github.com/SeokminHong/builder-pattern/actions/workflows/build_macos.yml)
[![Build Status (Ubuntu)](https://github.com/SeokminHong/builder-pattern/actions/workflows/build_ubuntu.yml/badge.svg)](https://github.com/SeokminHong/builder-pattern/actions/workflows/build_ubuntu.yml)
[![Build Status (WebAssembly)](https://github.com/SeokminHong/builder-pattern/actions/workflows/build_wasm.yml/badge.svg)](https://github.com/SeokminHong/builder-pattern/actions/workflows/build_wasm.yml)

{{desc}}

{{1}}

## Get Started

Add `builder-pattern` to `Cargo.toml`.

```toml
# Cargo.toml
[dependencies]
builder-pattern = "0.4"
```

The crate feature `future` is enabled by default. If you don't need asynchronous features, you can disable it.

```toml
# Cargo.toml
[dependencies]
builder-pattern = { version = "0.4", default-features = false }
```

{{2-}}

## License

[MIT](../LICENSE)
