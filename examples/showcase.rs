//! examples/showcase.rs

// Using the prelude brings all the essential RSB tools into scope.
use rsb::prelude::*;
// Visual/log macros: import explicitly; gate colored under feature
#[cfg(feature = "visual")]
use rsb::{colored, debug, error, fatal, info, okay, trace, warn};
#[cfg(not(feature = "visual"))]
use rsb::{debug, error, fatal, info, okay, trace, warn};

fn main() {
    // The bootstrap! macro handles collecting args, loading the environment,
    // and setting up the context all in one go.
    let args = bootstrap!();

    // The dual-dispatch pattern allows for "bootstrap" commands that run
    // before any configuration is loaded. These are for setup, installation, etc.
    if pre_dispatch!(&args, {
        "install" => install_deps,
        "init" => init_project,
        "check" => check_system
    }) {
        return;
    }

    // After pre-dispatch, load configuration files using the `src!` alias.
    info!("Loading configuration...");
    src!("./myapp.conf");

    // The main dispatcher routes commands to their handler functions.
    dispatch!(&args, {
        "build" => build_project,
        "deploy" => deploy,
        "logs" => logs,
        "config" => config,
        "process" => process_data,
        "test" => run_tests,
        "meta-test" => meta_test,
        "date-test" => date_test,
        "path-test" => path_test,
        "file-in-test" => file_in_test,
        "array-test" => array_test,
        "system-test" => system_test,
        "math-test" => math_test,
        "cap-stream-test" => cap_stream_test,
        "trap-test" => trap_test,
        "random-test" => random_test,
        "dict-test" => dict_test,
        "sed-test" => sed_test,
        "archive-test" => archive_test,
        "utils-test" => utils_test
    });
}

// --- Pre-Context (Bootstrap) Commands ---

fn install_deps(mut args: Args) -> i32 {
    info!("Installing dependencies...");
    let force = args.has_pop("--force");
    if force {
        warn!("Force installation enabled.");
    }
    cmd!("echo 'Simulating package installation...'").each(|line| okay!("{}", line));
    0
}

fn init_project(args: Args) -> i32 {
    let project_path_str = args.get_or(1, "new-rsb-project");
    let project_path = std::path::Path::new(&project_path_str);

    if let Some(name) = project_path.file_name().and_then(|n| n.to_str()) {
        validate!(is_name(name), "Invalid project name: {}", name);
    } else {
        error!("Invalid project path provided.");
        return 1;
    }

    validate!(
        !is_entity(&project_path_str),
        "Project directory already exists: {}",
        project_path_str
    );

    info!("Initializing project: {}", project_path_str);
    mkdir_p(&project_path_str);
    let readme_content = format!(
        "# {}\n\nInitialized with RSB.",
        project_path.file_name().unwrap().to_str().unwrap()
    );
    write_file(
        &project_path.join("README.md").to_str().unwrap(),
        &readme_content,
    );

    okay!("Project initialized successfully!");
    0
}

fn check_system(_args: Args) -> i32 {
    info!("Checking system requirements...");
    require_command!("git");
    require_command!("rustc");
    require_command!("cargo");
    okay!("All system requirements satisfied!");
    0
}

// --- Context-Aware Commands ---

fn build_project(mut args: Args) -> i32 {
    require_var!("HOME");
    set_var("PROJECT", "my-app");

    let version = args
        .has_val("--version")
        .unwrap_or_else(|| "1.0.0".to_string());
    let target = args.get_or(1, "debug");
    let clean = args.has_pop("--clean");
    if let Some(output_dir) = args.get_kv("output") {
        set_var("BUILD_DIR", &output_dir);
    } else {
        set_var("BUILD_DIR", "/tmp/builds");
    }
    if let Some(features) = args.get_array("features") {
        info!("Enabling features: {}", features.join(", "));
    }

    info!("Building $PROJECT v{} for target: {}", version, target);

    if clean {
        warn!("Cleaning workspace...");
        run_cmd("echo 'cargo clean'");
    }

    mkdir_p("$BUILD_DIR");
    let build_log_path = param!("BUILD_DIR", default: "/tmp") + "/build.log";

    pipe!("Compiling module 1...\nCompiling module 2...\n   Finished dev [unoptimized + debuginfo] target(s)")
        .tee(&build_log_path)
        .each(|line| okay!("{}", line));

    okay!("Build successful! Log at {}", build_log_path);
    0
}

fn run_tests(_args: Args) -> i32 {
    info!("Running tests...");
    let results = cmd!("echo 'Running 3 tests\ntest result: ok. 3 passed; 0 failed.'");

    if results.to_string().contains("failed") {
        error!("Tests failed!");
        return 1;
    }

    okay!("All tests passed!");
    0
}

fn deploy(mut args: Args) -> i32 {
    let env = args.get_or(1, "staging");
    let force = args.has_pop("--force");

    case!(env.as_str(), {
        "staging" => {
            info!("Deploying to staging environment.");
        },
        "production" => {
            warn!("Deploying to PRODUCTION environment.");
            if !force {
                error!("Production deploy requires the --force flag.");
                return 1;
            }
            okay!("Force flag provided. Proceeding with production deploy.");
        },
        _ => {
            error!("Unknown environment: {}. Please use 'staging' or 'production'.", env);
            return 1;
        }
    });

    okay!("Deployment to {} successful!", env);
    0
}

fn logs(mut args: Args) -> i32 {
    let log_file = args
        .has_val("--file")
        .unwrap_or_else(|| "app.log".to_string());
    let errors_only = args.has_pop("--errors");

    pipe!(
        "INFO: Application started\nDEBUG: Connecting to database\nERROR: Failed to connect\nINFO: Retrying..."
    ).to_file(&log_file);

    require_file!(&log_file);
    info!("Reading logs from {}", log_file);

    if errors_only {
        warn!("Showing errors only.");
        cat!(&log_file)
            .grep("ERROR")
            .each(|line| error!("{}", line));
    } else {
        cat!(&log_file).each(|line| echo!("{}", line));
    }
    0
}

fn config(args: Args) -> i32 {
    let action = args.get_or(1, "list");

    match action.as_str() {
        "set" => {
            let key = args.get_or(2, "");
            let value = args.get_or(3, "");
            validate!(!key.is_empty(), "Usage: config set <key> <value>");
            info!("Setting config: {} = {}", key, value);
            set_var(&key, &value);
            save_config_file("./myapp.conf", &[&key]);
            okay!("Configuration saved to ./myapp.conf");
        }
        "get" => {
            let key = args.get_or(2, "");
            validate!(!key.is_empty(), "Usage: config get <key>");
            echo!("{} = {}", key, get_var(&key));
        }
        _ => {
            error!("Unknown config action: {}", action);
            return 1;
        }
    }
    0
}

fn process_data(_args: Args) -> i32 {
    let input_file = "data.csv";
    let output_file = "processed.txt";

    write_file(
        input_file,
        "user,active,id\nalice,true,101\nbob,false,102\ncharlie,true,103\nalice,true,104",
    );

    info!("Processing {} -> {}", input_file, output_file);

    let processed_count = cat!(input_file)
        .grep("true")
        .cut(1, ",")
        .unique()
        .sort()
        .tee(output_file)
        .count();

    okay!("Processed {} unique active users.", processed_count);
    info!("Result saved to {}", output_file);

    echo!("\nProcessed Users:");
    cat!(output_file).each(|line| echo!("- {}", line));

    0
}

fn meta_test(args: Args) -> i32 {
    let file_path = args.get_or(1, "meta.txt");
    meta_keys!(&file_path, into: "META");

    echo!("Author: {}", get_var("META_author"));
    echo!("Version: {}", get_var("META_version"));
    0
}

fn array_test(_args: Args) -> i32 {
    info!("Testing array utilities...");
    set_array("MY_ARRAY", &["a", "b", "c"]);
    echo!("Array: $MY_ARRAY");
    echo!("Length: {}", array_length("MY_ARRAY"));
    echo!("Item 1: {}", array_get("MY_ARRAY", 1));
    array_push("MY_ARRAY", "d");
    echo!("Pushed 'd', new array: $MY_ARRAY");
    0
}

fn system_test(_args: Args) -> i32 {
    info!("Testing system utilities...");
    echo!("Line: {}", str_line!('-', 10));
    let num = rand_range!(1, 100);
    echo!("Random number: {}", num);
    validate!(num >= 1 && num <= 100, "Random number out of range");
    0
}

fn date_test(_args: Args) -> i32 {
    info!("Testing date macros...");
    echo!("Default: {}", date!());
    echo!("Epoch: {}", date!(epoch));
    echo!("Human: {}", date!(human));
    echo!("Custom: {}", date!("%Y-%m-%d"));
    let d = benchmark!({
        // sleep for a short duration
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
    info!("Benchmark macro returned duration: {:?}", d);
    0
}

fn path_test(args: Args) -> i32 {
    let path = args.get_or(1, "./README.md");
    info!("Testing path macros for: {}", path);

    let canon_path = path_canon!(&path);
    echo!("Canonical Path: {}", canon_path);

    path_split!(&path, into: "MYPATH");
    echo!("Parent: {}", get_var("MYPATH_parent"));
    echo!("Filename: {}", get_var("MYPATH_file_name"));
    0
}

fn file_in_test(args: Args) -> i32 {
    let dir = args.get_or(1, ".");
    info!("Testing file_in! macro for dir: {}", dir);

    file_in!(file in &dir => {
        echo!("Found file: $file");
    });
    0
}

fn math_test(_args: Args) -> i32 {
    set_var("A", "10");
    set_var("B", "3.5");
    math!("C = (A + 5) * B / 2"); // (10 + 5) * 3.5 / 2 = 15 * 3.5 / 2 = 52.5 / 2 = 26.25
    echo!("C = {}", get_var("C"));
    math!("C += 1.75");
    echo!("C += 1.75 -> {}", get_var("C")); // 28.0
    0
}

fn cap_stream_test(_args: Args) -> i32 {
    let mut stream = pipe!("hello\nworld");
    let temp_path = cap_stream!(stream);
    echo!("Captured to: {}", temp_path);
    if is_file(&temp_path) {
        echo!("Temp file exists.");
    }
    // The EXIT trap in bootstrap! should clean this up.
    0
}

fn trap_test(_args: Args) -> i32 {
    set_var("ERROR_COUNT", "0");
    trap!(|data: &EventData| {
        let source = data.data.get("source").unwrap();
        let status = data.data.get("status").unwrap();
        info!("ERROR TRAP: Command '{}' failed with status {}", source, status);
        math!("ERROR_COUNT += 1");
    }, on: "COMMAND_ERROR");

    // This command will fail and trigger the trap
    run!("ls /nonexistent-directory");

    echo!("Final error count: $ERROR_COUNT");
    0
}

fn random_test(_args: Args) -> i32 {
    echo!("rand_alnum: {}", rand_alnum!(10));
    echo!("rand_alpha: {}", rand_alpha!(10));
    echo!("rand_hex: {}", rand_hex!(10));
    echo!("rand_string: {}", rand_string!(10));
    echo!("rand_uuid: {}", rand_uuid!());
    0
}

fn dict_test(_args: Args) -> i32 {
    write_file("test.dict", "apple banana orange");
    let my_dict = dict!("test.dict");
    set_array(
        "MY_DICT",
        &my_dict.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
    );
    echo!("Random word: {}", rand_dict!("MY_DICT"));

    gen_dict!(alnum, 5, into: "RANDOM_WORDS");
    echo!("Generated words: $RANDOM_WORDS");
    0
}

fn sed_test(_args: Args) -> i32 {
    // Test data
    let sample_text = "Line 1\nLine 2\nIMPORTANT: Main content\nLine 4\nLine 5\nLine 6\n{{PLACEHOLDER}}\nLine 8\nLine 9\nLine 10";

    info!("Testing sed_lines (lines 2-4):");
    let lines_result = sed_lines!(sample_text, 2, 4);
    echo!("{}", lines_result);

    info!("Testing sed_around (2 lines around 'IMPORTANT'):");
    let around_result = sed_around!(sample_text, "IMPORTANT", 2);
    echo!("{}", around_result);

    info!("Testing sed_insert (replace {{PLACEHOLDER}}):");
    let insert_content = "INSERTED LINE 1\nINSERTED LINE 2\nINSERTED LINE 3";
    let insert_result = sed_insert!(insert_content, "{{PLACEHOLDER}}", sample_text);
    echo!("{}", insert_result);

    info!("Testing sed_template (replace all instances):");
    let template_text =
        "Hello {{NAME}}, welcome to {{PLACE}}! {{NAME}}, enjoy your stay at {{PLACE}}.";
    let template_result1 = sed_template!("Alice", "{{NAME}}", &template_text);
    let template_result2 = sed_template!("Wonderland", "{{PLACE}}", &template_result1);
    echo!("{}", template_result2);

    info!("Testing sed_replace (simple replacement):");
    let replace_result = sed_replace!("Hello world, world!", "world", "RSB");
    echo!("{}", replace_result);

    0
}

fn sed_block_test(_args: Args) -> i32 {
    let content = "
    # Other file content
    <config>
        <setting>old_value</setting>
    </config>
    # More content
    ";

    // Test replacing content within the block
    let result1 = pipe!(content)
        .sed_block("<config>", "</config>", "s/old_value/new_value/g")
        .to_string();
    echo!("--- Test 1: Replace 'old_value' ---\n{}", result1);

    // Test with no end pattern
    let result2 = pipe!(content)
        .sed_block("<config>", "NO_SUCH_END", "s/old_value/new_value/g")
        .to_string();
    if result2.contains("old_value") {
        echo!("Unclosed block contains: old_value");
    }
    echo!("--- Test 2: No end pattern ---\n{}", result2);

    0
}

fn color_test(_args: Args) -> i32 {
    info!("This is an info message.");
    okay!("This is an okay message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
    fatal!("This is a fatal message.");
    debug!("This is a debug message.");
    trace!("This is a trace message.");
    0
}

fn archive_test(_args: Args) -> i32 {
    info!("Testing archive operations...");

    // Create test files
    write_file("test1.txt", "Hello from file 1");
    write_file("test2.txt", "Hello from file 2");
    mkdir_p("testdir");
    write_file("testdir/test3.txt", "Hello from file 3");

    info!("Testing pack! macro (auto-detect format):");
    pack!("test.tar.gz", "test1.txt", "test2.txt", "testdir");

    info!("Testing tar! macro operations:");
    tar!(create: "test.tar", "test1.txt", "test2.txt");

    info!("Listing tar contents:");
    let tar_contents = tar!(list: "test.tar");
    echo!("{}", tar_contents);

    if is_command("zip") {
        info!("Testing zip! macro operations:");
        zip!(create: "test.zip", "test1.txt", "test2.txt");

        info!("Listing zip contents:");
        let zip_contents = zip!(list: "test.zip");
        echo!("{}", zip_contents);
    } else {
        warn!("zip command not available, skipping zip tests");
    }

    info!("Testing unpack! macro:");
    mkdir_p("extract_test");
    unpack!("test.tar", to: "extract_test");

    info!("Files extracted to extract_test:");
    file_in!(file in "extract_test" => {
        echo!("Found: $file");
    });

    // Cleanup
    rm_rf("test1.txt");
    rm_rf("test2.txt");
    rm_rf("testdir");
    rm_rf("test.tar");
    rm_rf("test.tar.gz");
    rm_rf("test.zip");
    rm_rf("extract_test");

    0
}

fn utils_test(_args: Args) -> i32 {
    info!("Testing system utilities...");

    // System information
    echo!("Hostname: {}", hostname!());
    echo!("User: {}", user!());
    echo!("Home: {}", home_dir!());
    echo!("Current dir: {}", current_dir!());

    // Process management (demo only, using sleep)
    info!("Testing process management...");
    let job_id = job!(background: "sleep 1");
    echo!("Started sleep job: {}", job_id);

    let sleep_pid = pid_of!("sleep");
    if !sleep_pid.is_empty() {
        echo!("Found sleep process with PID: {}", sleep_pid);
        echo!("Process exists: {}", process_exists!("sleep"));
    }

    job!(wait: job_id);

    // Locking demonstration
    info!("Testing locking mechanism...");
    with_lock!("/tmp/rsb-test.lock" => {
        echo!("Inside locked section!");
        sleep!(ms: 100);
    });

    // Network (if curl is available)
    if is_command("curl") {
        info!("Testing HTTP utilities...");
        // Test with a simple HTTP request (httpbin echo)
        let response = get!("https://httpbin.org/get?test=rsb", options: "-s --connect-timeout 5");
        if !response.is_empty() {
            echo!("HTTP GET response received ({} bytes)", response.len());

            // JSON parsing if jq is available
            if is_command("jq") {
                let test_param = json_get!(&response, ".args.test");
                if !test_param.is_empty() {
                    echo!("Parsed JSON field 'args.test': {}", test_param);
                }
            } else {
                warn!("jq not available, skipping JSON parsing test");
            }
        } else {
            warn!("HTTP request failed or returned empty response");
        }
    } else {
        warn!("curl not available, skipping HTTP tests");
    }

    0
}
