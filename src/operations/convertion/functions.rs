use crate::{err_handling::functions::Endswith, operations::image::functions::ImgtoImg};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::HashSet,
    path::Path,
    process::{Command, Stdio},
    sync::Arc,
};

pub fn convert_image(
    input: &str,
    output: &str,
    images: HashSet<Arc<str>>,
    args: Vec<String>,
    img_format: &str,
) {
    let pb = ProgressBar::new(images.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[2/2] [{elapsed_precise}] [{wide_bar:.green/red}]    {human_pos:.cyan}/{human_len:.blue} {spinner}",
        )
        .unwrap()
        .progress_chars("●>-"),
    );
    match Path::new(input).is_dir() {
        true => images.par_iter().for_each(|image| {
            let input: Arc<str> = input.ends_with_plus("/", image);

            let output: Arc<str> = output.img_to_img(image, img_format);

            let _ = command(input.as_ref(), output.as_ref(), args.clone());
            pb.inc(1);
        }),
        false => images.iter().for_each(|image| {
            let input: Arc<str> = Arc::from(input);

            let output: Arc<str> = output.output_to_img(image, img_format);
            pb.inc(1);
            let _ = command(input.as_ref(), output.as_ref(), args.clone());
        }),
    }
    pb.finish();
}

fn command(input: &str, output: &str, args: Vec<String>) {
    let mut args = args;
    let path = &output.output_clean();

    args.insert(0, input.to_string());
    args.push(output.to_string());

    if !Path::new(path).exists() {
        match Command::new("convert")
            .args(args)
            .stderr(Stdio::inherit())
            .spawn()
            .expect("convert not found in $PATH")
            .wait()
        {
            Ok(_) => (),
            Err(e) => println!("Error; {}", e),
        }
    }
}
