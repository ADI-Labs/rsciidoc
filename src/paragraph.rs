use nom::*;
use std::str;

#[derive(Debug)]
pub struct Paragraph<'a> {
    parts: Vec<Text<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Text<'a> {
    Plain(&'a str),
    Bold(&'a str),
}

impl<'a> Paragraph<'a> {
    pub fn to_html(&self) -> String {
        self.parts.iter()
            .map(|text| match *text {
                Text::Plain(ref s) => s.to_string(),
                Text::Bold(ref s) => ["<em>", &s[..], "</em>"].concat(),
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

named!(paragraph<Paragraph>,
    chain!(
        vec: many1!(alt!(plain | bold)),
        || { Paragraph { parts: vec } }
    )
);

named!(plain<Text>,
    chain!(
        bytes: is_not!("*") ~
        s:     expr_res!(str::from_utf8(bytes)),
        || { Text::Plain(s) }
    )
);

named!(bold<Text>,
    delimited!(
        char!('*'),
        chain!(
            bytes: is_not!("*") ~
            s:     expr_res!(str::from_utf8(bytes)),
            || { Text::Bold(s) }
        ),
        char!('*')
    )
);


#[test]
fn test_paragraph_parse() {
    if let IResult::Done(input, output) = paragraph(b"Hello *World*") {
        assert_eq!(input, b"");
        assert_eq!(output.parts, vec![Text::Plain("Hello "),
                                      Text::Bold("World")]);
    } else {
        panic!(r#" Paragraph parse failed with "Hello *World*" "#);
    }

    if let IResult::Done(input, output) = paragraph(b"*This* is *a test*") {
        assert_eq!(input, b"");
        assert_eq!(output.parts, vec![Text::Bold("This"),
                                      Text::Plain(" is "),
                                      Text::Bold("a test")]);
    } else {
        panic!(r#" Paragraph parse failed with "Hello *World*" "#);
    }
}

#[test]
fn test_plain_parse() {
    if let IResult::Done(input, output) = plain(b"Hello *World*") {
        assert_eq!(input, b"*World*");
        assert_eq!(output, Text::Plain("Hello "));
    } else {
        panic!(r#" Plain parse failed with "Hello *World*" "#);
    }

    match plain(b"*Hello World*") {
        IResult::Error(_) => (),
        _ => panic!(r#" Plain failed with "*Hello World*" "#),
    }
}

#[test]
fn test_html1() {
    let p = Paragraph {
        parts: vec![Text::Plain("plaintext "),
                    Text::Bold("bold text")],
    };

    assert!(p.to_html() == "plaintext <em>bold text</em>".to_owned());
}

#[test]
fn test_bold_parse() {
    if let IResult::Done(input, output) = bold(b"*Hello* World") {
        assert_eq!(input, b" World");
        assert_eq!(output, Text::Bold("Hello"));
    } else {
        panic!(r#" Bold failed with "*Hello* World" "#);
    }

    match bold(b"Hello World") {
        IResult::Error(_) => (),
        _ => panic!(r#" Bold failed with "Hello World" "#),
    };

    match bold(b"*Hello World") {
        IResult::Incomplete(_) => (),
        _ => panic!(r#" Bold failed with "*Hello World" "#),
    }
}

#[test]
#[should_panic]                 // TODO: Fix
fn test_bold_escaped() {
    if let IResult::Done(input, output) = bold(br"*Hell\*o* World") {
        assert_eq!(output, Text::Bold("Hell*o"));
        assert_eq!(input, b" World");
    } else {
        panic!("Bold failed with \"*Hell\\*o* World");
    }
}
