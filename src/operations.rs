use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use tokio::fs::read_dir;

pub mod convertion;
pub mod generics;
//pub mod image;
pub mod path;

pub async fn index_files<S: AsRef<str>>(input: S) -> Result<HashMap<PathBuf, Box<str>>, Error> {
    let mut map = HashMap::new();
    let mut entries = read_dir(input.as_ref()).await?;

    for entry in entries
        .next_entry()
        .await
        .iter()
        .filter_map(|entry| match entry {
            Some(value) => Some(value.path()),
            None => None,
        })
        .map(|path| (path.clone(), String::from(path.to_str().unwrap())))
    {
        let (key, value) = entry;

        map.insert(key, Box::from(value));
    }
    drop(entries);
    return Ok(map);
}
