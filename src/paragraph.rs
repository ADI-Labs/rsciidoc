use nom::*;
use std::str;

#[derive(Debug)]
pub struct Paragraph<'a> {
    parts: Vec<Text<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Text<'a> {
    Plain(&'a str),
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

named!(italics<Text>,
    delimited!(
        char!('*'),
        chain!(
            bytes: is_not!("*") ~
            s:     expr_res!(str::from_utf8(bytes)),
            || { Text::Italics(s) }
        ),
        char!('*')
    )
);


#[test]
fn test_html1() {
    let p = Paragraph {
        parts: vec![Text::Plain("plaintext "),
                    Text::Italics("italics text")],
    };

    assert!(p.to_html() == "plaintext <em>italics text</em>".to_owned());
}

#[test]
fn test_italics_parse() {
    if let IResult::Done(input, output) = italics(b"*Hello* World") {
        assert_eq!(input, b" World");
        assert_eq!(output, Text::Italics("Hello"));
    } else {
        panic!(r#" Italics failed with "*Hello* World" "#);
    }

    match italics(b"Hello World") {
        IResult::Error(_) => (),
        _ => panic!(r#" Italics failed with "Hello World" "#),
    };

    match italics(b"*Hello World") {
        IResult::Incomplete(_) => (),
        _ => panic!(r#" Italics failed with "*Hello World" "#),
    }
}

#[test]
#[should_panic]                 // TODO: Fix
fn test_italics_escaped() {
    if let IResult::Done(input, output) = italics(br"*Hell\*o* World") {
        assert_eq!(output, Text::Italics("Hell*o"));
        assert_eq!(input, b" World");
    } else {
        panic!("Italics failed with \"*Hell\\*o* World");
    }
}
