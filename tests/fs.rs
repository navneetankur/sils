#[test]
fn test_walk_dir() {
    let mut found_me = false;
    let this_file = "./tests/fs.rs";
    for path in sils::fs::walk_dir(".") {
        if path == std::path::Path::new(this_file) {
            found_me = true;
        }
    }
    assert!(found_me, "failed to find {this_file} by walk_dir");
}
