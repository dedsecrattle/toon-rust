//! Encoding TOON format from JSON values

use crate::error::Error;
use crate::options::EncodeOptions;
use serde_json::Value;
use std::io::Write;

/// Encode a JSON value to TOON format
///
/// # Arguments
///
/// * `value` - The JSON value to encode
/// * `options` - Optional encoding options
///
/// # Returns
///
/// A `Result` containing the TOON-formatted string or an error
pub fn encode(value: &Value, options: Option<&EncodeOptions>) -> Result<String, Error> {
    let default_opts = EncodeOptions::default();
    let opts = options.unwrap_or(&default_opts);
    let mut output = String::new();
    encode_value(value, &mut output, 0, opts)?;
    Ok(output)
}

fn encode_value(
    value: &Value,
    output: &mut String,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    match value {
        Value::Null => {
            // Null values are typically omitted or represented as empty
        }
        Value::Bool(b) => {
            output.push_str(if *b { "true" } else { "false" });
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                output.push_str(&i.to_string());
            } else if let Some(f) = n.as_f64() {
                output.push_str(&f.to_string());
            } else {
                return Err(Error::Serialization("Invalid number".to_string()));
            }
        }
        Value::String(s) => {
            encode_string(s, output, options.get_delimiter());
        }
        Value::Array(arr) => {
            encode_array(arr, output, indent_level, options)?;
        }
        Value::Object(obj) => {
            encode_object(obj, output, indent_level, options)?;
        }
    }
    Ok(())
}

fn encode_string(s: &str, output: &mut String, delimiter: char) {
    // Check if we need to quote the string
    let needs_quoting = s.contains(delimiter)
        || s.contains(' ')
        || s.contains('\n')
        || s.contains('\t')
        || s == "true"
        || s == "false"
        || s == "null"
        || s.parse::<f64>().is_ok();

    if needs_quoting {
        output.push('"');
        for ch in s.chars() {
            match ch {
                '"' => output.push_str("\\\""),
                '\\' => output.push_str("\\\\"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                _ => output.push(ch),
            }
        }
        output.push('"');
    } else {
        output.push_str(s);
    }
}

fn encode_array(
    arr: &[Value],
    output: &mut String,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    if arr.is_empty() {
        output.push_str("[0]:");
        return Ok(());
    }

    // Check if array contains uniform objects (tabular format)
    if let Some(keys) = check_uniform_objects(arr) {
        // For root-level arrays, include the header
        let length_marker = options
            .length_marker
            .map(|m| format!("{m}"))
            .unwrap_or_default();
        output.push_str(&format!("[{}{}]", length_marker, arr.len()));
        output.push('{');
        output.push_str(&keys.join(&options.get_delimiter().to_string()));
        output.push_str("}:\n");
        encode_tabular_array_rows(arr, keys, output, indent_level, options)?;
        return Ok(());
    }

    // Check if all elements are primitives (inline format)
    if arr.iter().all(is_primitive) {
        encode_inline_array(arr, output, options)?;
        return Ok(());
    }

    // Otherwise, use list format
    encode_list_array(arr, output, indent_level, options)?;
    Ok(())
}

fn is_primitive(value: &Value) -> bool {
    matches!(
        value,
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_)
    )
}

fn check_uniform_objects(arr: &[Value]) -> Option<Vec<String>> {
    if arr.is_empty() {
        return None;
    }

    // Get keys from first object (preserve order)
    let first = arr[0].as_object()?;
    let keys: Vec<String> = first.keys().cloned().collect();
    if keys.is_empty() {
        return None;
    }

    // Check if all objects have the same keys (order doesn't matter for this check)
    for item in arr.iter().skip(1) {
        let obj = item.as_object()?;
        let item_keys: std::collections::HashSet<String> = obj.keys().cloned().collect();
        let first_keys: std::collections::HashSet<String> = keys.iter().cloned().collect();
        if item_keys != first_keys {
            return None;
        }
    }

    Some(keys)
}

fn encode_tabular_array_rows(
    arr: &[Value],
    keys: Vec<String>,
    output: &mut String,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    let indent = options.get_indent();
    let indent_str = " ".repeat(indent_level * indent);
    let delimiter = options.get_delimiter();

    // Write rows (header already written by caller)
    for item in arr {
        output.push_str(&indent_str);
        output.push_str(&" ".repeat(indent));
        let obj = item
            .as_object()
            .ok_or_else(|| Error::Serialization("Expected object in tabular array".to_string()))?;

        let mut first = true;
        for key in &keys {
            if !first {
                output.push(delimiter);
            }
            let value = obj
                .get(key)
                .ok_or_else(|| Error::Serialization(format!("Missing key: {key}")))?;
            encode_primitive_value(value, output, delimiter)?;
            first = false;
        }
        output.push('\n');
    }

    Ok(())
}

fn encode_primitive_value(
    value: &Value,
    output: &mut String,
    delimiter: char,
) -> Result<(), Error> {
    match value {
        Value::Null => {}
        Value::Bool(b) => {
            output.push_str(if *b { "true" } else { "false" });
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                output.push_str(&i.to_string());
            } else if let Some(f) = n.as_f64() {
                output.push_str(&f.to_string());
            } else {
                return Err(Error::Serialization("Invalid number".to_string()));
            }
        }
        Value::String(s) => {
            encode_string(s, output, delimiter);
        }
        _ => {
            return Err(Error::Serialization(
                "Non-primitive value in tabular array".to_string(),
            ));
        }
    }
    Ok(())
}

fn encode_inline_array(
    arr: &[Value],
    output: &mut String,
    options: &EncodeOptions,
) -> Result<(), Error> {
    let length_marker = options
        .length_marker
        .map(|m| format!("{m}"))
        .unwrap_or_default();
    output.push_str(&format!("[{}{}]:", length_marker, arr.len()));

    let delimiter = options.get_delimiter();
    let mut first = true;
    for item in arr {
        if !first {
            output.push(delimiter);
        }
        match item {
            Value::Null => {}
            Value::Bool(b) => {
                output.push_str(if *b { "true" } else { "false" });
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    output.push_str(&i.to_string());
                } else if let Some(f) = n.as_f64() {
                    output.push_str(&f.to_string());
                }
            }
            Value::String(s) => {
                encode_string(s, output, delimiter);
            }
            _ => {
                return Err(Error::Serialization(
                    "Non-primitive in inline array".to_string(),
                ));
            }
        }
        first = false;
    }

    Ok(())
}

fn encode_list_array(
    arr: &[Value],
    output: &mut String,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    let indent = options.get_indent();
    let indent_str = " ".repeat(indent_level * indent);

    for item in arr {
        output.push_str(&indent_str);
        output.push_str(&" ".repeat(indent));
        output.push_str("- ");
        // For objects in list arrays, encode them inline as key: value
        match item {
            Value::Object(obj) => {
                let mut first = true;
                for (key, val) in obj {
                    if !first {
                        output.push(' ');
                    }
                    output.push_str(key);
                    output.push_str(": ");
                    encode_primitive_value(val, output, options.get_delimiter())?;
                    first = false;
                }
            }
            _ => {
                encode_value(item, output, indent_level + 1, options)?;
            }
        }
        output.push('\n');
    }

    Ok(())
}

fn encode_object(
    obj: &serde_json::Map<String, Value>,
    output: &mut String,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    if obj.is_empty() {
        return Ok(());
    }

    let indent = options.get_indent();
    let indent_str = " ".repeat(indent_level * indent);

    let mut first = true;
    for (key, value) in obj {
        if !first {
            output.push('\n');
        }
        output.push_str(&indent_str);
        output.push_str(key);

        match value {
            Value::Array(arr) => {
                // For arrays, check the format and encode appropriately
                if arr.is_empty() {
                    output.push_str("[0]:");
                } else if let Some(keys) = check_uniform_objects(arr) {
                    // Tabular array - output on same line: key[N]{...}:
                    let length_marker = options
                        .length_marker
                        .map(|m| format!("{m}"))
                        .unwrap_or_default();
                    output.push_str(&format!("[{}{}]", length_marker, arr.len()));
                    output.push('{');
                    output.push_str(&keys.join(&options.get_delimiter().to_string()));
                    output.push_str("}:\n");
                    // Now output the rows
                    encode_tabular_array_rows(arr, keys, output, indent_level, options)?;
                } else if arr.iter().all(is_primitive) {
                    // Inline array - output on same line: key[N]: value1,value2
                    let length_marker = options
                        .length_marker
                        .map(|m| format!("{m}"))
                        .unwrap_or_default();
                    output.push_str(&format!("[{}{}]:", length_marker, arr.len()));
                    let delimiter = options.get_delimiter();
                    let mut first = true;
                    for item in arr {
                        if !first {
                            output.push(delimiter);
                        }
                        encode_primitive_value(item, output, delimiter)?;
                        first = false;
                    }
                } else {
                    // List array - output on same line: key[N]:
                    let length_marker = options
                        .length_marker
                        .map(|m| format!("{m}"))
                        .unwrap_or_default();
                    output.push_str(&format!("[{}{}]:", length_marker, arr.len()));
                    output.push('\n');
                    encode_list_array(arr, output, indent_level, options)?;
                }
            }
            Value::Object(_) => {
                output.push_str(": ");
                output.push('\n');
                encode_value(value, output, indent_level + 1, options)?;
            }
            _ => {
                output.push_str(": ");
                encode_value(value, output, indent_level, options)?;
            }
        }
        first = false;
    }

    Ok(())
}

/// Encode a JSON value to TOON format and write it to a writer
///
/// This function streams the output directly to the writer without building
/// the entire string in memory, making it suitable for large datasets.
///
/// # Arguments
///
/// * `value` - The JSON value to encode
/// * `writer` - The writer to write the TOON-formatted output to
/// * `options` - Optional encoding options
///
/// # Returns
///
/// A `Result` indicating success or failure
///
/// # Example
///
/// ```rust,no_run
/// use std::fs::File;
/// use std::io::BufWriter;
/// use serde_json::json;
/// use toon_rust::encode_stream;
///
/// let data = json!({"name": "Alice", "age": 30});
/// let file = File::create("output.toon").unwrap();
/// let mut writer = BufWriter::new(file);
/// encode_stream(&data, &mut writer, None).unwrap();
/// ```
pub fn encode_stream<W: Write>(
    value: &Value,
    writer: &mut W,
    options: Option<&EncodeOptions>,
) -> Result<(), Error> {
    let default_opts = EncodeOptions::default();
    let opts = options.unwrap_or(&default_opts);
    encode_value_to_writer(value, writer, 0, opts)?;
    writer.flush().map_err(|e| Error::Io(e.to_string()))?;
    Ok(())
}

fn encode_value_to_writer<W: Write>(
    value: &Value,
    writer: &mut W,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    match value {
        Value::Null => {
            // Null values are typically omitted or represented as empty
        }
        Value::Bool(b) => {
            let s = if *b { "true" } else { "false" };
            writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                let s = i.to_string();
                writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
            } else if let Some(f) = n.as_f64() {
                let s = f.to_string();
                writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
            } else {
                return Err(Error::Serialization("Invalid number".to_string()));
            }
        }
        Value::String(s) => {
            encode_string_to_writer(s, writer, options.get_delimiter())?;
        }
        Value::Array(arr) => {
            encode_array_to_writer(arr, writer, indent_level, options)?;
        }
        Value::Object(obj) => {
            encode_object_to_writer(obj, writer, indent_level, options)?;
        }
    }
    Ok(())
}

fn encode_string_to_writer<W: Write>(
    s: &str,
    writer: &mut W,
    delimiter: char,
) -> Result<(), Error> {
    // Check if we need to quote the string
    let needs_quoting = s.contains(delimiter)
        || s.contains(' ')
        || s.contains('\n')
        || s.contains('\t')
        || s == "true"
        || s == "false"
        || s == "null"
        || s.parse::<f64>().is_ok();

    if needs_quoting {
        writer.write_all(b"\"").map_err(|e| Error::Io(e.to_string()))?;
        for ch in s.chars() {
            match ch {
                '"' => writer.write_all(b"\\\"").map_err(|e| Error::Io(e.to_string()))?,
                '\\' => writer.write_all(b"\\\\").map_err(|e| Error::Io(e.to_string()))?,
                '\n' => writer.write_all(b"\\n").map_err(|e| Error::Io(e.to_string()))?,
                '\r' => writer.write_all(b"\\r").map_err(|e| Error::Io(e.to_string()))?,
                '\t' => writer.write_all(b"\\t").map_err(|e| Error::Io(e.to_string()))?,
                _ => {
                    let mut buf = [0; 4];
                    let bytes = ch.encode_utf8(&mut buf).as_bytes();
                    writer.write_all(bytes).map_err(|e| Error::Io(e.to_string()))?;
                }
            }
        }
        writer.write_all(b"\"").map_err(|e| Error::Io(e.to_string()))?;
    } else {
        writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
    }
    Ok(())
}

fn encode_array_to_writer<W: Write>(
    arr: &[Value],
    writer: &mut W,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    if arr.is_empty() {
        writer.write_all(b"[0]:").map_err(|e| Error::Io(e.to_string()))?;
        return Ok(());
    }

    // Check if array contains uniform objects (tabular format)
    if let Some(keys) = check_uniform_objects(arr) {
        // For root-level arrays, include the header
        let length_marker = options
            .length_marker
            .map(|m| format!("{m}"))
            .unwrap_or_default();
        let header = format!("[{}{}]", length_marker, arr.len());
        writer.write_all(header.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        writer.write_all(b"{").map_err(|e| Error::Io(e.to_string()))?;
        let keys_str = keys.join(&options.get_delimiter().to_string());
        writer.write_all(keys_str.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        writer.write_all(b"}:\n").map_err(|e| Error::Io(e.to_string()))?;
        encode_tabular_array_rows_to_writer(arr, keys, writer, indent_level, options)?;
        return Ok(());
    }

    // Check if all elements are primitives (inline format)
    if arr.iter().all(is_primitive) {
        encode_inline_array_to_writer(arr, writer, options)?;
        return Ok(());
    }

    // Otherwise, use list format
    encode_list_array_to_writer(arr, writer, indent_level, options)?;
    Ok(())
}

fn encode_tabular_array_rows_to_writer<W: Write>(
    arr: &[Value],
    keys: Vec<String>,
    writer: &mut W,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    let indent = options.get_indent();
    let indent_str = " ".repeat(indent_level * indent);
    let delimiter = options.get_delimiter();

    // Write rows (header already written by caller)
    for item in arr {
        writer.write_all(indent_str.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        writer.write_all(" ".repeat(indent).as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        let obj = item
            .as_object()
            .ok_or_else(|| Error::Serialization("Expected object in tabular array".to_string()))?;

        let mut first = true;
        for key in &keys {
            if !first {
                let delim_bytes = [delimiter as u8];
                writer.write_all(&delim_bytes).map_err(|e| Error::Io(e.to_string()))?;
            }
            let value = obj
                .get(key)
                .ok_or_else(|| Error::Serialization(format!("Missing key: {key}")))?;
            encode_primitive_value_to_writer(value, writer, delimiter)?;
            first = false;
        }
        writer.write_all(b"\n").map_err(|e| Error::Io(e.to_string()))?;
    }

    Ok(())
}

fn encode_primitive_value_to_writer<W: Write>(
    value: &Value,
    writer: &mut W,
    delimiter: char,
) -> Result<(), Error> {
    match value {
        Value::Null => {}
        Value::Bool(b) => {
            let s = if *b { "true" } else { "false" };
            writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                let s = i.to_string();
                writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
            } else if let Some(f) = n.as_f64() {
                let s = f.to_string();
                writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
            } else {
                return Err(Error::Serialization("Invalid number".to_string()));
            }
        }
        Value::String(s) => {
            encode_string_to_writer(s, writer, delimiter)?;
        }
        _ => {
            return Err(Error::Serialization(
                "Non-primitive value in tabular array".to_string(),
            ));
        }
    }
    Ok(())
}

fn encode_inline_array_to_writer<W: Write>(
    arr: &[Value],
    writer: &mut W,
    options: &EncodeOptions,
) -> Result<(), Error> {
    let length_marker = options
        .length_marker
        .map(|m| format!("{m}"))
        .unwrap_or_default();
    let header = format!("[{}{}]:", length_marker, arr.len());
    writer.write_all(header.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;

    let delimiter = options.get_delimiter();
    let mut first = true;
    for item in arr {
        if !first {
            let delim_bytes = [delimiter as u8];
            writer.write_all(&delim_bytes).map_err(|e| Error::Io(e.to_string()))?;
        }
        match item {
            Value::Null => {}
            Value::Bool(b) => {
                let s = if *b { "true" } else { "false" };
                writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    let s = i.to_string();
                    writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
                } else if let Some(f) = n.as_f64() {
                    let s = f.to_string();
                    writer.write_all(s.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
                }
            }
            Value::String(s) => {
                encode_string_to_writer(s, writer, delimiter)?;
            }
            _ => {
                return Err(Error::Serialization(
                    "Non-primitive in inline array".to_string(),
                ));
            }
        }
        first = false;
    }

    Ok(())
}

fn encode_list_array_to_writer<W: Write>(
    arr: &[Value],
    writer: &mut W,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    let indent = options.get_indent();
    let indent_str = " ".repeat(indent_level * indent);

    for item in arr {
        writer.write_all(indent_str.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        writer.write_all(" ".repeat(indent).as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        writer.write_all(b"- ").map_err(|e| Error::Io(e.to_string()))?;
        // For objects in list arrays, encode them inline as key: value
        match item {
            Value::Object(obj) => {
                let mut first = true;
                for (key, val) in obj {
                    if !first {
                        writer.write_all(b" ").map_err(|e| Error::Io(e.to_string()))?;
                    }
                    writer.write_all(key.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
                    writer.write_all(b": ").map_err(|e| Error::Io(e.to_string()))?;
                    encode_primitive_value_to_writer(val, writer, options.get_delimiter())?;
                    first = false;
                }
            }
            _ => {
                encode_value_to_writer(item, writer, indent_level + 1, options)?;
            }
        }
        writer.write_all(b"\n").map_err(|e| Error::Io(e.to_string()))?;
    }

    Ok(())
}

fn encode_object_to_writer<W: Write>(
    obj: &serde_json::Map<String, Value>,
    writer: &mut W,
    indent_level: usize,
    options: &EncodeOptions,
) -> Result<(), Error> {
    if obj.is_empty() {
        return Ok(());
    }

    let indent = options.get_indent();
    let indent_str = " ".repeat(indent_level * indent);

    let mut first = true;
    for (key, value) in obj {
        if !first {
            writer.write_all(b"\n").map_err(|e| Error::Io(e.to_string()))?;
        }
        writer.write_all(indent_str.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
        writer.write_all(key.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;

        match value {
            Value::Array(arr) => {
                // For arrays, check the format and encode appropriately
                if arr.is_empty() {
                    writer.write_all(b"[0]:").map_err(|e| Error::Io(e.to_string()))?;
                } else if let Some(keys) = check_uniform_objects(arr) {
                    // Tabular array - output on same line: key[N]{...}:
                    let length_marker = options
                        .length_marker
                        .map(|m| format!("{m}"))
                        .unwrap_or_default();
                    let header = format!("[{}{}]", length_marker, arr.len());
                    writer.write_all(header.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
                    writer.write_all(b"{").map_err(|e| Error::Io(e.to_string()))?;
                    let keys_str = keys.join(&options.get_delimiter().to_string());
                    writer.write_all(keys_str.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
                    writer.write_all(b"}:\n").map_err(|e| Error::Io(e.to_string()))?;
                    // Now output the rows
                    encode_tabular_array_rows_to_writer(arr, keys, writer, indent_level, options)?;
                } else if arr.iter().all(is_primitive) {
                    // Inline array - output on same line: key[N]: value1,value2
                    let length_marker = options
                        .length_marker
                        .map(|m| format!("{m}"))
                        .unwrap_or_default();
                    let header = format!("[{}{}]:", length_marker, arr.len());
                    writer.write_all(header.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
                    let delimiter = options.get_delimiter();
                    let mut first = true;
                    for item in arr {
                        if !first {
                            let delim_bytes = [delimiter as u8];
                            writer.write_all(&delim_bytes).map_err(|e| Error::Io(e.to_string()))?;
                        }
                        encode_primitive_value_to_writer(item, writer, delimiter)?;
                        first = false;
                    }
                } else {
                    // List array - output on same line: key[N]:
                    let length_marker = options
                        .length_marker
                        .map(|m| format!("{m}"))
                        .unwrap_or_default();
                    let header = format!("[{}{}]:", length_marker, arr.len());
                    writer.write_all(header.as_bytes()).map_err(|e| Error::Io(e.to_string()))?;
                    writer.write_all(b"\n").map_err(|e| Error::Io(e.to_string()))?;
                    encode_list_array_to_writer(arr, writer, indent_level, options)?;
                }
            }
            Value::Object(_) => {
                writer.write_all(b": ").map_err(|e| Error::Io(e.to_string()))?;
                writer.write_all(b"\n").map_err(|e| Error::Io(e.to_string()))?;
                encode_value_to_writer(value, writer, indent_level + 1, options)?;
            }
            _ => {
                writer.write_all(b": ").map_err(|e| Error::Io(e.to_string()))?;
                encode_value_to_writer(value, writer, indent_level, options)?;
            }
        }
        first = false;
    }

    Ok(())
}
