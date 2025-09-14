//! Threads: job control, event, and trap macros (module-owned)

#[macro_export]
macro_rules! job {
    (background: $command:expr) => {{
        $crate::threads::start_background($command)
    }};
    (wait: $job_id:expr) => {{
        match $crate::threads::wait($job_id, None) {
            Ok(status) => status,
            Err(e) => { $crate::error!("Failed to wait for job {}: {}", $job_id, e); -1 }
        }
    }};
    (timeout: $timeout:expr, wait: $job_id:expr) => {{
        match $crate::threads::wait($job_id, Some($timeout)) {
            Ok(status) => status,
            Err(e) => { $crate::error!("Failed to wait for job {}: {}", $job_id, e); -1 }
        }
    }};
    (list) => {{
        let jobs = $crate::threads::list_jobs();
        if jobs.is_empty() { $crate::info!("No running jobs."); }
        for (id, cmd) in jobs { $crate::echo!("[{}] {}", id, cmd); }
    }};
}

// Event and trap macros still use the OS event registry for now
#[macro_export]
macro_rules! event {
    (register $event:expr, $handler:expr) => {{
        let mut handlers = $crate::os::EVENT_HANDLERS.lock().unwrap();
        let event_handlers = handlers.entry($event.to_string()).or_insert_with(Vec::new);
        event_handlers.push(Box::new($handler));
    }};
    (emit $event:expr, $($key:expr => $value:expr),*) => {{
        let mut data = ::std::collections::HashMap::new();
        $( data.insert($key.to_string(), $value.to_string()); )*
        let event_data = $crate::os::EventData { event_type: $event.to_string(), data, };
        if let Some(handlers) = $crate::os::EVENT_HANDLERS.lock().unwrap().get($event) {
            for handler in handlers { handler(&event_data); }
        }
    }};
}

#[macro_export]
macro_rules! trap {
    ($handler:expr, on: $signal:expr) => {{
        let sig_name = $signal.to_uppercase();
        match sig_name.as_str() {
            "SIGINT" | "SIGTERM" | "EXIT" | "COMMAND_ERROR" => {
                $crate::os::install_signal_handlers();
                $crate::event!(register &sig_name, $handler);
            }
            _ => { $crate::event!(register &sig_name, $handler); }
        }
    }};
}

// --- Thread Utility Macros ---
// Moved from time_math.rs - these belong in threads package

#[macro_export]
macro_rules! benchmark {
    ($body:block) => {
        {
            let start = std::time::Instant::now();
            $body
            let duration = start.elapsed();
            println!("Benchmark completed in: {:?}", duration);
            duration
        }
    };
}

#[macro_export]
macro_rules! sleep {
    ($secs:expr) => {{
        std::thread::sleep(std::time::Duration::from_secs($secs as u64));
    }};
    (ms: $millis:expr) => {{
        std::thread::sleep(std::time::Duration::from_millis($millis as u64));
    }};
}

