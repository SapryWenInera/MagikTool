mod image;
mod operations;
mod parser;

use std::collections::BTreeMap;
use std::{fs::create_dir_all, path::Path, process::Command, sync::Arc};
use operations::index_images;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use tokio::runtime::Runtime;

use crate::image::ImageManipulation;
use crate::operations::BtreeIterator;
use crate::parser::Parser;

fn main() {
    let runtime = Arc::from(Runtime::new().unwrap());
    let mut args = Parser::new();
    args.args_parse();
    dbg!(&args);
    match args.output.exists() {
        true => (),
        false => match args.output.is_image() {
            Some(_) => {
                let parent = args.output.parent().unwrap();
                create_dir_all(parent).unwrap()
            },
            None => create_dir_all(&args.output).unwrap()
        }
    };

    let input_map = if args.input.is_dir() {
        runtime.block_on(index_images(args.input)).unwrap()

    } else {
        let mut image = BTreeMap::new();

        image.insert(args.input, ());
        
        image
    };
    dbg!(&input_map);
    let output_map = runtime.block_on(input_map.merge_images(args.output, args.format));
    dbg!(&output_map); 
    let options: Vec<&str> = args.options.split_whitespace().collect();
    runtime.block_on(convert_images(output_map, options));
}

async fn convert_images(input: BTreeMap<Box<Path>, Box<Path>>, args: Vec<&str>) {
    input.par_iter().for_each(|(input_path, output_path)| {
        if input_path.extension() == output_path.extension() {
            Command::new("cp").arg("-f").arg(input_path.as_os_str()).arg(output_path.as_os_str()).spawn().unwrap().wait().unwrap();
        } else {
            Command::new("convert").arg(input_path.as_os_str()).args(args.clone()).arg(output_path.as_os_str()).spawn().unwrap().wait().unwrap();
        }
    })
}