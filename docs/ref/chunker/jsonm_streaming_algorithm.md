# JSONM Streaming JSON Forensic Analysis Algorithm

## Overview

The JSONM streaming algorithm performs character-by-character analysis of JSON data to extract forensic metadata, enable bounded-memory processing, and convert JSON structures into Meteor token streams. This approach enables processing of arbitrarily large JSON files (including terabyte-scale) with constant memory usage.

## Core Innovation

Unlike traditional JSON parsers that build complete object trees in memory, this algorithm:
- Analyzes JSON structure during streaming
- Extracts forensic metadata in real-time
- Maintains bounded memory usage regardless of file size
- Enables resumable processing with checkpoints
- Converts JSON to streamable Meteor format

## Algorithm Architecture

### Phase 1: Character Classification Pipeline

#### Character Type Detection
```
Input: character stream
Output: classified character types with pattern continuity

For each character c:
  classify_type = determine_character_type(c)
  if classify_type == previous_type:
    extend_current_pattern()
  else:
    complete_current_pattern()
    start_new_pattern(classify_type)
```

#### Character Classifications
- **Structural**: `{}[]:"`,
- **Alphanumeric**: `a-zA-Z0-9`
- **Numeric**: `0-9.-+eE`
- **Hexadecimal**: `0-9a-fA-F`
- **Whitespace**: ` \t\n\r`
- **Special**: Other characters

#### Pattern Buffer Management
```
pattern_buffer = {
  type: character_classification,
  content: accumulated_characters,
  start_position: byte_offset,
  end_position: byte_offset,
  context: current_parsing_context
}
```

### Phase 2: Structural Analysis

#### Bracket Counting & Depth Tracking
```
structural_state = {
  depth: 0,
  balance: 0,
  max_depth: 0,
  context_stack: []
}

process_structural_character(char):
  if char in ['{', '[']:
    depth++, balance++
    context_stack.push(get_context_type(char))
    max_depth = max(max_depth, depth)
  elif char in ['}', ']']:
    depth--, balance--
    context_stack.pop()
  
  # Validation
  if balance < 0:
    throw_error("Malformed JSON: unmatched closing bracket")
```

#### Context Path Building
```
build_context_path():
  path = []
  for context in context_stack:
    if context.type == "object":
      path.append(context.current_key)
    elif context.type == "array":
      path.append(context.current_index)
  return ".".join(path)
```

### Phase 3: Key-Value Extraction

#### Key Detection & Dictionary Building
```
key_dictionary = {
  "key_name": {
    id: auto_increment_id,
    frequency: occurrence_count,
    positions: [byte_positions],
    contexts: [context_paths],
    value_types: [inferred_types]
  }
}

process_key(key_string, position, context):
  if key_string not in key_dictionary:
    key_dictionary[key_string] = create_key_entry()
  
  key_dictionary[key_string].frequency++
  key_dictionary[key_string].positions.append(position)
  key_dictionary[key_string].contexts.append(context)
```

#### Value Type Inference
```
infer_value_type(pattern_buffer):
  content = pattern_buffer.content
  
  if all_numeric(content):
    return "number"
  elif all_hex(content) and length_consistent():
    return "hash" 
  elif all_alphanumeric(content) and short_length():
    return "identifier"
  elif mixed_patterns(content):
    return "text"
  else:
    return "unknown"
```

### Phase 4: Forensic Metadata Collection

#### Statistical Analysis
```
forensic_stats = {
  total_keys: unique_key_count,
  key_frequencies: sorted_frequency_distribution,
  depth_distribution: depth_level_counts,
  structure_patterns: detected_structural_patterns,
  anomalies: suspicious_patterns,
  size_estimates: {
    total_bytes: processed_byte_count,
    estimated_objects: object_count_estimate,
    estimated_arrays: array_count_estimate
  }
}
```

#### Anomaly Detection
```
detect_anomalies():
  # Unusual nesting depths
  if max_depth > DEPTH_THRESHOLD:
    flag_anomaly("excessive_nesting", max_depth)
  
  # Suspicious key patterns
  for key, stats in key_dictionary:
    if stats.frequency == 1 and key_looks_encoded(key):
      flag_anomaly("potential_encoded_key", key)
    
  # Value pattern irregularities  
  if detect_pattern_breaks(value_types):
    flag_anomaly("inconsistent_value_patterns")
```

### Phase 5: Meteor Token Generation

#### Context & Namespace Assignment
```
meteor_context = determine_context(current_structure)
meteor_namespace = generate_namespace(context_path)

meteor_stream = {
  context: meteor_context,
  namespace: meteor_namespace, 
  tokens: []
}
```

#### Token Stream Creation
```
generate_meteor_tokens():
  for key, value in processed_key_values:
    full_path = build_full_path(context_stack, key)
    token = f"{full_path}={value}"
    meteor_stream.tokens.append(token)
  
  return format_meteor_stream(meteor_stream)

format_meteor_stream(stream):
  return f"context:{stream.context};namespace:{stream.namespace};" + 
         ";".join(stream.tokens) + ";"
```

### Phase 6: Memory Management

#### Bounded Memory Strategy
```
memory_threshold_strategy():
  if metadata_size < 5MB:
    # Fast path: keep everything in memory
    store_in_memory(forensic_data)
  elif metadata_size < 50MB:
    # Hybrid: hot data in memory, cold data persisted
    store_hot_data_memory(recent_forensic_data)
    persist_cold_data(older_forensic_data)
  else:
    # Full streaming: minimal memory, everything persisted
    stream_to_storage(forensic_data)
    keep_minimal_working_set()
```

#### Checkpoint System
```
create_checkpoint():
  checkpoint = {
    byte_position: current_stream_position,
    parsing_state: current_parser_state,
    structural_state: bracket_balance_state,
    context_stack: current_context_stack,
    key_dictionary: current_key_stats,
    forensic_metadata: accumulated_stats
  }
  persist_checkpoint(checkpoint)

resume_from_checkpoint(checkpoint):
  restore_parser_state(checkpoint.parsing_state)
  restore_structural_state(checkpoint.structural_state)
  restore_context_stack(checkpoint.context_stack)
  seek_to_position(checkpoint.byte_position)
```

## JSONM Format Specification

### Structure
```
JSONM Document := Context Declaration + Namespace Declaration + Token Stream

Context Declaration := "context:" + context_type + ";"
Namespace Declaration := "namespace:" + namespace_path + ";"
Token Stream := (Key-Value Token ";")*

Key-Value Token := Key "=" Value
Key := Path | Simple_Key
Path := Simple_Key ("." Simple_Key)*
Value := String | Number | Boolean | Null
```

### Array Indexing
```
# Bracket notation (flattened during processing)
users[0].name=Alice;users[1].name=Bob;

# Dunder notation (equivalent internal representation)  
users__0__name=Alice;users__1__name=Bob;
```

### Example JSONM Output
```
context:object;namespace:api_response;
status=success;
users.0.name=Alice;users.0.age=25;users.0.city=Boston;
users.1.name=Bob;users.1.age=30;users.1.city=NYC;
metadata.total_count=2;metadata.timestamp=1640995200;
```

## Licensing Strategy

### Three-Tier Model
1. **AGPL License**: True open source contributions required
2. **Community Edition**: Academic/research/personal use
3. **Commercial License**: Any commercial use, including individual consultants

### Key Restrictions
- **One-degree usage rule**: Commercial licensees cannot sublicense the technology
- **Users can use applications** built with the technology
- **Users cannot redistribute** the underlying technology
- **Prevents middleman reselling** of core functionality

## Performance Characteristics

### Memory Usage
- **Constant memory footprint** regardless of input size
- **Configurable thresholds** for different storage strategies
- **Streaming forensic data** to external storage when needed

### Processing Speed
- **Single-pass analysis** combines multiple operations
- **Character-level optimization** with pattern detection
- **SIMD potential** for character classification
- **No object tree construction** overhead

### Scalability
- **Terabyte-scale processing** on single machines
- **Resumable operations** for very large files
- **Bounded resource usage** prevents memory exhaustion
- **Real-time insights** before processing completion

## Market Applications

### Security & Forensics
- JSON payload threat analysis
- API traffic anomaly detection
- Data exfiltration pattern recognition
- Compliance audit trails

### Data Processing
- Large-scale JSON transformation
- Memory-efficient data migration
- Real-time stream processing
- Format conversion pipelines

### Developer Tools
- JSON structure analysis
- Performance optimization insights
- Data quality assessment
- Schema evolution tracking

## Implementation Notes

### Critical Design Decisions
1. **Character-by-character processing** enables true streaming
2. **Forensic metadata extraction** provides unprecedented visibility
3. **Meteor token format** enables efficient data representation
4. **Bounded memory model** ensures scalability
5. **Resume capability** supports reliability for large datasets

### Technical Advantages
- **No preprocessing required** for arbitrarily large files
- **No distributed infrastructure** needed for terabyte processing
- **No format conversion** required before processing
- **No memory limitations** based on input size
- **No structural assumptions** about JSON content

This algorithm represents a fundamental advance in JSON processing, enabling previously impossible operations on massive datasets while providing rich forensic insights into data structure and content patterns.