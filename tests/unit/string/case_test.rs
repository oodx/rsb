use rsb::prelude::*;

#[test]
fn test_string_case_helpers() {
    assert_eq!(
        rsb::string::to_snake_case("HTTP server ID"),
        "http_server_id"
    );
    assert_eq!(rsb::string::to_kebab_case("User Name 42"), "user-name-42");
    assert_eq!(rsb::string::to_dot_case("Log File Name"), "log.file.name");
    assert_eq!(rsb::string::to_space_case("My File NAME"), "my file name");
    assert_eq!(rsb::string::to_camel_case("HTTP server id"), "httpServerId");
}

#[test]
fn test_ascii_only_normalization() {
    // Non-ASCII letters are stripped and treated as separators
    assert_eq!(rsb::string::to_kebab_case("Crème brûlée"), "cr-me-br-l-e");
    assert_eq!(rsb::string::to_snake_case("ñandú X"), "and_x");
    // Explanation: "Ångström 2X" -> ascii normalize: " ngstr m 2X" -> tokens ["ngstr", "m", "2", "X"] -> camel: ngstrM2X
    assert_eq!(rsb::string::to_camel_case("Ångström 2X"), "ngstrM2X");
}

#[test]
fn test_string_case_macros_value_and_var() {
    // Value forms
    assert_eq!(snake!("HTTP server ID"), "http_server_id");
    assert_eq!(kebab!("User Name 42"), "user-name-42");
    assert_eq!(slug!("User Name 42"), "user-name-42");
    assert_eq!(dot!("Log File Name"), "log.file.name");
    assert_eq!(space!("My File NAME"), "my file name");
    assert_eq!(camel!("HTTP server id"), "httpServerId");

    // Var forms
    set_var("CASE_INPUT", "HelloWorldAPI42");
    assert_eq!(snake_var!("CASE_INPUT"), "hello_world_api_42");
    assert_eq!(kebab_var!("CASE_INPUT"), "hello-world-api-42");
    assert_eq!(dot_var!("CASE_INPUT"), "hello.world.api.42");
    assert_eq!(space_var!("CASE_INPUT"), "hello world api 42");
    assert_eq!(camel_var!("CASE_INPUT"), "helloWorldApi42");
}

#[test]
fn test_param_case_generalized() {
    set_var("NAME", "HTTP server id");
    assert_eq!(param!("NAME", case: snake), "http_server_id");
    assert_eq!(param!("NAME", case: kebab), "http-server-id");
    assert_eq!(param!("NAME", case: slug), "http-server-id");
    assert_eq!(param!("NAME", case: dot), "http.server.id");
    assert_eq!(param!("NAME", case: space), "http server id");
    assert_eq!(param!("NAME", case: camel), "httpServerId");
    assert_eq!(param!("NAME", case: lower), "http server id");
    assert_eq!(param!("NAME", case: upper), "HTTP SERVER ID");
}

#[test]
fn test_stream_case_per_line() {
    use rsb::prelude::*;
    let out = Stream::from_string("HelloWorld\nHTTP server id\nname42")
        .snake()
        .to_string();
    assert_eq!(out, "hello_world\nhttp_server_id\nname_42");
}
