use crate::err_handling::functions::Endswith;
use itertools::Itertools;
use std::ffi::OsStr;
use std::sync::Arc;

const EXTENSIONS: [&str; 7] = ["jpeg", "jpg", "jxl", "png", "heic", "png", "webp"];

pub trait ImgtoImg {
    fn extension_extract(&self) -> String;
    fn extension_replace(&self, format: &str) -> Arc<str>;
    fn img_to_img(&self, img: &Arc<str>, extension: &str) -> Arc<str>;
    fn output_clean(&self) -> String;
    fn output_to_img(&self, img: &Arc<str>, extension: &str) -> Arc<str>;
}

impl ImgtoImg for str {
    fn extension_extract(&self) -> String {
        let o: String = EXTENSIONS
            .iter()
            .filter(|&&extension| self.ends_with(extension))
            .join("");

        return o[o.len() / 2..].to_string();
    }
    fn extension_replace(&self, img_ext: &str) -> Arc<str> {
        let mut output = self.to_owned();

        let _ = EXTENSIONS.iter().for_each(|extension| {
            if self.ends_with(extension) {
                output = output.replace(extension, img_ext).into();
            }
        });
        return Arc::from(output);
    }

    fn img_to_img(&self, img: &Arc<str>, extension: &str) -> Arc<str> {
        let output = self.ends_with_plus("/", img);

        let o = format!("{}:{}", extension, output);

        return o.extension_replace(extension);
    }

    fn output_clean(&self) -> String {
        let mut o = String::from(self);
        let _ = EXTENSIONS.iter().for_each(|extension| {
            if self.starts_with(extension) {
                let format = format!("{}:", extension);

                o = self.replace(&format, "");
            }
        });
        return o;
    }

    fn output_to_img(&self, img: &Arc<str>, extension: &str) -> Arc<str> {
        let o = match img.ends_with(extension) {
            true => self.img_to_img(img, extension),
            false => match self.ends_with("/") {
                true => Arc::from(format!(
                    "{}:{}{}",
                    img.as_ref().extension_extract(),
                    self,
                    img
                )),
                false => Arc::from(format!(
                    "{}:{}/{}",
                    img.as_ref().extension_extract(),
                    self,
                    img
                )),
            },
        };

        return o;
    }
}

pub fn is_image(input: &OsStr) -> bool {
    let input = input.to_str().unwrap();

    if EXTENSIONS.contains(&input) {
        return true;
    }
    return false;
}
