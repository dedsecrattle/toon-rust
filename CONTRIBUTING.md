# Contributing to toon-rust

Thank you for your interest in contributing to `toon-rust`! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/dedsecrattle/toon-rust/issues)
2. If not, create a new issue with:
   - A clear, descriptive title
   - Steps to reproduce the bug
   - Expected vs actual behavior
   - Rust version and OS information
   - Minimal code example if applicable

### Suggesting Enhancements

1. Check existing issues and discussions
2. Open an issue describing:
   - The enhancement and use case
   - Why it would be useful
   - Potential implementation approach (if you have ideas)

### Pull Requests

1. **Fork the repository** and create a feature branch
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow Rust style guidelines
   - Add tests for new functionality
   - Update documentation as needed

3. **Ensure code quality**
   ```bash
   # Format code
   cargo fmt
   
   # Run clippy
   cargo clippy -- -D warnings
   
   # Run tests
   cargo test
   ```

4. **Commit your changes**
   - Write clear, descriptive commit messages
   - Follow [Conventional Commits](https://www.conventionalcommits.org/) if possible

5. **Push and create a Pull Request**
   - Reference any related issues
   - Describe your changes clearly
   - Ensure CI checks pass

## Development Setup

```bash
# Clone the repository
git clone https://github.com/dedsecrattle/toon-rust.git
cd toon-rust

# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Build documentation
cargo doc --open
```

## Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and small
- Handle errors explicitly (avoid `unwrap()` in library code)

## Testing

- Add unit tests for new functionality
- Add integration tests for complex scenarios
- Ensure all tests pass before submitting PR
- Test edge cases and error conditions

## Documentation

- Update `README.md` if adding new features
- Add doc comments for public APIs
- Include usage examples in documentation
- Update `CHANGELOG.md` for user-facing changes

## Review Process

1. All PRs require review before merging
2. Maintainers will review for:
   - Code quality and style
   - Test coverage
   - Documentation completeness
   - Backward compatibility
3. Address review comments promptly
4. Squash commits if requested

## Questions?

Feel free to open an issue with the `question` label or start a discussion!

Thank you for contributing to `toon-rust`! 🎉

