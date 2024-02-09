use tokio::fs::read_dir;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Error;

pub mod convertion;
pub mod generics;
//pub mod image;
pub mod path;

pub async fn index_files<S: AsRef<str>>(
    input: S,
) -> Result<HashMap<PathBuf, Box<str>>, Error> {
    let mut map = HashMap::new();
    let mut entries = read_dir(input.as_ref()).await?;

    for entry in entries.next_entry().await {
        let entry = entry.unwrap();
        let key = entry.path();
        let value: Box<str> = Box::from(key.clone().to_str().unwrap());
        let _ = map.insert(key, value);
    }

    drop(entries);
    return Ok(map);
}
