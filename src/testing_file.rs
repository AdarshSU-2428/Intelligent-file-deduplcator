//Integration test of advance_filtering
#[cfg(test)]
use std::path::Path;
use std::time::Duration;
use regex::Regex;

mod advance_filtering {
    include!("C:\\Users\\ADARSH S SAHOO\\OneDrive\\Desktop\\Rust\\minor_project_1\\src\\advance_filtering.rs");
}

#[test]
fn test_advance_filtering() {
    let path = Path::new("C:\\Users\\ADARSH S SAHOO\\OneDrive\\Desktop\\Rust\\minor_project_1\\src\\advance_filter.rs");
    let duration = Duration::from_secs(3 * 365 * 24 * 60 * 60);
    let name_regex = Regex::new(".*\\.txt").unwrap();
    let min_size = 1024;
    let max_size = 5 * 1024 * 1024;
    let allowed_exts = vec!["txt", "log", "pdf"];
    let result = advance_filtering::advance_filtering(path, duration, name_regex, min_size, max_size, &allowed_exts);
    assert!(result.is_empty());
}