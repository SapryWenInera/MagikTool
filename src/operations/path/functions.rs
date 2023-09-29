use crate::err_handling::functions::read_dir_collect;
use std::{collections::HashSet, fs::read_dir, sync::Arc};

pub fn extract_files(input: &str) -> HashSet<Arc<str>> {
    let images = match read_dir(input) {
        Ok(entries) => read_dir_collect(entries),
        Err(e) => panic!("Error: {}", e),
    };
    return images;
}
