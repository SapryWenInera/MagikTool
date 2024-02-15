use std::path::Path;

const EXTENSIONS: [&str; 6] = ["heic", "jpeg", "jpg", "jxl", "png", "webp"];

pub trait ImageManipulation {
    fn is_image(&self) -> Option<Box<Path>>;
}

impl ImageManipulation for Path {
    fn is_image(&self) -> Option<Box<Path>> {
        let extensions = EXTENSIONS;
        for extension in extensions {
            let r = match self.extension() {
                Some(v) => v.eq(extension),
                None => false 
            };

            if r {
                return Some(Box::from(self));
            }
        }

        None
    }
}