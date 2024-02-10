use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use tokio::fs::read_dir;

use self::image::ImageManipulation;

pub mod convertion;
pub mod image;

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
