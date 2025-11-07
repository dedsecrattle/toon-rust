# toon-rust

[![CI](https://github.com/dedsecrattle/toon-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/dedsecrattle/toon-rust/actions/workflows/ci.yml)
[![Documentation](https://github.com/dedsecrattle/toon-rust/actions/workflows/docs-simple.yml/badge.svg)](https://dedsecrattle.github.io/toon-rust/)
[![crates.io](https://img.shields.io/crates/v/toon-rust.svg)](https://crates.io/crates/toon-rust)
[![docs.rs](https://docs.rs/toon-rust/badge.svg)](https://docs.rs/toon-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

Token-Oriented Object Notation (TOON) - Rust implementation

> **TOON** is a compact, human-readable format designed to reduce token usage in Large Language Model (LLM) prompts by **30–60%** compared to JSON.

## Features

- ✅ Full TOON specification v1.4 support
- ✅ Standalone API (works with `serde_json::Value`)
- ✅ Serde-compatible API (works with any `Serialize`/`Deserialize` types)
- ✅ Rust-optimized implementation with zero-copy parsing where possible
- ✅ Customizable delimiters (comma, tab, pipe)
- ✅ Length markers and indentation options
- ✅ Strict validation mode

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toon-rust = "0.1.0"
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = "1.0"
```

## Usage

### Standalone API

```rust
use toon_rust::{encode, decode};
use serde_json::json;

let data = json!({
    "items": [
        {"sku": "A1", "qty": 2, "price": 9.99},
        {"sku": "B2", "qty": 1, "price": 14.5}
    ]
});

// Encode to TOON
let toon = encode(&data, None).unwrap();
println!("{}", toon);
// Output:
// items[2]{sku,qty,price}:
//   A1,2,9.99
//   B2,1,14.5

// Decode from TOON
let decoded = decode(&toon, None).unwrap();
assert_eq!(data, decoded);
```

### Serde API

```rust
use serde::{Serialize, Deserialize};
use toon_rust::{to_string, from_str};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Product {
    sku: String,
    qty: u32,
    price: f64,
}

let products = vec![
    Product { sku: "A1".to_string(), qty: 2, price: 9.99 },
    Product { sku: "B2".to_string(), qty: 1, price: 14.5 },
];

// Serialize to TOON
let toon = to_string(&products).unwrap();

// Deserialize from TOON
let decoded: Vec<Product> = from_str(&toon).unwrap();
assert_eq!(products, decoded);
```

### Custom Options

```rust
use toon_rust::{encode, EncodeOptions, DecodeOptions};
use toon_rust::options::Delimiter;
use serde_json::json;

let data = json!({
    "tags": ["reading", "gaming", "coding"]
});

// Encode with custom options
let options = EncodeOptions::new()
    .delimiter(Delimiter::Pipe)
    .length_marker('#')
    .indent(4);

let toon = encode(&data, Some(&options)).unwrap();
// Output: tags[#3|]: reading|gaming|coding

// Decode with custom options
let decode_options = DecodeOptions::new()
    .indent(4)
    .strict(false);

let decoded = decode(&toon, Some(&decode_options)).unwrap();
```

## TOON Format

TOON uses minimal syntax to reduce token count:

- **Objects**: Indentation-based structure (like YAML)
- **Primitive arrays**: Inline format: `tags[3]: reading,gaming,coding`
- **Tabular arrays**: Uniform objects with header: `items[2]{sku,qty,price}:`
- **List arrays**: Non-uniform arrays: `items[3]:\n  - 1\n  - a: 1\n  - x`

### Example

```toon
items[2]{sku,qty,price}:
  A1,2,9.99
  B2,1,14.5
user:
  id: 1
  name: Alice
tags[3]: reading,gaming,coding
```

## API Reference

### Standalone API

- `encode(value: &Value, options: Option<&EncodeOptions>) -> Result<String, Error>`
- `decode(input: &str, options: Option<&DecodeOptions>) -> Result<Value, Error>`

### Serde API (requires `serde` feature)

- `to_string<T: Serialize>(value: &T) -> Result<String, Error>`
- `from_str<T: DeserializeOwned>(s: &str) -> Result<T, Error>`
- `to_writer<T: Serialize, W: Write>(value: &T, writer: &mut W) -> Result<(), Error>`
- `from_reader<T: DeserializeOwned, R: Read>(reader: &mut R) -> Result<T, Error>`

### Options

**EncodeOptions:**
- `delimiter(delimiter: Delimiter)` - Set delimiter (Comma, Tab, or Pipe)
- `length_marker(marker: char)` - Set length marker (e.g., `'#'` for `[#3]`)
- `indent(indent: usize)` - Set indentation level (default: 2)

**DecodeOptions:**
- `indent(indent: usize)` - Expected indentation level (default: 2)
- `strict(strict: bool)` - Enable strict validation (default: true)

## Performance

The implementation is optimized for Rust:

- Zero-copy parsing using string slices where possible
- Efficient memory management with pre-allocated buffers
- Minimal allocations during encoding/decoding

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Security

For security vulnerabilities, please email **itsprabxxx@gmail.com** instead of opening a public issue. See [SECURITY.md](SECURITY.md) for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and version history.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

- 🐛 Found a bug? [Open an issue](https://github.com/dedsecrattle/toon-rust/issues/new?template=bug_report.md)
- 💡 Have an idea? [Suggest a feature](https://github.com/dedsecrattle/toon-rust/issues/new?template=feature_request.md)
- 📖 Want to improve docs? PRs welcome!

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## Roadmap

See [ROADMAP.md](ROADMAP.md) for planned features and future improvements.

## References

- [TOON Specification](https://github.com/toon-format/toon)
- [TOON Format Website](https://toonformat.dev)

