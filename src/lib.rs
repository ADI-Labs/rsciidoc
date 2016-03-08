mod metadata;
mod paragraph;

pub struct Document {
    metadata: metadata::Metadata,
}

impl Document {
    pub fn new(text: &str) -> Document {
        Document {
            metadata: metadata::Metadata{ author: "".to_owned() },
        }
    }

    pub fn to_html(&self) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(1 == 1);
    }
}
