// Token Type Definitions and Classification

// TYPES TO MOVE FROM xstream/src/xstream/types.rs:

// Core token type definitions (excluding streamable.rs functionality)
// Generic token classification and validation types

// TOKEN TYPE CLASSIFICATIONS:

// #[derive(Debug, Clone, PartialEq)]
// pub enum TokenType {
//     Identifier,       // Variable names, function names
//     Literal,          // String literals, numbers
//     Operator,         // +, -, =, ==, etc.
//     Delimiter,        // (, ), [, ], {, }
//     Separator,        // ,, ;, :
//     Keyword,          // Reserved language keywords
//     Comment,          // // or /* */ style comments
//     Whitespace,       // Spaces, tabs, newlines
//     Unknown,          // Unclassified tokens
// }

// #[derive(Debug, Clone)]
// pub struct Token {
//     pub value: String,
//     pub token_type: TokenType,
//     pub position: Position,
//     pub length: usize,
// }

// #[derive(Debug, Clone)]
// pub struct Position {
//     pub line: usize,
//     pub column: usize,
//     pub offset: usize,
// }

// CLASSIFICATION FUNCTIONS:

// classify_token(value: &str, context: &TokenContext) -> TokenType
// - Classify token based on its value and context
// - Example: classify_token("123", &context) -> TokenType::Literal

// is_token_type(token: &Token, expected_type: TokenType) -> bool
// - Check if token matches expected type

// TOKEN RULES AND CONFIGURATION:

// #[derive(Debug, Clone)]
// pub struct TokenRules {
//     pub min_length: Option<usize>,
//     pub max_length: Option<usize>,
//     pub allowed_chars: Option<HashSet<char>>,
//     pub forbidden_chars: Option<HashSet<char>>,
//     pub patterns: Vec<Regex>,
// }

// #[derive(Debug)]
// pub enum ValidationError {
//     TooShort(usize, usize),    // found, min
//     TooLong(usize, usize),     // found, max
//     InvalidChar(char),
//     PatternMismatch(String),
// }