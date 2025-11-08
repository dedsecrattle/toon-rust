//! Decoding TOON format to JSON values

use crate::error::Error;
use crate::options::DecodeOptions;
use crate::simd;
use serde_json::{Map, Value};

/// Decode a TOON-formatted string to a JSON value
///
/// # Arguments
///
/// * `input` - The TOON-formatted string to decode
/// * `options` - Optional decoding options
///
/// # Returns
///
/// A `Result` containing the decoded JSON value or an error
pub fn decode(input: &str, options: Option<&DecodeOptions>) -> Result<Value, Error> {
    let default_opts = DecodeOptions::default();
    let opts = options.unwrap_or(&default_opts);
    let mut parser = Parser::new(input, opts);
    parser.parse()
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    options: &'a DecodeOptions,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str, options: &'a DecodeOptions) -> Self {
        Self {
            input,
            pos: 0,
            options,
        }
    }

    fn parse(&mut self) -> Result<Value, Error> {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Ok(Value::Object(Map::new()));
        }

        // Check if it's a root array (starts with [)
        if self.peek_char() == Some('[') {
            self.parse_array_value()
        } else {
            self.parse_object()
        }
    }

    fn parse_object(&mut self) -> Result<Value, Error> {
        let mut map = Map::new();
        let indent = self.options.get_indent();
        let initial_indent = self.count_indent(indent);

        loop {
            // Count indentation first (before skipping whitespace)
            let line_indent = self.count_indent(indent);
            if line_indent < initial_indent {
                // We've gone back to a lower indentation level
                break;
            }

            // Now skip the indentation whitespace
            for _ in 0..(line_indent * indent) {
                if self.peek_char() == Some(' ') {
                    self.advance();
                } else {
                    break;
                }
            }

            if self.pos >= self.input.len() {
                break;
            }
            if line_indent == 0 && !map.is_empty() && initial_indent == 0 {
                // Check if there's actually more content on this line
                let saved_pos = self.pos;
                let key_result = self.parse_key();
                self.pos = saved_pos;
                if key_result.is_err() {
                    break;
                }
            }

            // Parse key (may include array notation like "tags[3]")
            let key = self.parse_key()?;
            self.skip_whitespace();

            // Check if we have array notation in the key (e.g., "tags[3]:")
            let has_array_notation = self.peek_char() == Some('[');

            if !has_array_notation {
                // Normal key-value: key: value
                if self.peek_char() != Some(':') {
                    return Err(Error::parse(
                        self.pos,
                        format!("Expected ':' after key '{key}'"),
                    ));
                }
                self.advance(); // consume ':'
                self.skip_whitespace();
            } else {
                // Key with array notation: key[3]: value
                // The array part will be parsed as the value
            }

            // Check if value is on next line (indented) or inline
            let value = if has_array_notation {
                // Array notation: key[3]: value
                // Parse the array value
                let value = self.parse_array_value()?;
                // Skip to next line
                if self.pos < self.input.len() && self.peek_char() == Some('\n') {
                    self.advance();
                }
                value
            } else if self.peek_char() == Some('\n') {
                self.advance(); // consume '\n'
                                // Check if next line is more indented (nested object/array)
                let next_indent = self.count_indent(indent);
                if next_indent > line_indent {
                    // Parse nested object or array
                    if self.peek_char() == Some('[') {
                        self.parse_array_value()?
                    } else {
                        // Parse nested object
                        // After parsing nested object, check if we should continue
                        // The recursive parse_object() will have consumed all nested content
                        // and positioned us at the end or at a lower indentation level
                        self.parse_object()?
                    }
                } else {
                    // Same or less indent means we're done with this value
                    Value::Null
                }
            } else {
                // Inline value - parse until end of line or newline
                let value = self.parse_value_until_newline()?;
                // Skip to next line (if not already at end)
                if self.pos < self.input.len() && self.peek_char() != Some('\n') {
                    self.skip_to_next_line();
                } else if self.peek_char() == Some('\n') {
                    self.advance(); // consume newline
                }
                value
            };

            map.insert(key, value);

            // After inserting a nested object, check if we should continue
            // If we're at the end or at a lower indentation level, break
            if self.pos >= self.input.len() {
                break;
            }

            // Check indentation for next iteration
            let next_line_indent = self.count_indent(indent);
            if next_line_indent < initial_indent {
                break;
            }
            if next_line_indent == 0 && initial_indent == 0 && !map.is_empty() {
                // Check if there's actually a key to parse
                let saved_pos = self.pos;
                let key_result = self.parse_key();
                self.pos = saved_pos;
                if key_result.is_err() {
                    break;
                }
            }
        }

        Ok(Value::Object(map))
    }

    fn parse_value(&mut self) -> Result<Value, Error> {
        self.skip_whitespace();
        match self.peek_char() {
            Some('[') => self.parse_array_value(),
            Some('"') => self.parse_string(),
            Some('-') => {
                // List item marker
                self.advance();
                self.skip_whitespace();
                self.parse_value()
            }
            Some(ch) if ch.is_ascii_digit() || ch == '-' => self.parse_number(),
            Some(ch) if ch.is_ascii_alphabetic() => {
                // Try boolean/null first, then fall back to string
                let start = self.pos;
                let value = self.parse_boolean_or_null();
                if value.is_ok() {
                    return value;
                }
                // Reset and parse as string
                self.pos = start;
                self.parse_unquoted_string()
            }
            _ => self.parse_unquoted_string(),
        }
    }

    fn parse_unquoted_string(&mut self) -> Result<Value, Error> {
        let start = self.pos;
        // Parse until we hit whitespace, newline, or end
        while self.pos < self.input.len() {
            match self.peek_char() {
                Some(ch) if ch == ' ' || ch == '\n' || ch == '\t' || ch == '\r' => break,
                Some(_) => self.advance(),
                None => break,
            }
        }
        if self.pos == start {
            return Err(Error::parse(self.pos, "Expected value"));
        }
        Ok(Value::String(self.input[start..self.pos].to_string()))
    }

    fn parse_value_until_newline(&mut self) -> Result<Value, Error> {
        self.skip_whitespace();

        // Check what type of value we have
        match self.peek_char() {
            Some('[') => {
                // Array - parse array value
                self.parse_array_value()
            }
            Some('"') => self.parse_string(),
            Some(ch) if ch.is_ascii_digit() || ch == '-' => self.parse_number(),
            Some(ch) if ch.is_ascii_alphabetic() => {
                // Try boolean/null first, then fall back to string
                let start_pos = self.pos;
                let value = self.parse_boolean_or_null();
                if value.is_ok() {
                    return value;
                }
                // Reset and parse as string
                self.pos = start_pos;
                self.parse_unquoted_string()
            }
            _ => self.parse_unquoted_string(),
        }
    }

    fn parse_array_value(&mut self) -> Result<Value, Error> {
        if self.peek_char() != Some('[') {
            return Err(Error::parse(self.pos, "Expected '['"));
        }
        self.advance(); // consume '['

        // Parse length marker (optional #) and length
        let has_length_marker = self.peek_char() == Some('#');
        if has_length_marker {
            self.advance(); // consume '#'
        }

        let length_str = self.parse_while(|ch| ch.is_ascii_digit());
        let length: usize = length_str
            .parse()
            .map_err(|_| Error::parse(self.pos, "Invalid array length"))?;

        if self.peek_char() != Some(']') {
            return Err(Error::parse(self.pos, "Expected ']'"));
        }
        self.advance(); // consume ']'

        // Check for tabular format: {field1,field2}:
        if self.peek_char() == Some('{') {
            self.parse_tabular_array(length)
        } else if self.peek_char() == Some(':') {
            self.advance(); // consume ':'
            self.skip_whitespace();

            // Check if it's inline (same line) or list format (next line)
            if length == 0 {
                // Empty array - skip any whitespace and newline
                self.skip_whitespace();
                if self.peek_char() == Some('\n') {
                    self.advance();
                }
                Ok(Value::Array(Vec::new()))
            } else if self.peek_char() == Some('\n') || self.pos >= self.input.len() {
                self.parse_list_array(length)
            } else {
                self.parse_inline_array(length)
            }
        } else {
            Err(Error::parse(
                self.pos,
                "Expected ':' or '{' after array length",
            ))
        }
    }

    fn parse_tabular_array(&mut self, expected_length: usize) -> Result<Value, Error> {
        if self.peek_char() != Some('{') {
            return Err(Error::parse(self.pos, "Expected '{'"));
        }
        self.advance(); // consume '{'

        // Parse field names
        let fields_str = self.parse_while(|ch| ch != '}');
        let fields: Vec<&str> = fields_str.split(',').map(|s| s.trim()).collect();
        let delimiter = self.detect_delimiter();

        if self.peek_char() != Some('}') {
            return Err(Error::parse(self.pos, "Expected '}'"));
        }
        self.advance(); // consume '}'

        if self.peek_char() != Some(':') {
            return Err(Error::parse(self.pos, "Expected ':'"));
        }
        self.advance(); // consume ':'
                        // Skip to next line (consume newline if present)
        if self.peek_char() == Some('\n') {
            self.advance();
        }

        // Parse rows
        let mut items = Vec::new();
        let indent = self.options.get_indent();
        // Count base indentation of first row
        let base_indent = self.count_indent(indent);

        for _ in 0..expected_length {
            if self.pos >= self.input.len() {
                break;
            }

            // Count indentation of current line
            let line_indent = self.count_indent(indent);
            if line_indent < base_indent {
                break; // Back at lower indentation level
            }

            // Skip the indentation whitespace
            for _ in 0..(line_indent * indent) {
                if self.peek_char() == Some(' ') {
                    self.advance();
                } else {
                    break;
                }
            }

            let mut obj = Map::new();
            let start = self.pos;
            // Parse until newline
            while self.pos < self.input.len() && self.peek_char() != Some('\n') {
                self.advance();
            }
            let row = &self.input[start..self.pos];
            let values: Vec<&str> = self.split_row(row, delimiter);

            if values.len() != fields.len() && self.options.get_strict() {
                return Err(Error::LengthMismatch {
                    expected: fields.len(),
                    found: values.len(),
                });
            }

            for (i, field) in fields.iter().enumerate() {
                let value_str = values.get(i).unwrap_or(&"");
                let value = self.parse_primitive_value(value_str.trim())?;
                obj.insert(field.to_string(), value);
            }

            items.push(Value::Object(obj));
            // Skip to next line
            if self.pos < self.input.len() && self.peek_char() == Some('\n') {
                self.advance();
            }
        }

        if self.options.get_strict() && items.len() != expected_length {
            return Err(Error::LengthMismatch {
                expected: expected_length,
                found: items.len(),
            });
        }

        Ok(Value::Array(items))
    }

    fn parse_inline_array(&mut self, expected_length: usize) -> Result<Value, Error> {
        let delimiter = self.detect_delimiter();
        let start = self.pos;
        // Parse until newline
        while self.pos < self.input.len() && self.peek_char() != Some('\n') {
            self.advance();
        }
        let row = &self.input[start..self.pos];
        let values: Vec<&str> = self.split_row(row, delimiter);

        let mut items = Vec::new();
        for value_str in values {
            let trimmed = value_str.trim();
            if !trimmed.is_empty() {
                items.push(self.parse_primitive_value(trimmed)?);
            }
        }

        if self.options.get_strict() && items.len() != expected_length {
            return Err(Error::LengthMismatch {
                expected: expected_length,
                found: items.len(),
            });
        }

        Ok(Value::Array(items))
    }

    fn parse_list_array(&mut self, expected_length: usize) -> Result<Value, Error> {
        // Skip to next line if we're not already there
        if self.peek_char() == Some('\n') {
            self.advance();
        }
        let indent = self.options.get_indent();
        // Count base indentation of first item
        let base_indent = self.count_indent(indent);
        let mut items = Vec::new();

        for _ in 0..expected_length {
            if self.pos >= self.input.len() {
                break;
            }

            // Count indentation of current line
            let line_indent = self.count_indent(indent);
            if line_indent < base_indent {
                break; // Back at lower indentation level
            }

            // Skip the indentation whitespace
            for _ in 0..(line_indent * indent) {
                if self.peek_char() == Some(' ') {
                    self.advance();
                } else {
                    break;
                }
            }

            // Check if there's a '-' marker (optional in some formats)
            let has_dash = self.peek_char() == Some('-');
            if has_dash {
                self.advance(); // consume '-'
                self.skip_whitespace();
            }

            // Parse the value - we've already skipped indentation and optionally the '-'
            // The value could be a primitive, object, or array
            // Check if this line looks like an object (has key: value format)
            let line_start = self.pos;
            let line_end = self.input[line_start..]
                .find('\n')
                .map(|i| line_start + i)
                .unwrap_or(self.input.len());
            let line = &self.input[line_start..line_end].trim();

            let value = if self.peek_char() == Some('[') {
                self.parse_array_value()?
            } else if line.contains(':')
                && !line.starts_with('"')
                && line.matches(':').count() == 1
                && !line.trim_start().starts_with('-')
            {
                // It's an object (single key:value on this line, like "a: 1")
                // Parse as a simple key-value pair manually (don't use parse_object which expects indentation)
                let key = self.parse_key()?;
                self.skip_whitespace();
                if self.peek_char() != Some(':') {
                    return Err(Error::parse(
                        self.pos,
                        format!("Expected ':' after key '{key}'"),
                    ));
                }
                self.advance(); // consume ':'
                self.skip_whitespace();
                let val = self.parse_value()?;
                let mut obj = Map::new();
                obj.insert(key, val);
                Value::Object(obj)
            } else {
                // Primitive value (number, string, boolean, etc.)
                self.parse_value()?
            };
            items.push(value);
            // Skip to next line
            if self.pos < self.input.len() && self.peek_char() == Some('\n') {
                self.advance();
            }
        }

        if self.options.get_strict() && items.len() != expected_length {
            return Err(Error::LengthMismatch {
                expected: expected_length,
                found: items.len(),
            });
        }

        Ok(Value::Array(items))
    }

    fn parse_primitive_value(&self, s: &str) -> Result<Value, Error> {
        if s.is_empty() {
            return Ok(Value::Null);
        }

        // Try boolean
        if s == "true" {
            return Ok(Value::Bool(true));
        }
        if s == "false" {
            return Ok(Value::Bool(false));
        }

        // Try number
        if let Ok(n) = s.parse::<i64>() {
            return Ok(Value::Number(n.into()));
        }
        if let Ok(n) = s.parse::<f64>() {
            return Ok(Value::Number(
                serde_json::Number::from_f64(n)
                    .ok_or_else(|| Error::InvalidNumber(s.to_string()))?,
            ));
        }

        // Must be a string (possibly quoted)
        if s.starts_with('"') && s.ends_with('"') {
            self.parse_quoted_string(s)
        } else {
            Ok(Value::String(s.to_string()))
        }
    }

    fn parse_quoted_string(&self, s: &str) -> Result<Value, Error> {
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();
        let mut i = 1; // Skip opening quote

        while i < chars.len() - 1 {
            // Skip closing quote
            match chars[i] {
                '\\' => {
                    i += 1;
                    if i >= chars.len() - 1 {
                        return Err(Error::InvalidEscape("Unterminated escape".to_string()));
                    }
                    match chars[i] {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        _ => {
                            return Err(Error::InvalidEscape(format!("\\{}", chars[i])));
                        }
                    }
                }
                ch => result.push(ch),
            }
            i += 1;
        }

        Ok(Value::String(result))
    }

    fn parse_string(&mut self) -> Result<Value, Error> {
        if self.peek_char() != Some('"') {
            return Err(Error::parse(self.pos, "Expected '\"'"));
        }
        self.advance(); // consume opening quote

        let start = self.pos;
        let mut escaped = false;

        while self.pos < self.input.len() {
            let ch = self.input.chars().nth(self.pos).unwrap();
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                let s = &self.input[start..self.pos];
                self.advance(); // consume closing quote
                return self.parse_quoted_string(&format!("\"{s}\""));
            }
            self.advance();
        }

        Err(Error::UnterminatedString)
    }

    fn parse_number(&mut self) -> Result<Value, Error> {
        let start = self.pos;
        let mut has_dot = false;

        if self.peek_char() == Some('-') {
            self.advance();
        }

        while self.pos < self.input.len() {
            match self.peek_char() {
                Some(ch) if ch.is_ascii_digit() => {
                    self.advance();
                }
                Some('.') if !has_dot => {
                    has_dot = true;
                    self.advance();
                }
                _ => break,
            }
        }

        let s = &self.input[start..self.pos];
        if has_dot {
            let n = s
                .parse::<f64>()
                .map_err(|_| Error::InvalidNumber(s.to_string()))?;
            serde_json::Number::from_f64(n)
                .ok_or_else(|| Error::InvalidNumber(s.to_string()))
                .map(Value::Number)
        } else {
            s.parse::<i64>()
                .map(|n| Value::Number(n.into()))
                .map_err(|_| Error::InvalidNumber(s.to_string()))
        }
    }

    fn parse_boolean_or_null(&mut self) -> Result<Value, Error> {
        let start = self.pos;
        self.parse_while(|ch| ch.is_ascii_alphabetic());
        let s = &self.input[start..self.pos];

        match s {
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            "null" => Ok(Value::Null),
            _ => {
                // Not a boolean/null, reset position
                self.pos = start;
                Err(Error::parse(
                    self.pos,
                    format!("Not a boolean or null: {s}"),
                ))
            }
        }
    }

    fn parse_key(&mut self) -> Result<String, Error> {
        self.skip_whitespace();
        let start = self.pos;
        // Parse key - stop at ':', '[', space, newline, or tab
        while self.pos < self.input.len() {
            match self.peek_char() {
                Some(ch) if ch == ':' || ch == '[' || ch == ' ' || ch == '\n' || ch == '\t' => {
                    break
                }
                Some(_) => self.advance(),
                None => break,
            }
        }
        if self.pos == start {
            return Err(Error::parse(self.pos, "Expected key"));
        }
        Ok(self.input[start..self.pos].to_string())
    }

    fn detect_delimiter(&self) -> char {
        // Look ahead to detect delimiter
        let remaining = &self.input[self.pos..];

        // Use SIMD for larger inputs, fallback for small ones
        // Threshold: use SIMD if input is large enough to benefit (>= 32 bytes)
        if remaining.len() >= 32 {
            simd::detect_delimiter_simd(remaining)
        } else {
            simd::detect_delimiter_fallback(remaining)
        }
    }

    fn split_row<'b>(&self, row: &'b str, delimiter: char) -> Vec<&'b str> {
        // Use SIMD for larger inputs, fallback for small ones
        // Threshold: use SIMD if row is large enough to benefit (>= 32 bytes)
        if row.len() >= 32 {
            simd::split_row_simd(row, delimiter)
        } else {
            simd::split_row_fallback(row, delimiter)
        }
    }

    fn count_indent(&mut self, indent_size: usize) -> usize {
        let start = self.pos;
        let mut count = 0;
        let indent_str = " ".repeat(indent_size);
        while self.pos < self.input.len() {
            if self.pos + indent_size <= self.input.len() {
                let slice = &self.input[self.pos..self.pos + indent_size];
                if slice == indent_str {
                    count += 1;
                    self.pos += indent_size;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let indent_level = count;
        self.pos = start;
        indent_level
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            match self.input.chars().nth(self.pos) {
                Some(' ') | Some('\t') => self.pos += 1,
                _ => break,
            }
        }
    }

    fn skip_to_next_line(&mut self) {
        while self.pos < self.input.len() {
            if self.input.chars().nth(self.pos) == Some('\n') {
                self.pos += 1;
                break;
            }
            self.pos += 1;
        }
    }

    fn parse_while<F>(&mut self, mut pred: F) -> &'a str
    where
        F: FnMut(char) -> bool,
    {
        let start = self.pos;
        while self.pos < self.input.len() {
            if let Some(ch) = self.input.chars().nth(self.pos) {
                if pred(ch) {
                    self.pos += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        &self.input[start..self.pos]
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            self.pos += 1;
        }
    }
}
