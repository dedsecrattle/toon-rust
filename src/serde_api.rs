//! Serde-compatible API for TOON encoding and decoding

use crate::decode::decode;
use crate::encode::encode;
use crate::error::Error;
use crate::options::{DecodeOptions, EncodeOptions};
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Read, Write};

/// Serialize a value to a TOON-formatted string
///
/// # Arguments
///
/// * `value` - The value to serialize (must implement `Serialize`)
///
/// # Returns
///
/// A `Result` containing the TOON-formatted string or an error
///
/// # Example
///
/// ```rust,no_run
/// use serde::Serialize;
/// use toon_rust::to_string;
///
/// #[derive(Serialize)]
/// struct Product {
///     sku: String,
///     qty: u32,
/// }
///
/// let product = Product { sku: "A1".to_string(), qty: 2 };
/// let toon = to_string(&product).unwrap();
/// ```
pub fn to_string<T: Serialize>(value: &T) -> Result<String, Error> {
    let json_value = serde_json::to_value(value)
        .map_err(|e| Error::Serialization(e.to_string()))?;
    encode(&json_value, None)
}

/// Serialize a value to a TOON-formatted string with options
///
/// # Arguments
///
/// * `value` - The value to serialize (must implement `Serialize`)
/// * `options` - Encoding options
///
/// # Returns
///
/// A `Result` containing the TOON-formatted string or an error
pub fn to_string_with_options<T: Serialize>(
    value: &T,
    options: &EncodeOptions,
) -> Result<String, Error> {
    let json_value = serde_json::to_value(value)
        .map_err(|e| Error::Serialization(e.to_string()))?;
    encode(&json_value, Some(options))
}

/// Serialize a value to a writer in TOON format
///
/// # Arguments
///
/// * `value` - The value to serialize (must implement `Serialize`)
/// * `writer` - The writer to write to
///
/// # Returns
///
/// A `Result` indicating success or failure
pub fn to_writer<T: Serialize, W: Write>(value: &T, writer: &mut W) -> Result<(), Error> {
    let toon = to_string(value)?;
    writer
        .write_all(toon.as_bytes())
        .map_err(|e| Error::Io(e.to_string()))?;
    Ok(())
}

/// Serialize a value to a writer in TOON format with options
///
/// # Arguments
///
/// * `value` - The value to serialize (must implement `Serialize`)
/// * `writer` - The writer to write to
/// * `options` - Encoding options
///
/// # Returns
///
/// A `Result` indicating success or failure
pub fn to_writer_with_options<T: Serialize, W: Write>(
    value: &T,
    writer: &mut W,
    options: &EncodeOptions,
) -> Result<(), Error> {
    let toon = to_string_with_options(value, options)?;
    writer
        .write_all(toon.as_bytes())
        .map_err(|e| Error::Io(e.to_string()))?;
    Ok(())
}

/// Deserialize a TOON-formatted string to a value
///
/// # Arguments
///
/// * `s` - The TOON-formatted string to deserialize
///
/// # Returns
///
/// A `Result` containing the deserialized value or an error
///
/// # Example
///
/// ```rust,no_run
/// use serde::Deserialize;
/// use toon_rust::from_str;
///
/// #[derive(Deserialize)]
/// struct Product {
///     sku: String,
///     qty: u32,
/// }
///
/// let toon = "sku: A1\nqty: 2";
/// let product: Product = from_str(toon).unwrap();
/// ```
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Error> {
    from_str_with_options(s, None)
}

/// Deserialize a TOON-formatted string to a value with options
///
/// # Arguments
///
/// * `s` - The TOON-formatted string to deserialize
/// * `options` - Decoding options
///
/// # Returns
///
/// A `Result` containing the deserialized value or an error
pub fn from_str_with_options<T: DeserializeOwned>(
    s: &str,
    options: Option<&DecodeOptions>,
) -> Result<T, Error> {
    let json_value = decode(s, options)?;
    serde_json::from_value(json_value)
        .map_err(|e| Error::Deserialization(e.to_string()))
}

/// Deserialize a TOON-formatted reader to a value
///
/// # Arguments
///
/// * `reader` - The reader to read from
///
/// # Returns
///
/// A `Result` containing the deserialized value or an error
pub fn from_reader<T: DeserializeOwned, R: Read>(reader: &mut R) -> Result<T, Error> {
    let mut s = String::new();
    reader
        .read_to_string(&mut s)
        .map_err(|e| Error::Io(e.to_string()))?;
    from_str(&s)
}

/// Deserialize a TOON-formatted reader to a value with options
///
/// # Arguments
///
/// * `reader` - The reader to read from
/// * `options` - Decoding options
///
/// # Returns
///
/// A `Result` containing the deserialized value or an error
pub fn from_reader_with_options<T: DeserializeOwned, R: Read>(
    reader: &mut R,
    options: &DecodeOptions,
) -> Result<T, Error> {
    let mut s = String::new();
    reader
        .read_to_string(&mut s)
        .map_err(|e| Error::Io(e.to_string()))?;
    from_str_with_options(&s, Some(options))
}

