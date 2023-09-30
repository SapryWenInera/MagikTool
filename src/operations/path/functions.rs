use crate::err_handling::functions::read_dir_collect;
use std::fs::DirEntry;
use std::{collections::HashSet, fs::read_dir, sync::Arc};

pub fn extract_files(input: &str) -> HashSet<Arc<str>> {
    let images = match read_dir(input) {
        Ok(dir) => {
            let list: Vec<DirEntry> = dir.filter_map(|entry| entry.ok()).collect();

            read_dir_collect(list)
        }
        Err(e) => panic!("Error: {}", e),
    };
    return images;
}
