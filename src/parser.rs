use argparse::{ArgumentParser, Collect, Store};

#[derive(Debug)]
pub struct Parser {
    pub input: String,
    pub output: String,
    pub format: String,
    pub options: Vec<String>,
}

trait OutputHandler {
    fn output_check(&mut self, string: String);
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            input: String::new(),
            output: String::new(),
            format: String::from("jxl"),
            options: Vec::from(["-define".to_string(), "jxl:effort=1-4".to_string()]),
        }
    }

    pub fn args_parse(&mut self) {
        {
            let mut parser = ArgumentParser::new();

            parser.set_description("Bulk convert images using ImageMagick.");

            parser.stop_on_first_argument(true);

            parser
                .refer(&mut self.input)
                .add_option(&["--input", "-i"], Store, "Input Field: File/Directory")
                .required();

            parser.refer(&mut self.output).add_option(
                &["--output", "-o"],
                Store,
                "Output Field: File/Directory",
            );
            parser.refer(&mut self.format).add_option(
                &["--format", "-f"],
                Store,
                "Format for the output image. (Only required if --input is a directory)",
            );
            parser.refer(&mut self.options).add_argument(
                "args",
                Collect,
                "Arguments to pass to ImageMagick",
            );

            let _ = parser.parse_args_or_exit();
        }
        let _ = self.output.output_check(self.input.clone());
    }
}

impl OutputHandler for String {
    fn output_check(&mut self, string: String) {
        if self.is_empty() {
            self.push_str(&string)
        }
    }
}
