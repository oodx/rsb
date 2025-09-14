// Token Parsing and Splitting Functions

// CORE PARSING FUNCTIONS:

// tokenize(text: &str, delimiters: &[char]) -> Vec<String>
// - Split text into tokens using specified delimiters
// - Handles multiple consecutive delimiters
// - Example: tokenize("a,b,,c", &[',']) -> vec!["a", "b", "c"]

// parse_delimited(text: &str, delimiter: char, escape_char: Option<char>) -> Vec<String>
// - Parse delimited text with optional escape character support
// - Example: parse_delimited("a,b\\,c,d", ',', Some('\\')) -> vec!["a", "b,c", "d"]

// parse_quoted(text: &str, quote_chars: &[char]) -> Result<Vec<String>, ParseError>
// - Parse quoted tokens, handling nested quotes and escaping
// - Example: parse_quoted("\"hello\" 'world'", &['"', '\'']) -> vec!["hello", "world"]

// parse_key_value(text: &str, separator: char) -> Result<(String, String), ParseError>
// - Parse key=value pairs with validation
// - Example: parse_key_value("key=value", '=') -> ("key", "value")

// tokenize_with_context(text: &str, rules: &ParseRules) -> Vec<Token>
// - Advanced tokenization with context awareness
// - Returns Token structs with position, type, and value

// WHITESPACE & NORMALIZATION:

// normalize_whitespace(tokens: &[String]) -> Vec<String>
// - Normalize whitespace in token list
// - Remove empty tokens, trim whitespace

// split_preserve_quotes(text: &str, delimiter: char) -> Vec<String>
// - Split while preserving quoted sections intact