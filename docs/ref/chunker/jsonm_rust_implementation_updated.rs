use std::collections::{HashMap, VecDeque};
use std::io::{Read, BufReader};
use std::fs::File;

// JSON parsing states for proper state machine
#[derive(Debug, Clone, PartialEq)]
enum JsonState {
    ExpectValue,           // Expecting any JSON value
    ExpectKey,             // Expecting object key (after { or ,)
    ExpectColon,           // Expecting : after key
    ExpectValueAfterColon, // Expecting value after :
    ExpectCommaOrClose,    // Expecting , or } or ]
    InString,              // Currently parsing string content
    InNumber,              // Currently parsing number
    InTrue,                // Parsing "true"
    InFalse,               // Parsing "false"
    InNull,                // Parsing "null"
}

// Character classification with context awareness
#[derive(Debug, Clone, PartialEq)]
enum CharType {
    Structural,     // {}[]:",
    StringDelim,    // "
    Digit,          // 0-9
    Letter,         // a-zA-Z
    Sign,           // +-
    Decimal,        // .
    Exponent,       // eE
    Whitespace,     // \s\t\n\r
    Escape,         // \
    Other,          // Everything else
}

// Enhanced pattern buffer with JSON context
#[derive(Debug, Clone)]
struct PatternBuffer {
    char_type: CharType,
    content: String,
    start_position: u64,
    end_position: u64,
    json_state: JsonState,
    escaped: bool,
}

// Proper structural state with bracket matching
#[derive(Debug, Clone)]
struct StructuralState {
    depth: i32,
    balance: i32,
    max_depth: i32,
    context_stack: Vec<ContextFrame>,
    current_state: JsonState,
    bracket_stack: Vec<char>, // Track actual bracket types for matching
}

#[derive(Debug, Clone)]
enum ContextFrame {
    Object { 
        current_key: Option<String>,
        expecting_key: bool,
    },
    Array { 
        current_index: usize,
    },
}

// Enhanced key statistics
#[derive(Debug, Clone)]
struct KeyStats {
    id: u32,
    frequency: u32,
    positions: Vec<u64>,
    contexts: Vec<String>,
    value_types: Vec<InferredType>,
    first_seen: u64,
    last_seen: u64,
}

#[derive(Debug, Clone)]
enum InferredType {
    String,
    Number,
    Boolean,
    Null,
    Object,
    Array,
    Hash,       // Hex pattern
    Identifier, // Short alphanumeric
}

// Forensic metadata with better anomaly tracking
#[derive(Debug, Clone)]
struct ForensicStats {
    total_keys: u32,
    total_values: u32,
    key_frequencies: HashMap<String, u32>,
    depth_distribution: HashMap<i32, u32>,
    type_distribution: HashMap<String, u32>,
    anomalies: Vec<Anomaly>,
    size_estimates: SizeEstimates,
    parsing_errors: Vec<ParseError>,
}

#[derive(Debug, Clone)]
struct Anomaly {
    anomaly_type: AnomalyType,
    description: String,
    position: u64,
    severity: Severity,
}

#[derive(Debug, Clone)]
enum AnomalyType {
    ExcessiveNesting,
    LargeToken,
    SuspiciousPattern,
    UnusualFrequency,
    StructuralIrregularity,
}

#[derive(Debug, Clone)]
enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
struct ParseError {
    error_type: String,
    position: u64,
    expected: String,
    found: String,
}

#[derive(Debug, Clone)]
struct SizeEstimates {
    total_bytes: u64,
    estimated_objects: u32,
    estimated_arrays: u32,
    estimated_strings: u32,
    estimated_numbers: u32,
}

// Meteor token with proper path construction
#[derive(Debug, Clone)]
struct MeteorToken {
    context: String,
    namespace: String,
    key: String,
    value: String,
    position: u64,
}

// Enhanced checkpoint with full state
#[derive(Debug, Clone)]
struct Checkpoint {
    byte_position: u64,
    structural_state: StructuralState,
    current_buffer: Option<PatternBuffer>,
    key_dictionary: HashMap<String, KeyStats>,
    forensic_stats: ForensicStats,
    partial_tokens: Vec<MeteorToken>,
}

// Main processor with corrected logic
pub struct JsonmProcessor {
    // Core state
    current_position: u64,
    structural_state: StructuralState,
    pattern_buffer: Option<PatternBuffer>,
    
    // JSON parsing state
    current_key: Option<String>,
    current_path: Vec<String>,
    
    // Data collection
    key_dictionary: HashMap<String, KeyStats>,
    forensic_stats: ForensicStats,
    meteor_tokens: Vec<MeteorToken>,
    next_key_id: u32,
    
    // Memory management
    memory_threshold: usize,
    processing_mode: ProcessingMode,
}

#[derive(Debug)]
enum ProcessingMode {
    InMemory,
    Hybrid,
    Streaming,
}

impl JsonmProcessor {
    pub fn new() -> Self {
        Self {
            current_position: 0,
            structural_state: StructuralState {
                depth: 0,
                balance: 0,
                max_depth: 0,
                context_stack: Vec::new(),
                current_state: JsonState::ExpectValue,
                bracket_stack: Vec::new(),
            },
            pattern_buffer: None,
            current_key: None,
            current_path: Vec::new(),
            key_dictionary: HashMap::new(),
            forensic_stats: ForensicStats {
                total_keys: 0,
                total_values: 0,
                key_frequencies: HashMap::new(),
                depth_distribution: HashMap::new(),
                type_distribution: HashMap::new(),
                anomalies: Vec::new(),
                size_estimates: SizeEstimates {
                    total_bytes: 0,
                    estimated_objects: 0,
                    estimated_arrays: 0,
                    estimated_strings: 0,
                    estimated_numbers: 0,
                },
                parsing_errors: Vec::new(),
            },
            meteor_tokens: Vec::new(),
            next_key_id: 1,
            memory_threshold: 5 * 1024 * 1024,
            processing_mode: ProcessingMode::InMemory,
        }
    }
    
    pub fn process_stream<R: Read>(&mut self, reader: R) -> Result<Vec<MeteorToken>, Box<dyn std::error::Error>> {
        let mut buf_reader = BufReader::new(reader);
        let mut buffer = [0; 8192];
        
        loop {
            match buf_reader.read(&mut buffer)? {
                0 => break,
                n => {
                    for &byte in &buffer[..n] {
                        self.process_character(byte as char)?;
                        self.current_position += 1;
                        
                        // Update size estimates
                        self.forensic_stats.size_estimates.total_bytes += 1;
                    }
                }
            }
            
            self.check_memory_pressure();
        }
        
        self.finalize_processing()
    }
    
    fn process_character(&mut self, ch: char) -> Result<(), Box<dyn std::error::Error>> {
        let char_type = self.classify_character(ch);
        
        // Handle string escaping first
        if let Some(ref mut buffer) = &mut self.pattern_buffer {
            if buffer.json_state == JsonState::InString {
                if buffer.escaped {
                    // Handle escaped character
                    buffer.content.push(ch);
                    buffer.escaped = false;
                    buffer.end_position = self.current_position;
                    return Ok(());
                } else if ch == '\\' {
                    buffer.escaped = true;
                    buffer.content.push(ch);
                    buffer.end_position = self.current_position;
                    return Ok(());
                } else if ch == '"' {
                    // End of string
                    buffer.content.push(ch);
                    buffer.end_position = self.current_position;
                    let completed = self.pattern_buffer.take().unwrap();
                    self.process_completed_string(completed)?;
                    return Ok(());
                }
            }
        }
        
        // Handle whitespace - skip but track
        if char_type == CharType::Whitespace {
            return Ok(());
        }
        
        // State machine transitions
        match self.structural_state.current_state {
            JsonState::ExpectValue => self.handle_expect_value(ch, char_type)?,
            JsonState::ExpectKey => self.handle_expect_key(ch, char_type)?,
            JsonState::ExpectColon => self.handle_expect_colon(ch, char_type)?,
            JsonState::ExpectValueAfterColon => self.handle_expect_value_after_colon(ch, char_type)?,
            JsonState::ExpectCommaOrClose => self.handle_expect_comma_or_close(ch, char_type)?,
            JsonState::InString => {
                // Should be handled above, but fallback
                if let Some(ref mut buffer) = &mut self.pattern_buffer {
                    buffer.content.push(ch);
                    buffer.end_position = self.current_position;
                }
            },
            JsonState::InNumber => self.handle_in_number(ch, char_type)?,
            JsonState::InTrue | JsonState::InFalse | JsonState::InNull => {
                self.handle_literal_continuation(ch)?;
            },
        }
        
        Ok(())
    }
    
    fn classify_character(&self, ch: char) -> CharType {
        match ch {
            '{' | '}' | '[' | ']' | ':' | ',' => CharType::Structural,
            '"' => CharType::StringDelim,
            '0'..='9' => CharType::Digit,
            'a'..='z' | 'A'..='Z' => CharType::Letter,
            '+' | '-' => CharType::Sign,
            '.' => CharType::Decimal,
            'e' | 'E' => CharType::Exponent,
            ' ' | '\t' | '\n' | '\r' => CharType::Whitespace,
            '\\' => CharType::Escape,
            _ => CharType::Other,
        }
    }
    
    fn handle_expect_value(&mut self, ch: char, char_type: CharType) -> Result<(), Box<dyn std::error::Error>> {
        match char_type {
            CharType::StringDelim => {
                self.start_string_pattern(ch);
                self.structural_state.current_state = JsonState::InString;
            },
            CharType::Digit | CharType::Sign => {
                self.start_number_pattern(ch);
                self.structural_state.current_state = JsonState::InNumber;
            },
            CharType::Structural => {
                match ch {
                    '{' => {
                        self.handle_object_start()?;
                        self.structural_state.current_state = JsonState::ExpectKey;
                    },
                    '[' => {
                        self.handle_array_start()?;
                        self.structural_state.current_state = JsonState::ExpectValue;
                    },
                    _ => return Err(format!("Unexpected structural character '{}' when expecting value", ch).into()),
                }
            },
            CharType::Letter => {
                // Check for true, false, null
                match ch {
                    't' => {
                        self.start_literal_pattern(ch, JsonState::InTrue);
                        self.structural_state.current_state = JsonState::InTrue;
                    },
                    'f' => {
                        self.start_literal_pattern(ch, JsonState::InFalse);
                        self.structural_state.current_state = JsonState::InFalse;
                    },
                    'n' => {
                        self.start_literal_pattern(ch, JsonState::InNull);
                        self.structural_state.current_state = JsonState::InNull;
                    },
                    _ => return Err(format!("Unexpected character '{}' when expecting value", ch).into()),
                }
            },
            _ => return Err(format!("Unexpected character '{}' when expecting value", ch).into()),
        }
        Ok(())
    }
    
    fn handle_expect_key(&mut self, ch: char, char_type: CharType) -> Result<(), Box<dyn std::error::Error>> {
        match char_type {
            CharType::StringDelim => {
                self.start_string_pattern(ch);
                self.structural_state.current_state = JsonState::InString;
            },
            CharType::Structural if ch == '}' => {
                // Empty object
                self.handle_object_end()?;
                self.structural_state.current_state = JsonState::ExpectCommaOrClose;
            },
            _ => return Err(format!("Expected string key or '}}', found '{}'", ch).into()),
        }
        Ok(())
    }
    
    fn handle_expect_colon(&mut self, ch: char, char_type: CharType) -> Result<(), Box<dyn std::error::Error>> {
        if char_type == CharType::Structural && ch == ':' {
            self.structural_state.current_state = JsonState::ExpectValueAfterColon;
            Ok(())
        } else {
            Err(format!("Expected ':', found '{}'", ch).into())
        }
    }
    
    fn handle_expect_value_after_colon(&mut self, ch: char, char_type: CharType) -> Result<(), Box<dyn std::error::Error>> {
        // Same as handle_expect_value but tracks that we're after a colon
        self.handle_expect_value(ch, char_type)
    }
    
    fn handle_expect_comma_or_close(&mut self, ch: char, char_type: CharType) -> Result<(), Box<dyn std::error::Error>> {
        if char_type == CharType::Structural {
            match ch {
                ',' => {
                    // Continue with next key/value
                    if let Some(frame) = self.structural_state.context_stack.last() {
                        match frame {
                            ContextFrame::Object { .. } => {
                                self.structural_state.current_state = JsonState::ExpectKey;
                            },
                            ContextFrame::Array { .. } => {
                                self.structural_state.current_state = JsonState::ExpectValue;
                            },
                        }
                    }
                },
                '}' => {
                    self.handle_object_end()?;
                    self.structural_state.current_state = JsonState::ExpectCommaOrClose;
                },
                ']' => {
                    self.handle_array_end()?;
                    self.structural_state.current_state = JsonState::ExpectCommaOrClose;
                },
                _ => return Err(format!("Expected ',', '}}', or ']', found '{}'", ch).into()),
            }
        } else {
            return Err(format!("Expected ',', '}}', or ']', found '{}'", ch).into());
        }
        Ok(())
    }
    
    fn handle_in_number(&mut self, ch: char, char_type: CharType) -> Result<(), Box<dyn std::error::Error>> {
        match char_type {
            CharType::Digit | CharType::Decimal | CharType::Exponent | CharType::Sign => {
                if let Some(ref mut buffer) = &mut self.pattern_buffer {
                    buffer.content.push(ch);
                    buffer.end_position = self.current_position;
                }
            },
            CharType::Structural => {
                // End of number
                if let Some(completed) = self.pattern_buffer.take() {
                    self.process_completed_number(completed)?;
                }
                // Process the structural character
                self.handle_expect_comma_or_close(ch, char_type)?;
            },
            _ => return Err(format!("Invalid character '{}' in number", ch).into()),
        }
        Ok(())
    }
    
    fn handle_literal_continuation(&mut self, ch: char) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut buffer) = &mut self.pattern_buffer {
            buffer.content.push(ch);
            buffer.end_position = self.current_position;
            
            // Check if literal is complete
            let expected = match buffer.json_state {
                JsonState::InTrue => "true",
                JsonState::InFalse => "false", 
                JsonState::InNull => "null",
                _ => return Err("Invalid literal state".into()),
            };
            
            if buffer.content == expected {
                let completed = self.pattern_buffer.take().unwrap();
                self.process_completed_literal(completed)?;
                self.structural_state.current_state = JsonState::ExpectCommaOrClose;
            } else if !expected.starts_with(&buffer.content) {
                return Err(format!("Invalid literal: {}", buffer.content).into());
            }
        }
        Ok(())
    }
    
    fn start_string_pattern(&mut self, ch: char) {
        self.pattern_buffer = Some(PatternBuffer {
            char_type: CharType::StringDelim,
            content: String::from(ch),
            start_position: self.current_position,
            end_position: self.current_position,
            json_state: JsonState::InString,
            escaped: false,
        });
    }
    
    fn start_number_pattern(&mut self, ch: char) {
        self.pattern_buffer = Some(PatternBuffer {
            char_type: CharType::Digit,
            content: String::from(ch),
            start_position: self.current_position,
            end_position: self.current_position,
            json_state: JsonState::InNumber,
            escaped: false,
        });
    }
    
    fn start_literal_pattern(&mut self, ch: char, state: JsonState) {
        self.pattern_buffer = Some(PatternBuffer {
            char_type: CharType::Letter,
            content: String::from(ch),
            start_position: self.current_position,
            end_position: self.current_position,
            json_state: state,
            escaped: false,
        });
    }
    
    fn handle_object_start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.structural_state.depth += 1;
        self.structural_state.balance += 1;
        self.structural_state.max_depth = self.structural_state.max_depth.max(self.structural_state.depth);
        self.structural_state.bracket_stack.push('{');
        self.structural_state.context_stack.push(ContextFrame::Object {
            current_key: None,
            expecting_key: true,
        });
        self.forensic_stats.size_estimates.estimated_objects += 1;
        Ok(())
    }
    
    fn handle_object_end(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(bracket) = self.structural_state.bracket_stack.pop() {
            if bracket != '{' {
                return Err(format!("Bracket mismatch: expected '{{', found '{}'", bracket).into());
            }
        } else {
            return Err("Unmatched closing brace".into());
        }
        
        self.structural_state.depth -= 1;
        self.structural_state.balance -= 1;
        self.structural_state.context_stack.pop();
        
        if !self.current_path.is_empty() {
            self.current_path.pop();
        }
        
        Ok(())
    }
    
    fn handle_array_start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.structural_state.depth += 1;
        self.structural_state.balance += 1;
        self.structural_state.max_depth = self.structural_state.max_depth.max(self.structural_state.depth);
        self.structural_state.bracket_stack.push('[');
        self.structural_state.context_stack.push(ContextFrame::Array {
            current_index: 0,
        });
        self.forensic_stats.size_estimates.estimated_arrays += 1;
        Ok(())
    }
    
    fn handle_array_end(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(bracket) = self.structural_state.bracket_stack.pop() {
            if bracket != '[' {
                return Err(format!("Bracket mismatch: expected '[', found '{}'", bracket).into());
            }
        } else {
            return Err("Unmatched closing bracket".into());
        }
        
        self.structural_state.depth -= 1;
        self.structural_state.balance -= 1;
        self.structural_state.context_stack.pop();
        
        if !self.current_path.is_empty() {
            self.current_path.pop();
        }
        
        Ok(())
    }
    
    fn process_completed_string(&mut self, pattern: PatternBuffer) -> Result<(), Box<dyn std::error::Error>> {
        let content = pattern.content.trim_matches('"').to_string();
        
        // Determine if this is a key or value based on context
        if let Some(ContextFrame::Object { expecting_key, .. }) = self.structural_state.context_stack.last() {
            if *expecting_key {
                // This is a key
                self.current_key = Some(content.clone());
                self.structural_state.current_state = JsonState::ExpectColon;
                
                // Update context frame
                if let Some(ContextFrame::Object { current_key, expecting_key }) = self.structural_state.context_stack.last_mut() {
                    *current_key = Some(content);
                    *expecting_key = false;
                }
            } else {
                // This is a value
                self.process_key_value_pair(content, InferredType::String)?;
                self.structural_state.current_state = JsonState::ExpectCommaOrClose;
            }
        } else {
            // String value in array or root
            self.process_array_value(content, InferredType::String)?;
            self.structural_state.current_state = JsonState::ExpectCommaOrClose;
        }
        
        self.forensic_stats.size_estimates.estimated_strings += 1;
        Ok(())
    }
    
    fn process_completed_number(&mut self, pattern: PatternBuffer) -> Result<(), Box<dyn std::error::Error>> {
        let content = pattern.content;
        
        if let Some(ContextFrame::Object { expecting_key: false, .. }) = self.structural_state.context_stack.last() {
            self.process_key_value_pair(content, InferredType::Number)?;
        } else {
            self.process_array_value(content, InferredType::Number)?;
        }
        
        self.forensic_stats.size_estimates.estimated_numbers += 1;
        Ok(())
    }
    
    fn process_completed_literal(&mut self, pattern: PatternBuffer) -> Result<(), Box<dyn std::error::Error>> {
        let content = pattern.content;
        let inferred_type = match content.as_str() {
            "true" | "false" => InferredType::Boolean,
            "null" => InferredType::Null,
            _ => return Err(format!("Invalid literal: {}", content).into()),
        };
        
        if let Some(ContextFrame::Object { expecting_key: false, .. }) = self.structural_state.context_stack.last() {
            self.process_key_value_pair(content, inferred_type)?;
        } else {
            self.process_array_value(content, inferred_type)?;
        }
        
        Ok(())
    }
    
    fn process_key_value_pair(&mut self, value: String, value_type: InferredType) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(key) = &self.current_key {
            let full_path = self.build_current_path(Some(key));
            
            // Create meteor token
            let token = MeteorToken {
                context: "app".to_string(),
                namespace: "jsonm_parsed".to_string(),
                key: full_path.clone(),
                value: value.clone(),
                position: self.current_position,
            };
            
            self.meteor_tokens.push(token);
            
            // Update statistics
            self.update_key_statistics(key, &full_path, &value, value_type);
            
            // Clear current key
            self.current_key = None;
            
            self.forensic_stats.total_values += 1;
        }
        
        Ok(())
    }
    
    fn process_array_value(&mut self, value: String, value_type: InferredType) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ContextFrame::Array { current_index }) = self.structural_state.context_stack.last_mut() {
            let index_key = format!("__i_{}", current_index);
            let full_path = self.build_current_path(Some(&index_key));
            
            let token = MeteorToken {
                context: "app".to_string(),
                namespace: "jsonm_parsed".to_string(),
                key: full_path.clone(),
                value: value.clone(),
                position: self.current_position,
            };
            
            self.meteor_tokens.push(token);
            *current_index += 1;
            
            self.forensic_stats.total_values += 1;
        }
        
        Ok(())
    }
    
    fn build_current_path(&self, key: Option<&str>) -> String {
        let mut path_parts = Vec::new();
        
        for frame in &self.structural_state.context_stack {
            match frame {
                ContextFrame::Object { current_key: Some(obj_key), .. } => {
                    path_parts.push(obj_key.clone());
                },
                ContextFrame::Array { current_index } => {
                    path_parts.push(format!("__i_{}", current_index));
                },
                _ => {},
            }
        }
        
        if let Some(k) = key {
            path_parts.push(k.to_string());
        }
        
        path_parts.join(".")
    }
    
    fn update_key_statistics(&mut self, key: &str, full_path: &str, value: &str, value_type: InferredType) {
        let stats = self.key_dictionary.entry(key.to_string()).or_insert_with(|| {
            let id = self.next_key_id;
            self.next_key_id += 1;
            self.forensic_stats.total_keys += 1;
            
            KeyStats {
                id,
                frequency: 0,
                positions: Vec::new(),
                contexts: Vec::new(),
                value_types: Vec::new(),
                first_seen: self.current_position,
                last_seen: self.current_position,
            }
        });
        
        stats.frequency += 1;
        stats.positions.push(self.current_position);
        stats.contexts.push(full_path.to_string());
        stats.value_types.push(value_type);
        stats.last_seen = self.current_position;
        
        // Update frequency map
        *self.forensic_stats.key_frequencies.entry(key.to_string()).or_insert(0) += 1;
        
        // Anomaly detection
        self.detect_value_anomalies(key, value);
    }
    
    fn detect_value_anomalies(&mut self, key: &str, value: &str) {
        // Large value detection
        if value.len() > 1000 {
            self.forensic_stats.anomalies.push(Anomaly {
                anomaly_type: AnomalyType::LargeToken,
                description: format!("Large value for key '{}': {} characters", key, value.len()),
                position: self.current_position,
                severity: Severity::Medium,
            });
        }
        
        // Excessive nesting detection
        if self.structural_state.depth > 20 {
            self.forensic_stats.anomalies.push(Anomaly {
                anomaly_type: AnomalyType::ExcessiveNesting,
                description: format!("Deep nesting level: {}", self.structural_state.depth),
                position: self.current_position,
                severity: Severity::High,
            });
        }
    }
    
    fn check_memory_pressure(&mut self) {
        let estimated_size = 
            self.key_dictionary.len() * 200 + 
            self.meteor_tokens.len() * 150 +
            self.forensic_stats.anomalies.len() * 100;
            
        if estimated_size > self.memory_threshold * 10 {
            self.processing_mode = ProcessingMode::Streaming;
            // In real implementation, would stream data to external storage
        } else if estimated_size > self.memory_threshold {
            self.processing_mode = ProcessingMode::Hybrid;
        }
    }
    
    fn finalize_processing(&mut self) -> Result<Vec<MeteorToken>, Box<dyn std::error::Error>> {
        // Validate final state
        if self.structural_state.balance != 0 {
            return Err("Unbalanced JSON structure".into());
        }
        
        if !self.structural_state.bracket_stack.is_empty() {
            return Err("Unclosed brackets in JSON".into());
        }
        
        Ok(self.meteor_tokens.clone())
    }
    
    pub fn get_forensic_stats(&self) -> &ForensicStats {
        &self.forensic_stats
    }
    
    pub fn create_checkpoint(&self) -> Checkpoint {
        Checkpoint {
            byte_position: self.current_position,
            structural_state: self.structural_state.clone(),
            current_buffer: self.pattern_buffer.clone(),
            key_dictionary: self.key_dictionary.clone(),
            forensic_stats: self.forensic_stats.clone(),
            partial_tokens: self.meteor_tokens.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_simple_object() {
        let json = r#"{"name":"Alice","age":25}"#;
        let mut processor = JsonmProcessor::new();
        let cursor = Cursor::new(json.as_bytes());
        
        let result = processor.process_stream(cursor);
        assert!(result.is_ok());
        
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 2);
        
        // Check tokens
        assert!(tokens.iter().any(|t| t.key == "name" && t.value == "Alice"));
        assert!(tokens.iter().any(|t| t.key == "age" && t.value == "25"));
    }
    
    #[test]
    fn test_nested_structure() {
        let json = r#"{"user":{"name":"Bob","details":{"age":30}}}"#;
        let mut processor = JsonmProcessor::new();
        let cursor = Cursor::new(json.as_bytes());
        
        let result = processor.process_stream(cursor);
        assert!(result.is_ok());
        
        let tokens = result.unwrap();
        assert!(tokens.iter().any(|t| t.key == "user.name" && t.value == "Bob"));
        assert!(tokens.iter().any(|t| t.key == "user.details.age" && t.value == "30"));
    }
    
    #[test]
    fn test_array_handling() {
        let json = r#"{"items":["a","b","c"]}"#;
        let mut processor = JsonmProcessor::new();
        let cursor = Cursor::new(json.as_bytes());
        
        let result = processor.process_stream(cursor);
        assert!(result.is_ok());
        
        let tokens = result.unwrap();
        assert!(tokens.iter().any(|t| t.key == "items.__i_0" && t.value == "a"));
        assert!(tokens.iter().any(|t| t.key == "items.__i_1" && t.value == "b"));
        assert!(tokens.iter().any(|t| t.key == "items.__i_2" && t.value == "c"));
    }
    
    #[test]
    fn test_invalid_json() {
        let json = r#"{"name":"Alice","age":}"#; // Missing value
        let mut processor = JsonmProcessor::new();
        let cursor = Cursor::new(json.as_bytes());
        
        let result = processor.process_stream(cursor);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_bracket_mismatch() {
        let json = r#"{"name":"Alice"]"#; // Wrong closing bracket
        let mut processor = JsonmProcessor::new();
        let cursor = Cursor::new(json.as_bytes());
        
        let result = processor.process_stream(cursor);
        assert!(result.is_err());
    }
}