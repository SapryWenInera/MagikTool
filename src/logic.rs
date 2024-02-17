use crate::image::ImageManipulation;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::BTreeMap, fs::read_dir, io::Error, path::Path};

pub trait BtreeIterator {
    async fn merge_images<S: AsRef<str> + std::marker::Sync>(
        self,
        o_path: Box<Path>,
        format: S,
    ) -> BTreeMap<Box<Path>, Box<Path>>;
}

pub async fn index_images<P: Into<Box<Path>>>(input: P) -> Result<BTreeMap<Box<Path>, ()>, Error> {
    let input: Box<Path> = input.into();
    let map = read_dir(input)?
        .map(|r| r.unwrap())
        .filter_map(|f| f.path().is_image())
        .map(|p| (p, ()))
        .collect();
    dbg!(&map);
    Ok(map)
}

impl BtreeIterator for BTreeMap<Box<Path>, ()> {
    async fn merge_images<S: AsRef<str> + std::marker::Sync>(
        self,
        o_path: Box<Path>,
        format: S,
    ) -> BTreeMap<Box<Path>, Box<Path>> {
        self.par_iter()
            .filter_map(|(key, _val)| path_indexer(key.to_owned(), o_path.clone(), format.as_ref()))
            .collect()
    }
}

fn path_indexer<S: AsRef<str>>(
    i_p: impl Into<Box<Path>>,
    o_p: impl Into<Box<Path>>,
    f: S,
) -> Option<(Box<Path>, Box<Path>)> {
    let path: Box<Path> = i_p.into();
    let o_path: Box<Path> = o_p.into();
    let file_name = path.file_name()?;

    match o_path.is_image() {
        Some(image) => {
            let out_path = image.with_extension(f.as_ref());
            Some((path, out_path.into_boxed_path()))
        }
        None => {
            let out_path = o_path.join(file_name).with_extension(f.as_ref());

            Some((path, out_path.into_boxed_path()))
        }
    }
}
