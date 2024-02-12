mod image;
mod operations;
mod parser;

use std::collections::BTreeMap;
use std::{fs::create_dir_all, path::PathBuf, process::Command, sync::Arc};
use operations::{index_images, PathBufExtras};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use tokio::runtime::Runtime;

use crate::image::ImageManipulation;
use crate::parser::Parser;

fn main() {
    let runtime = Arc::from(Runtime::new().unwrap());
    let mut args = Parser::new();
    args.args_parse();
    dbg!(&args);
    match args.output.is_image() {
        Some(p) => match p.exists() {
            true => (),
            false => create_dir_all(p).unwrap()
        },
        None => ()
    }

    let input_map = if args.input.is_dir() {
        runtime.block_on(index_images(args.input)).unwrap()

    } else {
        let value: Arc<str> = Arc::from(args.input.to_string_lossy());
        let mut image = BTreeMap::new();

        image.insert(args.input, value);

        image
    };
    dbg!(&input_map);
    let output_map = runtime.block_on(args.output.merge_images(input_map, args.format));
    dbg!(&output_map); 
    let args: Vec<&str> = args.options.split_whitespace().collect();

    let _ = convert_images(output_map, args);
}

fn convert_images(input: BTreeMap<PathBuf, PathBuf>, args: Vec<&str>) {
    input.par_iter().for_each(|(input_path, output_path)| {
        Command::new("convert").arg(input_path).args(args.clone()).arg(output_path).spawn().unwrap().wait().unwrap();
    })
}