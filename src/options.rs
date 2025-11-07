//! Options for encoding and decoding TOON format

/// Delimiter character for tabular arrays
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Delimiter {
    /// Comma delimiter (default)
    #[default]
    Comma,
    /// Tab delimiter
    Tab,
    /// Pipe delimiter
    Pipe,
}

impl Delimiter {
    /// Get the delimiter character
    pub fn as_char(self) -> char {
        match self {
            Delimiter::Comma => ',',
            Delimiter::Tab => '\t',
            Delimiter::Pipe => '|',
        }
    }
}

/// Options for encoding TOON format
#[derive(Debug, Clone, Default)]
pub struct EncodeOptions {
    /// Delimiter for tabular arrays (default: comma)
    pub delimiter: Option<Delimiter>,
    /// Optional hash prefix for array lengths (e.g., `[#3]` instead of `[3]`)
    pub length_marker: Option<char>,
    /// Number of spaces per indentation level (default: 2)
    pub indent: Option<usize>,
}

impl EncodeOptions {
    /// Create new default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the delimiter
    pub fn delimiter(mut self, delimiter: Delimiter) -> Self {
        self.delimiter = Some(delimiter);
        self
    }

    /// Set the length marker (typically `'#'`)
    pub fn length_marker(mut self, marker: char) -> Self {
        self.length_marker = Some(marker);
        self
    }

    /// Set the indentation level
    pub fn indent(mut self, indent: usize) -> Self {
        self.indent = Some(indent);
        self
    }

    /// Get the delimiter, defaulting to comma
    pub fn get_delimiter(&self) -> char {
        self.delimiter.unwrap_or_default().as_char()
    }

    /// Get the indentation, defaulting to 2
    pub fn get_indent(&self) -> usize {
        self.indent.unwrap_or(2)
    }
}

/// Options for decoding TOON format
#[derive(Debug, Clone, Default)]
pub struct DecodeOptions {
    /// Expected number of spaces per indentation level (default: 2)
    pub indent: Option<usize>,
    /// Enable strict validation (default: true)
    pub strict: Option<bool>,
}

impl DecodeOptions {
    /// Create new default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the expected indentation level
    pub fn indent(mut self, indent: usize) -> Self {
        self.indent = Some(indent);
        self
    }

    /// Set strict mode
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = Some(strict);
        self
    }

    /// Get the indentation, defaulting to 2
    pub fn get_indent(&self) -> usize {
        self.indent.unwrap_or(2)
    }

    /// Get strict mode, defaulting to true
    pub fn get_strict(&self) -> bool {
        self.strict.unwrap_or(true)
    }
}
