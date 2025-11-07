//! Advanced options usage example

use serde_json::json;
use toon_rust::{decode, encode, DecodeOptions, EncodeOptions};
use toon_rust::options::Delimiter;

fn main() {
    let data = json!({
        "tags": ["reading", "gaming", "coding"],
        "items": [
            {"sku": "A1", "qty": 2, "price": 9.99},
            {"sku": "B2", "qty": 1, "price": 14.5}
        ]
    });

    // Encode with custom delimiter (pipe)
    println!("Encoding with pipe delimiter:\n");
    let pipe_options = EncodeOptions::new().delimiter(Delimiter::Pipe);
    let toon_pipe = encode(&data, Some(&pipe_options)).unwrap();
    println!("{}", toon_pipe);

    // Encode with length marker
    println!("\nEncoding with length marker:\n");
    let marker_options = EncodeOptions::new().length_marker('#');
    let toon_marker = encode(&data, Some(&marker_options)).unwrap();
    println!("{}", toon_marker);

    // Encode with custom indentation
    println!("\nEncoding with 4-space indentation:\n");
    let indent_options = EncodeOptions::new().indent(4);
    let toon_indent = encode(&data, Some(&indent_options)).unwrap();
    println!("{}", toon_indent);

    // Decode with custom options
    println!("\nDecoding with custom indentation:\n");
    let decode_options = DecodeOptions::new().indent(4).strict(true);
    let decoded = decode(&toon_indent, Some(&decode_options)).unwrap();
    println!("Decoded: {}", decoded);
}

