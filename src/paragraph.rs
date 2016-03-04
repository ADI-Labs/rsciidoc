pub struct Paragraph {
    parts: Vec<Text>,
}

pub enum Text {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let p = Paragraph::new("hello world");
    }

    #[test]
    fn test_html() {
        let p = Paragraph {
            parts: vec![Text::Plain("plaintext".to_owned()),
                        Text::Bold("bold text".to_owned())],
        };
    }
}
