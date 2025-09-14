// Token Formatting and Display Functions

// FUNCTIONS TO MOVE FROM xstream color functions:

// color_token(token: &str, color: TokenColor) -> String
// - Apply color to token for display (non-stream related)
// - Example: color_token("identifier", TokenColor::Blue) -> "\033[34midentifier\033[0m"

// format_token_list(tokens: &[String], style: DisplayStyle) -> String
// - Format token list for display with consistent styling

// TOKEN FORMATTING FUNCTIONS:

// quote_token(value: &str, quote_char: char) -> String
// - Add quotes around token value
// - Example: quote_token("hello world", '"') -> "\"hello world\""

// escape_token(value: &str, escape_rules: &EscapeRules) -> String
// - Escape special characters in token
// - Example: escape_token("new\nline", &EscapeRules::default()) -> "new\\nline"

// join_tokens(tokens: &[String], separator: &str) -> String
// - Join tokens with specified separator
// - Example: join_tokens(&["a", "b", "c"], ", ") -> "a, b, c"

// pad_token(value: &str, width: usize, alignment: Alignment) -> String
// - Pad token to specified width with alignment
// - Example: pad_token("hi", 5, Alignment::Center) -> " hi  "

// DISPLAY FORMATTING:

// format_token_table(tokens: &[Token], columns: usize) -> String
// - Format tokens in table layout for display

// highlight_token(text: &str, token: &str, highlight_color: TokenColor) -> String
// - Highlight specific token in text with color

// format_token_with_type(token: &Token) -> String
// - Format token with its type information for debugging