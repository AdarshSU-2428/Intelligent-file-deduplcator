mod parallel_hashing;
mod advance_filtering;
mod quarantine_and_report;
mod hash_algo;

use std::path::Path;
use std::time::Duration;
use regex::Regex;
use colored::*;

use crate::parallel_hashing::parallel_hashing;
use crate::advance_filtering::advance_filtering;
use crate::quarantine_and_report::{quarantine_folder, quarantine_and_report, write_report};
use crate::parallel_hashing::hash_algo::HashAlgorithm;

fn main() {
    let target_dir = Path::new("./test_folder");
    let quarantine_dir = quarantine_folder("quarantine_folder");

    let min_size = 1024; 
    let max_size = 5 * 1024 * 1024; 
    let allowed_exts = vec!["txt", "log", "pdf"];
    let name_regex = Regex::new(r"^(data_|test_).*").unwrap();
    let modified_after = Duration::from_secs(3 * 365 * 24 * 60 * 60);

    let files = advance_filtering(&target_dir, modified_after, name_regex, min_size, max_size, &allowed_exts);
    println!("{}", format!("Filtered files: {}", files.len()).yellow().bold());

    let algo = HashAlgorithm::SHA256;

    let hash_map = parallel_hashing(files, &algo);

    let (report, total_saved) = quarantine_and_report(&hash_map, &quarantine_dir);
    write_report(&report);

    println!("{}", "Deduplication complete".cyan().bold());
    println!("{}", format!("Total space saved: {} bytes", total_saved).blue().bold());
    println!("{}", "Reports written to report.json".magenta().bold());
}


    



