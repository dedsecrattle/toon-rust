//! Token-Oriented Object Notation (TOON) - Rust implementation
//!
//! TOON is a compact, human-readable format designed to reduce token usage
//! in Large Language Model (LLM) prompts by 30–60% compared to JSON.
//!
//! # Examples
//!
//! ## Standalone API
//!
//! ```rust
//! use toon_rust::{encode, decode};
//! use serde_json::json;
//!
//! let data = json!({
//!     "items": [
//!         {"sku": "A1", "qty": 2, "price": 9.99},
//!         {"sku": "B2", "qty": 1, "price": 14.5}
//!     ]
//! });
//!
//! let toon = encode(&data, None).unwrap();
//! let decoded = decode(&toon, None).unwrap();
//! ```
//!
//! ## Streaming API
//!
//! For large datasets, use the streaming API to avoid loading everything into memory:
//!
//! ```rust,no_run
//! use std::fs::File;
//! use std::io::BufWriter;
//! use serde_json::json;
//! use toon_rust::{encode_stream, decode_stream};
//!
//! // Encode to file
//! let data = json!({"name": "Alice", "age": 30});
//! let file = File::create("output.toon").unwrap();
//! let mut writer = BufWriter::new(file);
//! encode_stream(&data, &mut writer, None).unwrap();
//!
//! // Decode from file
//! let file = File::open("output.toon").unwrap();
//! let decoded = decode_stream(file, None).unwrap();
//! ```
//!
//! ## Serde API (requires `serde` feature)
//!
//! ```rust,no_run
//! use serde::{Serialize, Deserialize};
//! use toon_rust::{to_string, from_str};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Product {
//!     sku: String,
//!     qty: u32,
//!     price: f64,
//! }
//!
//! let products = vec![
//!     Product { sku: "A1".to_string(), qty: 2, price: 9.99 },
//!     Product { sku: "B2".to_string(), qty: 1, price: 14.5 },
//! ];
//!
//! let toon = to_string(&products).unwrap();
//! let decoded: Vec<Product> = from_str(&toon).unwrap();
//! ```

pub mod decode;
pub mod encode;
pub mod error;
pub mod options;
mod simd;

pub use decode::{decode, decode_stream};
pub use encode::{encode, encode_stream};
pub use error::Error;
pub use options::{DecodeOptions, EncodeOptions};

#[cfg(feature = "serde")]
pub mod serde_api;

#[cfg(feature = "serde")]
pub use serde_api::{from_reader, from_str, to_string, to_writer};
