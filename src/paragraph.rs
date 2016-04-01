pub struct Paragraph<'a> {
    parts: Vec<Text<'a>>,
}

pub enum Text<'a> {
    Plain(&'a str),      // TODO: make these &str and figure out lifetimes
    Italics(&'a str),
}

impl<'a> Paragraph<'a> {
    pub fn to_html(&self) -> String {
        self.parts.iter()
            .map(|text| match *text {
                Text::Plain(ref s) => s.to_string(),
                Text::Italics(ref s) => ["<em>", &s[..], "</em>"].concat(),
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_html1() {
        let p = Paragraph {
            parts: vec![Text::Plain("plaintext "),
                        Text::Italics("italics text")],
        };

        assert!(p.to_html() == "plaintext <em>italics text</em>".to_owned());
    }
}
