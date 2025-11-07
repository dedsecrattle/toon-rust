//! Round-trip tests for TOON encoding and decoding

use serde_json::json;
use toon_rust::{decode, encode};

#[test]
fn test_roundtrip_simple_object() {
    let original = json!({
        "name": "Alice",
        "age": 30
    });
    let toon = encode(&original, None).unwrap();
    let decoded = decode(&toon, None).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_primitive_array() {
    let original = json!({
        "tags": ["reading", "gaming", "coding"]
    });
    let toon = encode(&original, None).unwrap();
    let decoded = decode(&toon, None).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_tabular_array() {
    let original = json!({
        "items": [
            {"sku": "A1", "qty": 2, "price": 9.99},
            {"sku": "B2", "qty": 1, "price": 14.5}
        ]
    });
    let toon = encode(&original, None).unwrap();
    let decoded = decode(&toon, None).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_nested_object() {
    let original = json!({
        "user": {
            "id": 1,
            "name": "Alice",
            "profile": {
                "bio": "Developer"
            }
        }
    });
    let toon = encode(&original, None).unwrap();
    let decoded = decode(&toon, None).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_mixed_types() {
    let original = json!({
        "string": "hello",
        "number": 42,
        "float": 3.14,
        "boolean": true,
        "null": null,
        "array": [1, 2, 3],
        "object": {"key": "value"}
    });
    let toon = encode(&original, None).unwrap();
    let decoded = decode(&toon, None).unwrap();
    assert_eq!(original, decoded);
}

