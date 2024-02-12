use std::collections::BTreeMap;
use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;
use std::sync::Arc;

use crate::image::ImageManipulation;

pub trait PathBufExtras {
    async fn merge_images<S: AsRef<str>>(&self, map: BTreeMap<PathBuf, Arc<str>>, format: S) -> BTreeMap<PathBuf, PathBuf>;
}

pub async fn index_images<P: Into<PathBuf>>(input: P) -> Result<BTreeMap<PathBuf, Arc<str>>, Error> {
    let input: PathBuf = input.into();
    let map = read_dir(input)?.map(|r| r.unwrap())
    .filter_map(|f| f.path().is_image()).map(|p| (p.clone(), Arc::from(p.to_string_lossy().to_string()))).collect();
    dbg!(&map);
    Ok(map)
}

impl PathBufExtras for PathBuf {
    async fn merge_images<S: AsRef<str>>(&self, map: BTreeMap<PathBuf, Arc<str>>, format: S) -> BTreeMap<PathBuf, PathBuf> {
        let mut out_map = BTreeMap::new();

        for (path, _boxed_path) in map {
            let new_path = path.with_extension(format.as_ref());
            let file_name = new_path.file_name().unwrap().to_string_lossy();
            let out_path = self.join(String::from(file_name));

            match self.is_image() {
                Some(p) => out_map.insert(path, p),
                None => out_map.insert(path, out_path)
            };

        }

        out_map
    }
}