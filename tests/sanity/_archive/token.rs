use rsb::prelude::*;

#[test]
fn sanity_token_basic_operations() {
    // Test basic token operations
    let token = rsb::token::create_token("test_data");
    assert!(!token.is_empty());

    // Test token validation
    let is_valid = rsb::token::validate_token(&token, "test_data");
    assert!(is_valid);

    // Test invalid token
    let is_invalid = rsb::token::validate_token(&token, "wrong_data");
    assert!(!is_invalid);
}

#[test]
fn sanity_token_generation() {
    // Test token generation with different inputs
    let token1 = rsb::token::create_token("input1");
    let token2 = rsb::token::create_token("input2");
    let token3 = rsb::token::create_token("input1"); // Same as token1

    assert_ne!(token1, token2); // Different inputs should produce different tokens
    assert_eq!(token1, token3);  // Same inputs should produce same tokens
}

#[test]
fn sanity_token_timestamp() {
    // Test timestamped tokens
    let timestamped = rsb::token::create_timestamped_token("data");
    assert!(!timestamped.is_empty());

    // Should be longer than basic token due to timestamp
    let basic = rsb::token::create_token("data");
    assert!(timestamped.len() > basic.len());
}

#[test]
fn sanity_token_expiry() {
    // Test token expiry functionality
    let expires_in = std::time::Duration::from_secs(3600); // 1 hour
    let expiring_token = rsb::token::create_expiring_token("data", expires_in);

    assert!(!expiring_token.is_empty());

    // Test validation with expiry
    let is_valid = rsb::token::validate_expiring_token(&expiring_token, "data");
    assert!(is_valid); // Should be valid immediately

    // Test already expired token (very short expiry)
    let short_expires = std::time::Duration::from_nanos(1);
    let expired_token = rsb::token::create_expiring_token("data", short_expires);
    std::thread::sleep(std::time::Duration::from_millis(1));

    let is_expired = rsb::token::validate_expiring_token(&expired_token, "data");
    assert!(!is_expired); // Should be expired
}

#[test]
fn sanity_token_secure_generation() {
    // Test secure random token generation
    let secure1 = rsb::token::generate_secure_token(32);
    let secure2 = rsb::token::generate_secure_token(32);

    assert_eq!(secure1.len(), 32);
    assert_eq!(secure2.len(), 32);
    assert_ne!(secure1, secure2); // Should be different

    // Test different lengths
    let short_token = rsb::token::generate_secure_token(16);
    let long_token = rsb::token::generate_secure_token(64);

    assert_eq!(short_token.len(), 16);
    assert_eq!(long_token.len(), 64);
}

#[test]
fn sanity_token_encoding() {
    // Test token encoding/decoding
    let original_data = "test_payload_data";
    let encoded = rsb::token::encode_data(original_data);

    assert!(!encoded.is_empty());
    assert_ne!(encoded, original_data);

    let decoded = rsb::token::decode_data(&encoded);
    assert_eq!(decoded, original_data);
}

#[test]
fn sanity_token_session_management() {
    // Test session token management
    let session_token = rsb::token::create_session_token("user123");
    assert!(!session_token.is_empty());

    // Test session validation
    let is_valid_session = rsb::token::validate_session_token(&session_token, "user123");
    assert!(is_valid_session);

    // Test invalid session
    let is_invalid_session = rsb::token::validate_session_token(&session_token, "user456");
    assert!(!is_invalid_session);
}