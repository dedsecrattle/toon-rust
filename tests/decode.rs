//! Tests for TOON decoding

use serde_json::json;
use toon_rust::{decode, DecodeOptions};

#[test]
fn test_decode_simple_object() {
    let toon = "name: Alice\nage: 30";
    let result = decode(toon, None).unwrap();
    assert_eq!(result["name"], "Alice");
    assert_eq!(result["age"], 30);
}

#[test]
fn test_decode_primitive_array() {
    let toon = "tags[3]: reading,gaming,coding";
    let result = decode(toon, None).unwrap();
    let tags = result["tags"].as_array().unwrap();
    assert_eq!(tags[0], "reading");
    assert_eq!(tags[1], "gaming");
    assert_eq!(tags[2], "coding");
}

#[test]
fn test_decode_tabular_array() {
    let toon = "items[2]{sku,qty,price}:\n  A1,2,9.99\n  B2,1,14.5";
    let result = decode(toon, None).unwrap();
    let items = result["items"].as_array().unwrap();
    assert_eq!(items[0]["sku"], "A1");
    assert_eq!(items[0]["qty"], 2);
    assert_eq!(items[1]["sku"], "B2");
}

#[test]
fn test_decode_list_array() {
    let toon = "items[3]:\n  - 1\n  - a: 1\n  - x";
    let result = decode(toon, None).unwrap();
    let items = result["items"].as_array().unwrap();
    assert_eq!(items[0], 1);
    assert_eq!(items[2], "x");
}

#[test]
fn test_decode_nested_object() {
    let toon = "user:\n  id: 1\n  name: Alice";
    let result = decode(toon, None).unwrap();
    assert_eq!(result["user"]["id"], 1);
    assert_eq!(result["user"]["name"], "Alice");
}

#[test]
fn test_decode_empty_array() {
    let toon = "items[0]:";
    let result = decode(toon, None).unwrap();
    let items = result["items"].as_array().unwrap();
    assert!(items.is_empty());
}

#[test]
fn test_decode_quoted_string() {
    let toon = "note: \"hello, world\"";
    let result = decode(toon, None).unwrap();
    assert_eq!(result["note"], "hello, world");
}

#[test]
fn test_decode_boolean_and_null() {
    let toon = "active: true\ninactive: false\nempty: null";
    let result = decode(toon, None).unwrap();
    assert_eq!(result["active"], true);
    assert_eq!(result["inactive"], false);
    assert!(result["empty"].is_null());
}

#[test]
fn test_decode_with_custom_indent() {
    let toon = "user:\n    id: 1\n    name: Alice";
    let options = DecodeOptions::new().indent(4);
    let result = decode(toon, Some(&options)).unwrap();
    assert_eq!(result["user"]["id"], 1);
}

#[test]
fn test_decode_with_length_marker() {
    let toon = "tags[#3]: reading,gaming,coding";
    let result = decode(toon, None).unwrap();
    let tags = result["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 3);
}

