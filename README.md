# derive_static_str

A proc macro for composing static strings from enum variants. Sometimes you just want a `&'static str` with some prefix or suffix...

[Crates.io](https://crates.io/crates/derive_static_str)

[Documentation](https://docs.rs/derive_static_str)

## Usage

Add `derive_static_str` and `strum` to your `Cargo.toml`:

```toml
[dependencies]
derive_static_str = "0.1.0"
strum = { version = "0.24", features = ["derive"] }
```

Both prefixes and suffixes can be added via:

```rust
#[static_str(prefix = "...")]
#[static_str(suffix = "...")]
```

The overall case of the output can be controlled with `strum`'s `serialize_all`.

```rust
use derive_static_str::{static_str, DeriveStaticStr};
use strum::EnumString;

#[derive(EnumString, DeriveStaticStr)]
#[static_str(prefix = "my_prefix_", suffix = "_suffix")]
#[strum(serialize_all = "snake_case")]
enum MyEnum {
    VariantOne,
    VariantTwo,
}

fn main() {
    assert_eq!(MyEnum::VariantOne.as_static_str(), "my_prefix_variant_one_suffix");
    assert_eq!(MyEnum::VariantTwo.as_static_str(), "my_prefix_variant_two_suffix");
}
```
