pub mod hash_algo;

use rayon::prelude::*;
use std::{collections::HashMap};
use std::path::PathBuf;
use crate::parallel_hashing::hash_algo::{hash_calculate, HashAlgorithm};

pub fn parallel_hashing(file_paths: Vec<PathBuf>, algo: &HashAlgorithm) -> HashMap<String, Vec<PathBuf>> {
    let hash_pairs: Vec<(String, PathBuf)> = file_paths
        .par_iter()
        .filter_map(|path| {
            hash_calculate(path, algo).map(|hash| (hash, path.clone()))
        })
        .collect();

    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for (hash, path) in hash_pairs {
        hash_map.entry(hash).or_default().push(path);
    }

    hash_map
}