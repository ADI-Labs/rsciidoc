pub struct Paragraph {
    parts: Vec<Text>,
}

enum Text {
    Plain(String),      // TODO: make these &str and figure out lifetimes
    Bold(String),
}

impl Paragraph {
    pub fn new(text: &str) -> Paragraph {
        Paragraph {
            parts: Vec::new(),
        }
    }

    pub fn to_html(&self) -> String {
        let s = String::new();

        s
    }
}
