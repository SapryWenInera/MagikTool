use crate::operations::path::functions::extract_files;
use core::panic;
use std::path::Path;
use std::sync::Arc;
use std::{collections::HashSet, fs::metadata};

pub fn chooser(input: &str) -> HashSet<Arc<str>> {
    match metadata(input) {
        Ok(path) => match path.is_dir() {
            true => extract_files(input),
            false => HashSet::from([Arc::from(
                Path::new(input).file_name().unwrap().to_str().unwrap(),
            )]),
        },
        Err(_) => panic!("-i/--input has invalid value."),
    }
}
