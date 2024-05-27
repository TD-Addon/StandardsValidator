use super::Context;
use crate::{handlers::Handler, util::is_marker};
use std::{error::Error, fmt};
use tes3::esp::{Book, TES3Object};

pub struct BookValidator {}

impl Handler<'_> for BookValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        if let TES3Object::Book(book) = record {
            if is_marker(book) {
                return;
            }
            if !book.text.is_empty() {
                let mut parser = Parser::new(book);
                if let Err(e) = parser.parse(&book.text) {
                    println!(
                        "Failed to parse HTML in {} {} at index {}",
                        book.id, e.message, e.index
                    );
                } else if !parser.invisible.is_empty() {
                    println!(
                        "Book {} contains invisible text {}",
                        book.id, parser.invisible
                    );
                }
            }
        }
    }
}

enum ParseState {
    None,
    OpenTag,
    CloseTag,
    Attributes,
    AttributeValue,
    SelfClosing,
}

#[derive(Debug)]
struct ParseError {
    message: String,
    index: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at index {}", self.message, self.index)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        return self.message.as_str();
    }
}

impl ParseError {
    pub fn new(message: String, index: usize) -> ParseError {
        ParseError { message, index }
    }
}

struct Slice {
    pub start: usize,
    pub end: usize,
}

impl Slice {
    pub fn new() -> Self {
        Self { start: 1, end: 0 }
    }

    pub fn get<'a>(&self, string: &'a str) -> Option<&'a str> {
        if self.end <= self.start {
            return None;
        }
        unsafe {
            return Some(string.get_unchecked(self.start..self.end));
        }
    }

    pub fn get_or_empty<'a>(&self, string: &'a str) -> &'a str {
        return self.get(string).unwrap_or_default();
    }

    pub fn clear(&mut self) {
        self.start = 1;
        self.end = 0;
    }

    pub fn append(&mut self, i: usize, c: char) {
        if self.end < self.start {
            self.start = i;
        }
        self.end = i + c.len_utf8();
    }
}

const TAGS: [&str; 7] = ["div", "font", "br", "p", "img", "b", "deprecated"]; //ok, so maybe that last one isn't real

struct Parser<'a> {
    record: &'a Book,
    img: bool,
    invisible: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(record: &'a Book) -> Parser<'a> {
        Parser {
            record,
            img: false,
            invisible: "",
        }
    }

    fn parse(&mut self, html: &'a str) -> Result<(), ParseError> {
        let mut state = ParseState::None;
        let mut tag = Slice::new();
        let mut text = Slice::new();
        let mut attribute = Slice::new();
        let mut value = Slice::new();
        let mut stack: Vec<&str> = Vec::new();
        for (offset, c) in html.char_indices() {
            match state {
                ParseState::None => {
                    if c == '<' {
                        state = ParseState::OpenTag;
                        tag.clear();
                        if let Some(t) = text.get(html) {
                            self.on_text(t);
                            text.clear();
                        }
                    } else {
                        text.append(offset, c);
                    }
                }
                ParseState::OpenTag => {
                    if c == '/' {
                        if stack.is_empty() {
                            return Err(ParseError::new("Unexpected /".to_string(), offset));
                        }
                        state = ParseState::CloseTag;
                        tag.clear();
                    } else if c == '<' {
                        return Err(ParseError::new("Unexpected <".to_string(), offset));
                    } else if c == '>' {
                        let elem = tag.get_or_empty(html);
                        self.on_element(elem);
                        state = ParseState::None;
                        stack.push(elem);
                    } else if c.is_whitespace() {
                        self.on_element(tag.get_or_empty(html));
                        state = ParseState::Attributes;
                        attribute.clear();
                    } else {
                        tag.append(offset, c);
                    }
                }
                ParseState::Attributes => {
                    if c == '=' {
                        if attribute.get(html).is_none() {
                            return Err(ParseError::new("Unexpected =".to_string(), offset));
                        }
                        state = ParseState::AttributeValue;
                        value.clear();
                    } else if c == '/' {
                        state = ParseState::SelfClosing;
                    } else if c == '>' {
                        self.on_element(tag.get_or_empty(html));
                        state = ParseState::None;
                        stack.push(tag.get_or_empty(html));
                    } else if c.is_whitespace() {
                        if let Some(a) = attribute.get(html) {
                            self.on_attribute(a, value.get_or_empty(html));
                            attribute.clear();
                            value.clear();
                        }
                    } else {
                        attribute.append(offset, c);
                    }
                }
                ParseState::SelfClosing => {
                    if c != '>' {
                        return Err(ParseError::new("Expected >".to_string(), offset));
                    }
                    if let Some(e) = stack.pop() {
                        self.on_close(e);
                    }
                    state = ParseState::None;
                }
                ParseState::AttributeValue => {
                    if c == '"' {
                        if let Some(v) = value.get(html) {
                            if !v.starts_with('"') {
                                return Err(ParseError::new("Unexpected \"".to_string(), offset));
                            }
                            unsafe {
                                self.on_attribute(
                                    attribute.get_or_empty(html),
                                    v.get_unchecked(1usize..),
                                );
                            }
                            state = ParseState::Attributes;
                            value.clear();
                        }
                        value.start = offset;
                        value.end = offset + 1;
                    } else if !value.get_or_empty(html).starts_with('"') {
                        if c == '/' {
                            state = ParseState::SelfClosing;
                        } else if c == '>' {
                            let elem = tag.get_or_empty(html);
                            self.on_element(elem);
                            state = ParseState::None;
                            stack.push(elem);
                        } else if c.is_whitespace() {
                            if value.get(html).is_none() {
                                return Err(ParseError::new(
                                    "Unexpected space".to_string(),
                                    offset,
                                ));
                            }
                            self.on_attribute(
                                attribute.get_or_empty(html),
                                value.get_or_empty(html),
                            );
                            state = ParseState::Attributes;
                            attribute.clear();
                            value.clear();
                        } else {
                            value.append(offset, c);
                        }
                    } else {
                        value.append(offset, c);
                    }
                }
                ParseState::CloseTag => {
                    if c == '>' {
                        let elem = tag.get_or_empty(html);
                        if let Some(initial) = stack.pop() {
                            let mut expected = initial;
                            while expected != elem {
                                if let Some(s) = stack.pop() {
                                    expected = s;
                                } else {
                                    break;
                                }
                            }
                            if elem != expected {
                                return Err(ParseError::new(
                                    "Unexpected </".to_string()
                                        + elem
                                        + "> expected </"
                                        + initial
                                        + ">",
                                    offset,
                                ));
                            }
                            self.on_close(initial);
                            state = ParseState::None;
                        } else {
                            return Err(ParseError::new(
                                "Unexpected </".to_string() + elem + ">",
                                offset,
                            ));
                        }
                    } else {
                        tag.append(offset, c);
                    }
                }
            };
        }
        return match state {
            ParseState::CloseTag => Err(ParseError::new(
                "Unfinished closing tag </".to_string() + tag.get_or_empty(html),
                tag.start,
            )),
            ParseState::None => {
                if let Some(t) = text.get(html) {
                    self.on_text(t);
                }
                return Ok(());
            }
            _ => Err(ParseError::new(
                "Unfinished opening tag <".to_string() + tag.get_or_empty(html),
                tag.start,
            )),
        };
    }

    fn on_element(&mut self, tag: &str) {
        self.invisible = "";
        let lower = tag.to_ascii_lowercase();
        if !TAGS.contains(&lower.as_str()) {
            println!(
                "Book {} contains invalid HTML opening tag <{}>",
                self.record.id, tag
            );
        }
        self.img = lower == "img";
    }

    fn on_attribute(&mut self, attribute: &str, value: &str) {
        if self.img && attribute.eq_ignore_ascii_case("src") && value.contains('/') {
            println!("Book {} contains invalid IMG SRC {}", self.record.id, value);
        }
    }

    fn on_text(&mut self, text: &'a str) {
        self.invisible = text.trim();
    }

    fn on_close(&mut self, tag: &str) {
        if !TAGS.contains(&tag.to_ascii_lowercase().as_str()) {
            println!(
                "Book {} contains invalid HTML closing tag <{}>",
                self.record.id, tag
            );
        }
        self.img = false;
    }
}
