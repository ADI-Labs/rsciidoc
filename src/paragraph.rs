use nom::*;
use std::str;

#[derive(Debug, PartialEq)]
pub struct Paragraph<'a> {
    parts: Vec<Text<'a>>,
}

#[derive(Debug, PartialEq)]
enum Text<'a> {
    Plain(&'a str),
    Bold(Paragraph<'a>),
    Italics(Paragraph<'a>),
}

impl<'a> Paragraph<'a> {
    fn new(parts: Vec<Text>) -> Paragraph {
        Paragraph {parts: parts}
    }
    pub fn to_html(&self) -> String {
        self.parts.iter()
            .map(|text| match *text {
                Text::Plain(s)       => s.to_string(),
                Text::Bold(ref p)    => ["<strong>", &p.to_html(), "</strong>"].concat(),
                Text::Italics(ref p) => ["<em>", &p.to_html(), "</em>"].concat(),
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

named!(pub parse_paragraph<Paragraph>,
    map!(
        many1!(alt!(parse_bold | parse_italics | parse_plain)),
        Paragraph::new
    )
);

named!(parse_only_paragraph<Paragraph>, complete!(parse_paragraph));

named!(parse_plain<Text>,
    map!(
        map_res!(is_not!("_*"), str::from_utf8),
        Text::Plain
    )
);

named!(parse_bold<Text>,
    map_res!(
        delimited!(char!('*'), is_not!("*"), char!('*')),
        |bytes| {
            match parse_only_paragraph(bytes) {
                IResult::Done(_, output) => Ok(Text::Bold(output)),
                IResult::Error(err) => Err(err),
                _ => panic!("Nom complete! macro returned an IResult::Incomplete")
            }
        }
    )
);

named!(parse_italics<Text>,
    map_res!(
        delimited!(char!('_'), is_not!("_"), char!('_')),
        |bytes| {
            match parse_only_paragraph(bytes) {
                IResult::Done(_, output) => Ok(Text::Italics(output)),
                IResult::Error(err) => Err(err),
                _ => panic!("Nom complete! macro returned an IResult::Incomplete")
            }
        }
    )
);


#[test]
fn test_plain_parse() {
    if let IResult::Done(b"*World*", output) = parse_plain(b"Hello *World*") {
        assert_eq!(output, Text::Plain("Hello "));
    } else {
        panic!(r#" Plain parse failed with "Hello *World*" "#);
    }

    match parse_plain(b"*Hello World*") {
        IResult::Error(_) => (),
        _ => panic!(r#" Plain failed with "*Hello World*" "#),
    }
}

#[test]
fn test_bold_parse() {
    if let IResult::Done(b" World", output) = parse_bold(b"*Hello* World") {
        assert_eq!(output, Text::Bold(Paragraph::new(
            vec![Text::Plain("Hello")]
        )));
    } else {
        panic!(r#" Bold failed with "*Hello* World" "#);
    }

    match parse_bold(b"Hello World") {
        IResult::Error(_) => (),
        _ => panic!(r#" Bold failed with "Hello World" "#),
    };

    match parse_bold(b"*Hello World") {
        IResult::Incomplete(_) => (),
        _ => panic!(r#" Bold failed with "*Hello World" "#),
    }
}

#[test]
fn test_paragraph_parse() {
    let data = b"This *is _a_ serious* _test *of* this_ parser";
    if let IResult::Done(remaining, output) = parse_paragraph(data) {
        assert_eq!(remaining, b"");
        println!("{:#?}", output.parts);
        assert_eq!(output.parts, vec![
            Text::Plain("This "),
            Text::Bold(Paragraph::new(vec![
                Text::Plain("is "),
                Text::Italics(Paragraph::new(vec![Text::Plain("a")])),
                Text::Plain(" serious"),
            ])),
            Text::Plain(" "),
            Text::Italics(Paragraph::new(vec![
                Text::Plain("test "),
                Text::Bold(Paragraph::new(vec![Text::Plain("of")])),
                Text::Plain(" this"),
            ])),
            Text::Plain(" parser")
        ]);
    }
}

#[test]
fn test_html1() {
    let p = Paragraph::new(vec![
        Text::Plain("plaintext "),
        Text::Bold(Paragraph::new(vec![Text::Plain("bold text")]))
    ]);

    assert!(p.to_html() == "plaintext <strong>bold text</strong>".to_owned());
}

#[test]
fn test_integration() {
    let data = b"This *is _a_ serious* _test *of* this_ parser";
    if let IResult::Done(_, p) = parse_paragraph(data) {
        assert_eq!(p.to_html(),
            "This <strong>is <em>a</em> serious</strong> <em>test <strong>of</strong> this</em> parser"
        );
    } else {
        panic!("Parse failed");
    }
}
