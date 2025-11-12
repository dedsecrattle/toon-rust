//! Streaming API example for TOON format
//!
//! This example demonstrates how to use the streaming API to encode and decode
//! large datasets without loading everything into memory.

use serde_json::json;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use toon_rust::{decode_stream, encode_stream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a large dataset
    let items: Vec<_> = (0..100)
        .map(|i| {
            json!({
                "id": i,
                "name": format!("Product {}", i),
                "price": (i as f64) * 1.5,
                "in_stock": i % 2 == 0
            })
        })
        .collect();
    let data = json!({
        "products": items,
        "metadata": {
            "total": 100,
            "source": "example"
        }
    });

    println!("Encoding large dataset to file using streaming...");
    
    // Encode using streaming API - writes directly to file without building string in memory
    let file = File::create("example_output.toon")?;
    let mut writer = BufWriter::new(file);
    encode_stream(&data, &mut writer, None)?;
    drop(writer); // Ensure buffer is flushed

    println!("✓ Encoded to example_output.toon");

    println!("\nDecoding from file using streaming...");
    
    // Decode using streaming API - reads incrementally without loading entire file
    let file = File::open("example_output.toon")?;
    let decoded = decode_stream(file, None)?;

    println!("✓ Decoded successfully");
    println!("\nDecoded data summary:");
    println!("  Products: {}", decoded["products"].as_array().unwrap().len());
    println!("  Metadata total: {}", decoded["metadata"]["total"]);

    // Clean up
    std::fs::remove_file("example_output.toon")?;
    println!("\n✓ Cleaned up example_output.toon");

    Ok(())
}

