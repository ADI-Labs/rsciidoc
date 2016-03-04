mod metadata;
mod paragraph;

pub struct Document {
    metadata: metadata::Metadata,
}

impl Document {
    pub fn new(text: &str) -> Document {
        Document {
            metadata: metadata::Metadata{},
        }
    }

    pub fn to_html(&self) -> String {
        let s = String::new();

        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(1 == 1);
    }
}
