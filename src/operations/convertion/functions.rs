use crate::{err_handling::functions::Endswith, operations::image::functions::ImgtoImg};
use rayon::prelude::*;
use std::{
    collections::HashSet,
    io::{stdout, Write},
    path::Path,
    process::Command,
    sync::Arc,
};

pub fn convert_image(
    input: &str,
    output: &str,
    images: HashSet<Arc<str>>,
    args: Vec<String>,
    img_format: &str,
) {
    match Path::new(input).is_dir() {
        true => images.par_iter().for_each(|image| {
            let input: Arc<str> = input.ends_with_plus("/", image);

            let output: Arc<str> = output.img_to_img(image, img_format);

            let _ = command(input, output, args.clone());
        }),
        false => println!("File"),
    }
}

fn command(input: Arc<str>, output: Arc<str>, args: Vec<String>) {
    let mut args = args;
    let path = &output.as_ref().output_clean();

    args.insert(0, input.to_string());
    args.push(input.to_string());

    if !Path::new(path).exists() {
        match Command::new("convert")
            .args(args)
            .spawn()
            .expect("convert not found in $PATH")
            .wait()
        {
            Ok(_) => {
                print!("\r{} to {} successful.", input, output);
                stdout().flush().unwrap();
            }
            Err(e) => println!("Error; {}", e),
        }
    } else {
        println!("{} exists.", path)
    }
}
