use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use std::time::Duration;
use regex::Regex;

pub fn advance_filtering(path: &Path, duration: Duration, name_regex: Regex, min_size: u64, max_size: u64, allowed_exts: &[&str]) -> Vec<PathBuf> 
{
    let mut result = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let meta = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if meta.len() < min_size || meta.len() > max_size {
            continue;
        }

        let allowed_ext = match path.extension() {
            Some(e) => match e.to_str() {
                Some(ext) => allowed_exts.contains(&ext),
                None => false,
            },
            None => false,
        };

        if !allowed_ext {
            continue;
        }

        let fname = match path.file_name().unwrap().to_str() {
            Some(name) => name,
            None => continue,
        };

        if !name_regex.is_match(fname) {
            continue
        }

        let modified_time = match meta.modified().unwrap().elapsed() {
            Ok(t) => t,
            Err(_) => continue,
        };

        if modified_time > duration {
            continue;
        }

        result.push(path.to_path_buf());
    }
    result
}


