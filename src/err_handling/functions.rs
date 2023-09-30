use crate::operations::image::functions::is_image;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    collections::HashSet,
    fs::{create_dir, create_dir_all, DirEntry},
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
pub fn read_dir_collect(input: Vec<DirEntry>) -> HashSet<Arc<str>> {
    let c = usize_to_64(&input);

    let style = ProgressStyle::default_bar()
        .template("[1/2] [{elapsed_precise}] Processing Files: {msg}")
        .unwrap();

    let pb = ProgressBar::new(c);
    pb.set_style(style);

    let out: HashSet<Arc<str>> = input
        .iter()
        .filter_map(|name| {
            name.path().extension().and_then(|extension| {
                let msg = progress_msg(&name);

                pb.set_message(msg);

                if is_image(extension) {
                    name.file_name().to_str().map(|image| Arc::from(image))
                } else {
                    None
                }
            })
        })
        .collect();
    pb.finish_with_message("Done");
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

fn usize_to_64(input: &Vec<DirEntry>) -> u64 {
    let input = input;
    let o: usize = input.iter().count();

    return o as u64;
}

fn progress_msg(input: &DirEntry) -> String {
    let o = input.to_owned();

    return o.file_name().into_string().unwrap();
}
