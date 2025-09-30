//! Event System
//!
//! Simple event registry for subscribing to and emitting events.

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct EventData {
    pub event_type: String,
    pub data: HashMap<String, String>,
}

lazy_static! {
    // A registry for event handlers.
    pub static ref EVENT_HANDLERS: Arc<Mutex<HashMap<String, Vec<Box<dyn Fn(&EventData) + Send + Sync>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}
