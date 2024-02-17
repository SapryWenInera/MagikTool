use argparse::{ArgumentParser, Collect, Store};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Parser {
    pub input: Box<Path>,
    pub output: Box<Path>,
    pub format: Box<str>,
    pub options: Box<str>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            input: PathBuf::new().into_boxed_path(),
            output: PathBuf::new().into_boxed_path(),
            format: Box::from(""),
            options: Box::from("-define jxl:effort=1-4"),
        }
    }

    pub fn args_parse(&mut self) {
        let mut input = String::from(self.input.to_string_lossy());
        let mut output = String::from(self.output.to_string_lossy());
        let mut format = String::from(self.format.as_ref());
        let mut options: Vec<String> = self
            .options
            .clone()
            .split_whitespace()
            .map(|s| String::from(s))
            .collect();
        {
            let mut parser = ArgumentParser::new();

            parser.set_description("Bulk convert images using ImageMagick.");

            parser.stop_on_first_argument(true);

            parser
                .refer(&mut input)
                .add_option(&["--input", "-i"], Store, "Input Field: File/Directory")
                .required();

            parser.refer(&mut output).add_option(
                &["--output", "-o"],
                Store,
                "Output Field: File/Directory",
            );
            parser.refer(&mut format).add_option(
                &["--format", "-f"],
                Store,
                "Format for the output image. (Only required if --input is a directory)",
            );
            parser.refer(&mut options).add_argument(
                "args",
                Collect,
                "Arguments to pass to ImageMagick",
            );

            parser.parse_args_or_exit();
        }
        dbg!(&input);
        dbg!(&output);
        dbg!(&format);
        dbg!(&options);
        self.input = PathBuf::from(input).into_boxed_path();
        self.output = if output.is_empty() {
            self.input.clone()
        } else {
            PathBuf::from(output).into_boxed_path()
        };
        self.format = if self.output.extension().is_none() && format.is_empty() {
            Box::from("jxl")
        } else if !format.is_empty() {
            Box::from(format)
        } else {
            Box::from(self.output.extension().unwrap().to_string_lossy())
        };
        self.options = options.join(" ").into_boxed_str();
    }
}
