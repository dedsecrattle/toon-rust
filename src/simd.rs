//! SIMD-optimized parsing functions for tabular arrays
//!
//! This module provides high-performance implementations of delimiter detection
//! and row splitting using SIMD instructions for parallel processing.

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Detect delimiter character using SIMD for fast scanning
///
/// Scans the input string for tab ('\t'), pipe ('|'), or comma (',') delimiters
/// in parallel using SIMD instructions.
///
/// # Returns
///
/// The first delimiter found in priority order: tab > pipe > comma
/// Defaults to comma if none found.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn detect_delimiter_simd_x86_64(input: &str) -> char {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return ',';
    }

    // Create SIMD vectors for each delimiter (16 bytes for SSE2)
    let tab_vec = _mm_set1_epi8(b'\t' as i8);
    let pipe_vec = _mm_set1_epi8(b'|' as i8);
    let comma_vec = _mm_set1_epi8(b',' as i8);

    let mut found_tab = false;
    let mut found_pipe = false;
    let mut found_comma = false;

    // Process in chunks of 16 bytes (SSE2 register size)
    let chunks = bytes.chunks_exact(16);
    let remainder = chunks.remainder();

    for chunk in chunks {
        // Load 16 bytes (unaligned is fine for most cases)
        let chunk_vec = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);

        // Compare with each delimiter
        let tab_mask = _mm_cmpeq_epi8(chunk_vec, tab_vec);
        let pipe_mask = _mm_cmpeq_epi8(chunk_vec, pipe_vec);
        let comma_mask = _mm_cmpeq_epi8(chunk_vec, comma_vec);

        // Check if any byte matches (movemask gives us a bitmask)
        let tab_bits = _mm_movemask_epi8(tab_mask);
        let pipe_bits = _mm_movemask_epi8(pipe_mask);
        let comma_bits = _mm_movemask_epi8(comma_mask);

        if tab_bits != 0 {
            found_tab = true;
        }
        if pipe_bits != 0 {
            found_pipe = true;
        }
        if comma_bits != 0 {
            found_comma = true;
        }

        // Early exit if we found tab (highest priority)
        if found_tab {
            return '\t';
        }
    }

    // Process remainder
    for &byte in remainder {
        if byte == b'\t' {
            return '\t';
        } else if byte == b'|' {
            found_pipe = true;
        } else if byte == b',' {
            found_comma = true;
        }
    }

    // Return in priority order
    if found_tab {
        '\t'
    } else if found_pipe {
        '|'
    } else if found_comma {
        ','
    } else {
        ',' // default
    }
}

/// Split a row by delimiter while respecting quoted strings, using SIMD
///
/// This function uses SIMD to quickly find delimiter positions and quote positions,
/// then processes them to handle quote tracking correctly.
///
/// # Arguments
///
/// * `row` - The row string to split
/// * `delimiter` - The delimiter character to split on
///
/// # Returns
///
/// A vector of string slices representing the split fields
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn split_row_simd_x86_64<'a>(row: &'a str, delimiter: char) -> Vec<&'a str> {
    let bytes = row.as_bytes();
    if bytes.is_empty() {
        return vec![row];
    }

    let delimiter_byte = delimiter as u8;
    let quote_byte = b'"';
    let backslash_byte = b'\\';

    let delim_vec = _mm_set1_epi8(delimiter_byte as i8);
    let quote_vec = _mm_set1_epi8(quote_byte as i8);
    let backslash_vec = _mm_set1_epi8(backslash_byte as i8);

    let mut result = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let mut prev_was_backslash = false;

    // Process in chunks of 16 bytes
    let chunks = bytes.chunks_exact(16);
    let remainder_start = chunks.len() * 16;

    for (chunk_idx, chunk) in chunks.enumerate() {
        let chunk_start = chunk_idx * 16;
        let chunk_vec = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);

        // Find delimiters, quotes, and backslashes in parallel
        let delim_mask = _mm_cmpeq_epi8(chunk_vec, delim_vec);
        let quote_mask = _mm_cmpeq_epi8(chunk_vec, quote_vec);
        let backslash_mask = _mm_cmpeq_epi8(chunk_vec, backslash_vec);

        // Get bitmasks
        let delim_bits = _mm_movemask_epi8(delim_mask) as u16;
        let quote_bits = _mm_movemask_epi8(quote_mask) as u16;
        let backslash_bits = _mm_movemask_epi8(backslash_mask) as u16;

        // Process each byte in the chunk
        for i in 0..16 {
            let pos = chunk_start + i;
            if pos >= bytes.len() {
                break;
            }
            let byte = bytes[pos];
            let is_backslash = (backslash_bits >> i) & 1 != 0;
            let is_quote = (quote_bits >> i) & 1 != 0;
            let is_delimiter = (delim_bits >> i) & 1 != 0;

            // Handle backslash: if we see a backslash, the next character might be escaped
            // But if the previous was a backslash, this one escapes it, so reset
            if is_backslash {
                prev_was_backslash = !prev_was_backslash;
            } else {
                // Not a backslash - if prev was backslash, this char is escaped
                let is_escaped = prev_was_backslash;
                prev_was_backslash = false;

                // Handle quotes (only if not escaped)
                if is_quote && !is_escaped {
                    in_quotes = !in_quotes;
                }

                // Handle delimiter (only if not in quotes)
                if is_delimiter && !in_quotes {
                    result.push(&row[start..pos]);
                    start = pos + 1;
                }
            }
        }
    }

    // Process remainder
    for (i, &byte) in bytes[remainder_start..].iter().enumerate() {
        let pos = remainder_start + i;

        // Handle backslash tracking
        if byte == backslash_byte {
            prev_was_backslash = !prev_was_backslash;
        } else {
            // Not a backslash - check if previous was backslash (this char is escaped)
            let is_escaped = prev_was_backslash;
            prev_was_backslash = false;

            // Handle quotes (only if not escaped)
            if byte == quote_byte && !is_escaped {
                in_quotes = !in_quotes;
            }

            // Handle delimiter (only if not in quotes)
            if byte == delimiter_byte && !in_quotes {
                result.push(&row[start..pos]);
                start = pos + 1;
            }
        }
    }

    // Add the final segment
    result.push(&row[start..]);
    result
}

/// Public wrapper for SIMD delimiter detection with fallback
pub fn detect_delimiter_simd(input: &str) -> char {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") && input.len() >= 16 {
            unsafe {
                return detect_delimiter_simd_x86_64(input);
            }
        }
    }

    // Fallback for other architectures or small inputs
    detect_delimiter_fallback(input)
}

/// Public wrapper for SIMD row splitting with fallback
pub fn split_row_simd<'a>(row: &'a str, delimiter: char) -> Vec<&'a str> {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") && row.len() >= 16 {
            unsafe {
                return split_row_simd_x86_64(row, delimiter);
            }
        }
    }

    // Fallback for other architectures or small inputs
    split_row_fallback(row, delimiter)
}

/// Fallback implementation for small inputs or when SIMD isn't beneficial
///
/// This is used when the input is too small to benefit from SIMD operations.
pub fn detect_delimiter_fallback(input: &str) -> char {
    if input.contains('\t') {
        '\t'
    } else if input.contains('|') {
        '|'
    } else {
        ','
    }
}

/// Fallback implementation for row splitting
pub fn split_row_fallback<'a>(row: &'a str, delimiter: char) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let chars: Vec<char> = row.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        match ch {
            '"' if i == 0 || chars[i - 1] != '\\' => {
                in_quotes = !in_quotes;
            }
            _ if *ch == delimiter && !in_quotes => {
                result.push(&row[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    result.push(&row[start..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_delimiter_tab() {
        let input = "field1\tfield2\tfield3";
        assert_eq!(detect_delimiter_simd(input), '\t');
    }

    #[test]
    fn test_detect_delimiter_pipe() {
        let input = "field1|field2|field3";
        assert_eq!(detect_delimiter_simd(input), '|');
    }

    #[test]
    fn test_detect_delimiter_comma() {
        let input = "field1,field2,field3";
        assert_eq!(detect_delimiter_simd(input), ',');
    }

    #[test]
    fn test_detect_delimiter_priority() {
        // Tab should have priority over pipe and comma
        let input = "field1,field2|field3\tfield4";
        assert_eq!(detect_delimiter_simd(input), '\t');
    }

    #[test]
    fn test_split_row_simple() {
        let row = "a,b,c";
        let result = split_row_simd(row, ',');
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_row_with_quotes() {
        let row = r#"a,"b,c",d"#;
        let result = split_row_simd(row, ',');
        assert_eq!(result, vec!["a", r#""b,c""#, "d"]);
    }

    #[test]
    fn test_split_row_with_escaped_quotes() {
        let row = r#"a,"b\"c",d"#;
        let result = split_row_simd(row, ',');
        assert_eq!(result, vec!["a", r#""b\"c""#, "d"]);
    }

    #[test]
    fn test_split_row_tab_delimiter() {
        let row = "a\tb\tc";
        let result = split_row_simd(row, '\t');
        assert_eq!(result, vec!["a", "b", "c"]);
    }
}
