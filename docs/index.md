# toon-rust Documentation

Welcome to the documentation for `toon-rust` - a Rust implementation of Token-Oriented Object Notation (TOON).

## What is TOON?

TOON is a compact, human-readable format designed to reduce token usage in Large Language Model (LLM) prompts by 30–60% compared to JSON. It achieves this through minimal syntax, indentation-based structure, and tabular arrays.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
toon-rust = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage Examples

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

## API Reference

See the [full API documentation](https://dedsecrattle.github.io/toon-rust/toon_rust/) generated from rustdoc.

## Format Examples

### Objects
```toon
name: Alice
age: 30
```

### Primitive Arrays
```toon
tags[3]: reading,gaming,coding
```

### Tabular Arrays
```toon
items[2]{sku,qty,price}:
  A1,2,9.99
  B2,1,14.5
```

### Nested Objects
```toon
user:
  id: 1
  name: Alice
```

## Resources

- [GitHub Repository](https://github.com/dedsecrattle/toon-rust)
- [Documentation Website](https://dedsecrattle.github.io/toon-rust/)
- [crates.io](https://crates.io/crates/toon-rust)
- [TOON Specification](https://github.com/toon-format/toon)
- [TOON Format Website](https://toonformat.dev)

## License

MIT License - see [LICENSE](https://github.com/dedsecrattle/toon-rust/blob/main/LICENSE) file for details.

