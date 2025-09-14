use rsb::prelude::*;

#[test]
fn uat_bash_demo() {
    println!("\n=== UAT: Bash Commands (curl/tar/zip) ===\n");

    // Network
    let get = rsb::bash::http_get("https://example.com");
    println!("curl GET status: {} (len={})", get.status, get.output.len());

    // Archive listing on fake files to demonstrate failure status
    let tar_list = rsb::bash::list_tar("/no/such/file.tar");
    println!("tar -tf status: {}", tar_list.status);
    let zip_list = rsb::bash::list_zip("/no/such/file.zip");
    println!("unzip -l status: {}", zip_list.status);
}

