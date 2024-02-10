mod operations;
mod parser;

use std::{collections::HashMap, fs::create_dir_all};

use operations::index_images;
use tokio::runtime::Runtime;

use crate::parser::Parser;

fn main() {
    let runtime = Runtime::new().unwrap();
    let mut args = Parser::new();

    args.args_parse();

    if !args.output.exists() && args.input.is_dir() {
        create_dir_all(args.output).unwrap();
    };

    let input_map = if args.input.is_dir() {
        runtime.block_on(index_images(&args.input)).unwrap()

    } else {
        let value: Box<str> = Box::from(args.input.to_str().unwrap());
        let mut image = HashMap::new();

        image.insert(args.input, value);

        image
    };
}
