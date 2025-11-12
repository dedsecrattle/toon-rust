//! Tests for TOON streaming API

use serde_json::json;
use std::io::{BufWriter, Cursor};
use toon_rust::{decode, decode_stream, encode, encode_stream, DecodeOptions, EncodeOptions};

#[test]
fn test_encode_stream_simple_object() {
    let data = json!({
        "name": "Alice",
        "age": 30
    });

    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    // Compare with non-streaming version
    let expected = encode(&data, None).unwrap();
    assert_eq!(output, expected);
    assert!(output.contains("name: Alice"));
    assert!(output.contains("age: 30"));
}

#[test]
fn test_decode_stream_simple_object() {
    let toon = "name: Alice\nage: 30";
    let mut cursor = Cursor::new(toon.as_bytes());
    let result = decode_stream(&mut cursor, None).unwrap();

    // Compare with non-streaming version
    let expected = decode(toon, None).unwrap();
    assert_eq!(result, expected);
    assert_eq!(result["name"], "Alice");
    assert_eq!(result["age"], 30);
}

#[test]
fn test_encode_stream_primitive_array() {
    let data = json!({
        "tags": ["reading", "gaming", "coding"]
    });

    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let expected = encode(&data, None).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_decode_stream_primitive_array() {
    let toon = "tags[3]: reading,gaming,coding";
    let mut cursor = Cursor::new(toon.as_bytes());
    let result = decode_stream(&mut cursor, None).unwrap();

    let expected = decode(toon, None).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_encode_stream_tabular_array() {
    let data = json!({
        "items": [
            {"sku": "A1", "qty": 2, "price": 9.99},
            {"sku": "B2", "qty": 1, "price": 14.5}
        ]
    });

    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let expected = encode(&data, None).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_decode_stream_tabular_array() {
    let toon = "items[2]{sku,qty,price}:\n  A1,2,9.99\n  B2,1,14.5";
    let mut cursor = Cursor::new(toon.as_bytes());
    let result = decode_stream(&mut cursor, None).unwrap();

    let expected = decode(toon, None).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_encode_stream_nested_object() {
    let data = json!({
        "user": {
            "id": 1,
            "name": "Alice"
        }
    });

    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let expected = encode(&data, None).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_decode_stream_nested_object() {
    let toon = "user:\n  id: 1\n  name: Alice";
    let mut cursor = Cursor::new(toon.as_bytes());
    let result = decode_stream(&mut cursor, None).unwrap();

    let expected = decode(toon, None).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_streaming_roundtrip() {
    let data = json!({
        "name": "Alice",
        "age": 30,
        "tags": ["reading", "gaming"],
        "items": [
            {"sku": "A1", "qty": 2, "price": 9.99}
        ]
    });

    // Encode using streaming
    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();

    // Decode using streaming
    let mut cursor = Cursor::new(&buffer);
    let decoded = decode_stream(&mut cursor, None).unwrap();

    assert_eq!(data, decoded);
}

#[test]
fn test_streaming_roundtrip_large_array() {
    // Create a large array of objects
    let items: Vec<_> = (0..1000)
        .map(|i| {
            json!({
                "id": i,
                "name": format!("Item {}", i),
                "value": i * 2
            })
        })
        .collect();
    let data = json!({ "items": items });

    // Encode using streaming
    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();

    // Decode using streaming
    let mut cursor = Cursor::new(&buffer);
    let decoded = decode_stream(&mut cursor, None).unwrap();

    assert_eq!(data, decoded);
}

#[test]
fn test_encode_stream_with_options() {
    let data = json!({
        "items": [
            {"sku": "A1", "qty": 2},
            {"sku": "B2", "qty": 1}
        ]
    });

    let options = EncodeOptions::new().delimiter(toon_rust::options::Delimiter::Pipe);
    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, Some(&options)).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let expected = encode(&data, Some(&options)).unwrap();
    assert_eq!(output, expected);
    assert!(output.contains("|"));
}

#[test]
fn test_decode_stream_with_options() {
    let toon = "user:\n    id: 1\n    name: Alice";
    let options = DecodeOptions::new().indent(4);
    let mut cursor = Cursor::new(toon.as_bytes());
    let result = decode_stream(&mut cursor, Some(&options)).unwrap();

    let expected = decode(toon, Some(&options)).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_encode_stream_to_bufwriter() {
    let data = json!({"test": "value"});
    let mut buffer = Vec::new();
    let mut writer = BufWriter::new(&mut buffer);
    encode_stream(&data, &mut writer, None).unwrap();
    drop(writer); // Ensure buffer is flushed

    let output = String::from_utf8(buffer).unwrap();
    let expected = encode(&data, None).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_streaming_empty_array() {
    let data = json!({"items": []});
    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let mut cursor = Cursor::new(output.as_bytes());
    let decoded = decode_stream(&mut cursor, None).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_streaming_empty_object() {
    let data = json!({});
    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let mut cursor = Cursor::new(output.as_bytes());
    let decoded = decode_stream(&mut cursor, None).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_streaming_mixed_types() {
    let data = json!({
        "string": "hello",
        "number": 42,
        "float": 3.14,
        "boolean": true,
        "null": null,
        "array": [1, 2, 3],
        "object": {"key": "value"}
    });

    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();

    let mut cursor = Cursor::new(&buffer);
    let decoded = decode_stream(&mut cursor, None).unwrap();

    assert_eq!(data, decoded);
}

#[test]
fn test_streaming_root_array() {
    let data = json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]);

    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();

    let mut cursor = Cursor::new(&buffer);
    let decoded = decode_stream(&mut cursor, None).unwrap();

    assert_eq!(data, decoded);
}

#[test]
fn test_streaming_quoted_strings() {
    let data = json!({
        "note": "hello, world",
        "quote": "She said \"hello\""
    });

    let mut buffer = Vec::new();
    encode_stream(&data, &mut buffer, None).unwrap();

    let mut cursor = Cursor::new(&buffer);
    let decoded = decode_stream(&mut cursor, None).unwrap();

    assert_eq!(data, decoded);
}

// Test that streaming produces identical output to non-streaming
#[test]
fn test_streaming_output_identical() {
    let test_cases = vec![
        json!({"simple": "value"}),
        json!({"array": [1, 2, 3]}),
        json!({"nested": {"key": "value"}}),
        json!({"mixed": [1, "two", true, null]}),
        json!({
            "items": [
                {"sku": "A1", "qty": 2, "price": 9.99},
                {"sku": "B2", "qty": 1, "price": 14.5}
            ]
        }),
    ];

    for data in test_cases {
        // Non-streaming
        let non_streaming = encode(&data, None).unwrap();

        // Streaming
        let mut buffer = Vec::new();
        encode_stream(&data, &mut buffer, None).unwrap();
        let streaming = String::from_utf8(buffer).unwrap();

        assert_eq!(
            non_streaming, streaming,
            "Output mismatch for data: {:?}",
            data
        );
    }
}
