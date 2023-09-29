mod err_handling;
mod operations;
mod parser;

use crate::{
    err_handling::functions::makedir,
    operations::{convertion::functions::convert_image, generics::functions::chooser},
    parser::Parser,
};

fn main() {
    let mut args = Parser::new();

    args.args_parse();

    let _ = makedir(&args.output);

    let images = chooser(&args.input);

    let _result = convert_image(
        &args.input,
        &args.output,
        images,
        args.options,
        &args.format,
    );

    let duration = start.elapsed().as_secs_f64();
}
