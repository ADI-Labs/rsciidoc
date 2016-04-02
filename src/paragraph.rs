use nom::*;
use std::str;

#[derive(Debug, PartialEq)]
pub struct Paragraph<'a> {
    parts: Vec<Text<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Text<'a> {
    Plain(&'a str),
    Bold(Paragraph<'a>),
}

impl<'a> Paragraph<'a> {
    pub fn to_html(&self) -> String {
        self.parts.iter()
            .map(|text| match *text {
                Text::Plain(ref s) => s.to_string(),
                Text::Bold(ref p) => ["<strong>", &p.to_html(), "</strong>"].concat(),
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

named!(only_paragraph<Paragraph>, complete!(paragraph));

named!(plain<Text>,
    chain!(
        s: map_res!(is_not!("*"), str::from_utf8),
        || { Text::Plain(s) }
    )
);

named!(bold<Text>,
    map_res!(
        delimited!(char!('*'), is_not!("*"), char!('*')),
        |bytes| {
            match only_paragraph(bytes) {
                IResult::Done(input, output) => Ok(Text::Bold(output)),
                IResult::Error(err) => Err(err),
                _ => panic!("Nom complete! macro returned an IResult::Incomplete")
            }
        }
    )
);

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
fn test_bold_parse() {
    if let IResult::Done(input, output) = bold(b"*Hello* World") {
        assert_eq!(input, b" World");
        assert_eq!(output, Text::Bold(Paragraph {
            parts: vec![Text::Plain("Hello")]
        }));
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
fn test_paragraph_parse() {
    if let IResult::Done(input, output) = paragraph(b"Hello *World*") {
        assert_eq!(input, b"");
        assert_eq!(output.parts, vec![
            Text::Plain("Hello "),
            Text::Bold(Paragraph {parts: vec![Text::Plain("World")]}),
        ]);
    } else {
        panic!(r#" Paragraph parse failed with "Hello *World*" "#);
    }

    if let IResult::Done(input, output) = paragraph(b"*This* *is* *a* *test*") {
        assert_eq!(input, b"");
        assert_eq!(output.parts, vec![
            Text::Bold(Paragraph {parts: vec![Text::Plain("This")]}),
            Text::Plain(" "),
            Text::Bold(Paragraph {parts: vec![Text::Plain("is")]}),
            Text::Plain(" "),
            Text::Bold(Paragraph {parts: vec![Text::Plain("a")]}),
            Text::Plain(" "),
            Text::Bold(Paragraph {parts: vec![Text::Plain("test")]}),
        ]);
    } else {
        panic!(r#" Paragraph parse failed with "Hello *World*" "#);
    }
}

#[test]
fn test_html1() {
    let p = Paragraph { parts: vec![
            Text::Plain("plaintext "),
            Text::Bold(Paragraph { parts: vec![Text::Plain("bold text")]})
    ]};

    assert!(p.to_html() == "plaintext <strong>bold text</strong>".to_owned());
}
