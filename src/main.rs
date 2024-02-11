mod image;
mod operations;
mod parser;

use std::collections::BTreeMap;
use std::{fs::create_dir_all, path::PathBuf, process::Command, sync::Arc};

use operations::{index_images, PathBufExtras};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use tokio::runtime::Runtime;

use crate::parser::Parser;

fn main() {
    let runtime = Arc::from(Runtime::new().unwrap());
    let mut args = Parser::new();

    args.args_parse();

    if !args.output.exists() && args.input.is_dir() {
        create_dir_all(&args.output).unwrap();
    };

    let input_map = if args.input.is_dir() {
        runtime.block_on(index_images(&args.input)).unwrap()

    } else {
        let value: Arc<str> = Arc::from(args.input.to_str().unwrap());
        let mut image = BTreeMap::new();

        image.insert(args.input, value);

        image
    };
    let output_map = runtime.block_on(args.output.merge_images(input_map.clone(), &args.format));

    let args: Vec<&str> = args.options.split_whitespace().collect();

    let _ = convert_images(input_map, output_map, args);
}

fn convert_images(input: BTreeMap<PathBuf, Arc<str>>, output: BTreeMap<PathBuf, Arc<str>>, args: Vec<&str>) {
    let out = output.into_iter().next();
    input.into_par_iter().for_each(|(path, _boxed_path)| {
        let (out_path, _boxed_out_path) = out.clone().unwrap();

        match Command::new("convert").arg(path).args(args.clone()).arg(out_path).spawn() {
            Ok(mut r) => match r.wait() {
                Ok(_) => (),
                Err(err) => panic!("{}", err)
            },
            Err(err) => panic!("{}", err)
        }
    });
}