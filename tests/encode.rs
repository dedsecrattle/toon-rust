//! Tests for TOON encoding

use serde_json::json;
use toon_rust::{encode, EncodeOptions};

#[test]
fn test_encode_simple_object() {
    let data = json!({
        "name": "Alice",
        "age": 30
    });
    let result = encode(&data, None).unwrap();
    assert!(result.contains("name: Alice"));
    assert!(result.contains("age: 30"));
}

#[test]
fn test_encode_primitive_array() {
    let data = json!({
        "tags": ["reading", "gaming", "coding"]
    });
    let result = encode(&data, None).unwrap();
    assert!(result.contains("tags[3]:"));
    assert!(result.contains("reading"));
    assert!(result.contains("gaming"));
    assert!(result.contains("coding"));
}

#[test]
fn test_encode_tabular_array() {
    let data = json!({
        "items": [
            {"sku": "A1", "qty": 2, "price": 9.99},
            {"sku": "B2", "qty": 1, "price": 14.5}
        ]
    });
    let result = encode(&data, None).unwrap();
    assert!(result.contains("items[2]{"));
    assert!(result.contains("sku,qty,price"));
    assert!(result.contains("A1,2,9.99"));
    assert!(result.contains("B2,1,14.5"));
}

#[test]
fn test_encode_list_array() {
    let data = json!({
        "items": [1, {"a": 1}, "x"]
    });
    let result = encode(&data, None).unwrap();
    assert!(result.contains("items[3]:"));
    assert!(result.contains("- 1"));
    assert!(result.contains("- a: 1"));
    assert!(result.contains("- x"));
}

#[test]
fn test_encode_nested_object() {
    let data = json!({
        "user": {
            "id": 1,
            "name": "Alice"
        }
    });
    let result = encode(&data, None).unwrap();
    assert!(result.contains("user:"));
    assert!(result.contains("id: 1"));
    assert!(result.contains("name: Alice"));
}

#[test]
fn test_encode_empty_array() {
    let data = json!({
        "items": []
    });
    let result = encode(&data, None).unwrap();
    assert!(result.contains("items[0]:"));
}

#[test]
fn test_encode_with_custom_delimiter() {
    let data = json!({
        "items": [
            {"sku": "A1", "qty": 2},
            {"sku": "B2", "qty": 1}
        ]
    });
    let options = EncodeOptions::new().delimiter(toon_rust::options::Delimiter::Pipe);
    let result = encode(&data, Some(&options)).unwrap();
    assert!(result.contains("sku|qty"));
    assert!(result.contains("A1|2"));
}

#[test]
fn test_encode_with_length_marker() {
    let data = json!({
        "tags": ["reading", "gaming"]
    });
    let options = EncodeOptions::new().length_marker('#');
    let result = encode(&data, Some(&options)).unwrap();
    assert!(result.contains("tags[#2]:"));
}

#[test]
fn test_encode_string_with_comma() {
    let data = json!({
        "note": "hello, world"
    });
    let result = encode(&data, None).unwrap();
    assert!(result.contains("\"hello, world\""));
}

#[test]
fn test_encode_boolean_string_vs_boolean() {
    let data = json!({
        "items": ["true", true]
    });
    let result = encode(&data, None).unwrap();
    assert!(result.contains("\"true\""));
    assert!(result.contains(",true"));
}

