use std::path::PathBuf;

use argparse::{ArgumentParser, Collect, Store};

#[derive(Debug)]
pub struct Parser {
    pub input: PathBuf,
    pub output: PathBuf,
    pub format: Box<str>,
    pub options: Box<str>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            input: PathBuf::new(),
            output: PathBuf::new(),
            format: Box::from("jxl"),
            options: Box::from("-define jxl:effort=1-4"),
        }
    }

    pub fn args_parse(&mut self) {
        let mut input = String::from(self.input.to_str().unwrap());
        let mut output = String::from(self.output.to_str().unwrap());
        let mut format = String::from(self.format.as_ref());
        let mut options: Vec<String> = self.options.clone().split_whitespace().map(|s| String::from(s)).collect();
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

            let _ = parser.parse_args_or_exit();
        }

        self.input = PathBuf::from(input);
        self.output = if output.is_empty() {
            self.input.clone()
        } else {
            PathBuf::from(output)
        };
        self.format = Box::from(format);
        self.options = options.join(" ").into_boxed_str();
    }
}
