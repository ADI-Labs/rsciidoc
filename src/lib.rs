mod metadata;
mod paragraph;
mod list;

pub struct Document<'a> {
    metadata: metadata::Metadata,
    content: Vec<DocumentPart<'a>>,
}

pub enum DocumentPart<'a> {
    Paragraph(paragraph::Paragraph<'a>),
    List(list::List),
}

impl<'a> Document<'a> {
    pub fn new(text: &'a str) -> Document<'a> {
        Document {
            metadata: metadata::Metadata{ author: "".to_owned() },
            content: Vec::new(),
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
