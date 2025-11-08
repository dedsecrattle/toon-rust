//! Basic standalone API usage example

use serde_json::json;
use toon_rust::{decode, encode};

fn main() {
    // Create some data
    let data = json!({
        "name": "Alice",
        "age": 30,
        "tags": ["reading", "gaming", "coding"],
        "items": [
            {"sku": "A1", "qty": 2, "price": 9.99},
            {"sku": "B2", "qty": 1, "price": 14.5}
        ]
    });

    // Encode to TOON
    println!("Encoding to TOON format:\n");
    let toon = encode(&data, None).unwrap();
    println!("{toon}");

    // Decode from TOON
    println!("\nDecoding from TOON format:\n");
    let decoded = decode(&toon, None).unwrap();
    println!("Decoded: {decoded}");
}
