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
        self.parts.iter()
            .map(|text| match *text {
                Text::Plain(ref s) => s.to_string(),
                Text::Bold(ref s) => ["<strong>", &s[..], "</strong>"].concat(),
            })
            .collect::<Vec<_>>()
            .concat()
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
    fn test_html1() {
        let p = Paragraph {
            parts: vec![Text::Plain("plaintext ".to_owned()),
                        Text::Bold("bold text".to_owned())],
        };

        assert!(p.to_html() == "plaintext <strong>bold text</strong>".to_owned());
    }
}
