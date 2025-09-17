//! ID generation implementation functions
//!
//! Core ID generation functions migrated from src/random.rs

/// Generates a new v4 UUID.
pub fn get_rand_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
