# Roadmap - Future Upgrades for toon-rust

This document outlines potential future improvements and enhancements for the `toon-rust` library.

## 🚀 Performance Enhancements

### 1. Streaming API

- **Goal**: Support encoding/decoding large datasets without loading everything into memory
- **Implementation**: Add `encode_stream()` and `decode_stream()` functions
- **Use Case**: Processing large files or network streams
- **Priority**: High

```rust
// Proposed API
let mut writer = BufWriter::new(File::create("output.toon")?);
encode_stream(&data, &mut writer, options)?;
```

### 2. SIMD Optimizations

- **Goal**: Use SIMD instructions for faster parsing of tabular arrays
- **Implementation**: Leverage `packed_simd` or `portable-simd` for delimiter detection
- **Use Case**: High-throughput scenarios with large tabular arrays
- **Priority**: Medium

### 3. Zero-Copy Improvements

- **Goal**: Further reduce allocations during parsing
- **Implementation**: Use `Cow<str>` for string values, `&str` slices where possible
- **Use Case**: Memory-constrained environments
- **Priority**: Medium

### 4. Parallel Encoding/Decoding

- **Goal**: Parallelize processing of large arrays
- **Implementation**: Use `rayon` for parallel tabular array processing
- **Use Case**: Multi-core systems with large datasets
- **Priority**: Low (add as optional feature)

## 📊 Feature Additions

### 5. Custom Delimiters

- **Goal**: Allow any character as delimiter, not just comma/tab/pipe
- **Implementation**: Extend `Delimiter` enum or add `custom_delimiter(char)` option
- **Use Case**: Integration with existing data formats
- **Priority**: Medium

```rust
let options = EncodeOptions::new()
    .custom_delimiter(';'); // Use semicolon instead
```

### 6. Schema Validation

- **Goal**: Validate TOON structure against a schema
- **Implementation**: Add schema definition and validation layer
- **Use Case**: Type-safe TOON parsing
- **Priority**: Low

### 7. Pretty Printing

- **Goal**: Enhanced formatting options for human readability
- **Implementation**: Add options for color output, alignment, etc.
- **Use Case**: CLI tools, debugging
- **Priority**: Low

### 8. Comments Support

- **Goal**: Support comments in TOON format (if spec allows)
- **Implementation**: Parse and optionally preserve comments
- **Use Case**: Configuration files, documentation
- **Priority**: Low

### 9. Binary TOON Format

- **Goal**: Binary encoding for even more compact representation
- **Implementation**: Add `toon-binary` feature with binary serialization
- **Use Case**: Network protocols, storage optimization
- **Priority**: Low

### 10. Incremental Parsing

- **Goal**: Parse TOON incrementally as data arrives
- **Implementation**: State machine-based parser with partial results
- **Use Case**: Network protocols, real-time processing
- **Priority**: Medium

## 🔧 API Improvements

### 11. Builder Pattern for Options

- **Goal**: More ergonomic option configuration
- **Implementation**: Already partially done, but could add more convenience methods
- **Priority**: Low

### 12. Error Recovery

- **Goal**: Continue parsing after errors, collecting all errors
- **Implementation**: Add `decode_with_errors()` returning `Vec<Error>`
- **Use Case**: Linting tools, format validators
- **Priority**: Medium

### 13. Position Information in Errors

- **Goal**: Provide line/column numbers in parse errors
- **Implementation**: Track position during parsing
- **Use Case**: Better error messages for debugging
- **Priority**: High

```rust
match decode(&toon, None) {
    Err(Error::Parse { line, column, message }) => {
        eprintln!("Error at line {}:{} - {}", line, column, message);
    }
    // ...
}
```

### 14. Custom Error Types

- **Goal**: More specific error types for different failure modes
- **Implementation**: Split `Error` enum into more granular types
- **Use Case**: Better error handling in user code
- **Priority**: Medium

### 15. Async Support

- **Goal**: Async/await support for I/O operations
- **Implementation**: Add `tokio` or `async-std` features
- **Use Case**: Async web servers, async file I/O
- **Priority**: Medium

```rust
#[cfg(feature = "async")]
pub async fn encode_async<T: Serialize>(value: &T) -> Result<String, Error> {
    // ...
}
```

## 🛠️ Developer Experience

### 16. Procedural Macros

- **Goal**: Derive macros for TOON serialization
- **Implementation**: `#[derive(ToonSerialize, ToonDeserialize)]`
- **Use Case**: Simpler API without Serde dependency
- **Priority**: Low

### 17. CLI Tool

- **Goal**: Command-line tool for TOON conversion
- **Implementation**: `toon-cli` binary crate
- **Use Case**: Quick conversions, testing, debugging
- **Priority**: Medium

```bash
# Proposed CLI usage
toon convert input.json output.toon
toon validate data.toon
toon format --indent 4 data.toon
```

### 18. VS Code Extension

- **Goal**: Syntax highlighting and formatting for `.toon` files
- **Implementation**: Language server protocol (LSP) support
- **Use Case**: Better editor experience
- **Priority**: Low

### 19. Benchmark Suite

- **Goal**: Comprehensive performance benchmarks
- **Implementation**: Expand criterion benchmarks, compare with JSON
- **Use Case**: Performance tracking, optimization validation
- **Priority**: Medium

### 20. Fuzzing

- **Goal**: Automated fuzzing for security and correctness
- **Implementation**: Add `cargo-fuzz` integration
- **Use Case**: Finding edge cases and security issues
- **Priority**: High

## 📚 Documentation & Testing

### 21. More Examples

- **Goal**: Comprehensive example gallery
- **Implementation**: Add examples for edge cases, real-world scenarios
- **Use Case**: Learning, reference
- **Priority**: Medium

### 22. Integration Tests

- **Goal**: Test against official TOON test suite
- **Implementation**: Import and run TOON spec test cases
- **Use Case**: Compliance verification
- **Priority**: High

### 23. Property-Based Testing

- **Goal**: Use `proptest` or `quickcheck` for round-trip properties
- **Implementation**: Generate random data, verify encode/decode round-trips
- **Use Case**: Finding edge cases automatically
- **Priority**: Medium

### 24. Documentation Improvements

- **Goal**: More detailed API docs with examples
- **Implementation**: Expand rustdoc comments, add more code examples
- **Use Case**: Better developer experience
- **Priority**: Medium

## 🔒 Security & Reliability

### 25. Resource Limits

- **Goal**: Prevent DoS attacks via resource exhaustion
- **Implementation**: Add max depth, max size, max array length limits
- **Use Case**: Parsing untrusted input
- **Priority**: High

```rust
let options = DecodeOptions::new()
    .max_depth(100)
    .max_size(10_000_000) // 10MB limit
    .max_array_length(1_000_000);
```

### 26. Memory Safety Audits

- **Goal**: Security review of parsing code
- **Implementation**: External security audit
- **Use Case**: Production readiness
- **Priority**: Medium

### 27. No-Std Support

- **Goal**: Support embedded/`no_std` environments
- **Implementation**: Feature flag for `no_std`, use `alloc` crate
- **Use Case**: Embedded systems, WebAssembly
- **Priority**: Medium

## 🌐 Ecosystem Integration

### 28. WebAssembly Support

- **Goal**: Optimize for WASM, publish to npm
- **Implementation**: WASM bindings, npm package
- **Use Case**: Browser-based applications
- **Priority**: Low

### 29. Python Bindings

- **Goal**: Python wrapper using PyO3
- **Implementation**: `toon-python` crate with Python bindings
- **Use Case**: Python ecosystem integration
- **Priority**: Low

### 30. Node.js Bindings

- **Goal**: Native Node.js addon
- **Implementation**: `napi-rs` or `neon` bindings
- **Use Case**: Node.js ecosystem integration
- **Priority**: Low

## 📈 Metrics & Observability

### 31. Token Count Utilities

- **Goal**: Helper functions to count tokens in TOON vs JSON
- **Implementation**: Add `token_count()` and `compare_with_json()` functions
- **Use Case**: Measuring token savings
- **Priority**: Medium

```rust
let toon = encode(&data, None)?;
let json = serde_json::to_string(&data)?;
let savings = compare_token_counts(&toon, &json);
println!("Saved {} tokens ({:.1}%)", savings.count, savings.percentage);
```

### 32. Format Statistics

- **Goal**: Analyze TOON structure (array types, nesting depth, etc.)
- **Implementation**: Add `analyze()` function returning statistics
- **Use Case**: Optimization, debugging
- **Priority**: Low

## 🎯 Quick Wins (Easy to Implement)

1. ✅ **Better error messages** - Add line/column numbers
2. ✅ **Resource limits** - Add max depth/size options
3. ✅ **More examples** - Add edge case examples
4. ✅ **CLI tool** - Basic conversion tool
5. ✅ **Token counting** - Simple token comparison utilities

## 📅 Suggested Timeline

### Phase 1 (Next Release - v0.2.0)

- Error position information
- Resource limits
- More comprehensive tests
- Fuzzing setup

### Phase 2 (v0.3.0)

- Streaming API
- Async support
- CLI tool
- Token counting utilities

### Phase 3 (v0.4.0+)

- SIMD optimizations
- Custom delimiters
- Incremental parsing
- WebAssembly support

## 🤝 Contributing Ideas

Have ideas for improvements? Please:

1. Open an issue with the `enhancement` label
2. Discuss in GitHub Discussions
3. Submit a PR with implementation

## 📝 Notes

- Priorities are suggestions and may change based on community feedback
- Some features may require TOON specification updates
- Performance improvements should be benchmarked before merging
- Breaking changes should be carefully considered and documented

---

**Last Updated**: 2024-11-08
