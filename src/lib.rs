#[macro_use]
extern crate nom;

use nom::IResult;

mod paragraph;

pub struct Document<'a> {
    content: Vec<DocPart<'a>>,
}

enum DocPart<'a> {
    Paragraph(paragraph::Paragraph<'a>),
}

named!(parse_document<Document>,
    complete!(map!(
        many1!(
            map!(paragraph::parse_paragraph, DocPart::Paragraph)
        ),
        |vec| Document {content: vec}
    ))
);

impl<'a> Document<'a> {
    pub fn new(text: &'a str) -> Document<'a> {
        match parse_document(text.as_bytes()) {
            IResult::Done(_, output) => output,
            _ => panic!("Parsing failed")
        }
    }

    pub fn to_html(&self) -> String {
        self.content.iter()
            .map(|part| match *part {
                DocPart::Paragraph(ref p) => p.to_html()
            })
            .collect::<Vec<_>>()
            .concat()
    }
}
