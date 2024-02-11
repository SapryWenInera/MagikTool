use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use tokio::fs::read_dir;
use crate::image::ImageManipulation;

pub trait PathBufExtras {
    async fn merge_images<S: AsRef<str>>(&self, map: HashMap<PathBuf, Box<str>>, format: S) -> HashMap<PathBuf, Box<str>>;
}

pub async fn index_images<P: Into<PathBuf>>(input: P) -> Result<HashMap<PathBuf, Box<str>>, Error> {
    let mut map = HashMap::new();
    let mut entries = read_dir(input.into()).await?;

    for entry in entries
        .next_entry()
        .await
        .iter()
        .filter_map(|entry| match entry {
            Some(value) => Some(value.path()),
            None => None,
        })
        .filter_map(|f| f.is_image())
        .map(|path| (path.clone(), String::from(path.to_string_lossy())))
    {
        let (key, value) = entry;

        map.insert(key, Box::from(value));
    }
    drop(entries);
    Ok(map)
}

impl PathBufExtras for PathBuf {
    async fn merge_images<S: AsRef<str>>(&self, map: HashMap<PathBuf, Box<str>>, format: S) -> HashMap<PathBuf, Box<str>> {
        let mut out_map = HashMap::new();

        for (path, _boxed_path) in map {
            let new_path = path.with_extension(format.as_ref());
            let file_name = new_path.file_name().unwrap().to_string_lossy();
            let out_path = self.join(String::from(file_name));
            let value:Box<str> = Box::from(out_path.clone().to_string_lossy());

            out_map.insert(out_path, value);

        }

        out_map
    }
}