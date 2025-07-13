use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::fs;
use std::fs::File;
use serde_json;
use serde::Serialize;

#[derive(Serialize)]
pub struct DuplicateFiles {
    pub kept_file: String,
    pub hash_value: String,
    pub bytes_saved: u64,
    pub quarantined_file: Vec<String>,
}

pub fn quarantine_folder(dir: &str) -> PathBuf {
    let quarantine = PathBuf::from(dir);
    fs::create_dir_all(&quarantine).expect("Failed to create quarantine folder");
    quarantine
}

pub fn quarantine_and_report(hash_map: &HashMap<String, Vec<PathBuf>>, quarantine: &Path) -> (Vec<DuplicateFiles>, u64) 
{
    let mut report = Vec::new();
    let mut total_saved = 0u64 ;

    for (hash, paths) in hash_map {
        if paths.len() > 1 {
            let original = &paths[0];
            let mut quarantined = Vec::new();

            for dup in &paths[1..] {
                let target = quarantine.join(dup.file_name().unwrap());
                if fs::rename(dup, &target).is_ok() {
                    let size = fs::metadata(&target).unwrap().len();
                    total_saved += size;
                    quarantined.push(dup.display().to_string());
                }
                else {
                    eprint!("Failed to quarantine {}.", dup.display());
                }
            }

            report.push(DuplicateFiles{
                kept_file: original.display().to_string(),
                hash_value: hash.clone(),
                bytes_saved: total_saved,
                quarantined_file: quarantined
            });
        }
    }

    (report, total_saved)
}

pub fn write_report(report: &[DuplicateFiles]) {
    let json = serde_json::to_string_pretty(report).unwrap();
    let mut file = File::create("report.json").expect("Cannot create report file");
    file.write_all(json.as_bytes()).unwrap();
}