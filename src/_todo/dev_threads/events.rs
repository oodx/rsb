// Event Handling (from macros/jobs_events.rs)

// MACRO TO MOVE FROM macros/jobs_events.rs:

// trap!(signal, handler) -> ()
// - Register signal handler for process events
// - Example: trap!(SIGINT, { cleanup(); exit(0); })

// trap!(signal, "command") -> ()
// - Register shell command as signal handler
// - Example: trap!(SIGTERM, "echo 'Terminating...' && cleanup.sh")

// event!(event_type, callback) -> EventHandle
// - Register callback for custom event types
// - Returns handle for event management
// - Example: event!("file_change", |path| { reload_config(path); })

// emit!(event_type, data) -> ()
// - Emit custom event with data
// - Triggers registered callbacks
// - Example: emit!("file_change", "/etc/config.toml")