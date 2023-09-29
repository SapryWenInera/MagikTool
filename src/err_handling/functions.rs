use crate::operations::image::functions::is_image;
use std::{
    collections::HashSet,
    fs::{create_dir, create_dir_all, DirEntry, ReadDir},
    path::Path,
    sync::Arc,
};

pub trait Endswith {
    fn ends_with_plus(&self, pattern: &str, img: &Arc<str>) -> Arc<str>;
}

impl Endswith for str {
    fn ends_with_plus(&self, pattern: &str, img: &Arc<str>) -> Arc<str> {
        match self.ends_with(pattern) {
            true => Arc::from(format!("{}{}", self, img)),
            false => Arc::from(format!("{}/{}", self, img)),
        }
    }
}
pub fn read_dir_collect(input: ReadDir) -> HashSet<Arc<str>> {
    let out: HashSet<Arc<str>> = input
        .filter_map(|objs| {
            objs.ok().and_then(|name: DirEntry| {
                name.path().extension().and_then(|extension| {
                    if is_image(extension) {
                        name.file_name().to_str().map(|image| Arc::from(image))
                    } else {
                        None
                    }
                })
            })
        })
        .collect();
    return out;
}

pub fn makedir(input: &str) {
    let path = Path::new(input);

    match path.exists() {
        true => {}
        false => match create_dir(input) {
            Ok(_) => (),
            Err(_) => match create_dir_all(input) {
                Ok(_) => (),
                Err(e) => panic!("Error: {}", e),
            },
        },
    }
}
