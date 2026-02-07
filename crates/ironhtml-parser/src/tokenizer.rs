//! HTML5 tokenizer.
//!
//! This module implements the tokenization stage of HTML parsing as specified
//! in the WHATWG HTML Living Standard.
//!
//! ## Reference
//!
//! - [Tokenization](https://html.spec.whatwg.org/multipage/parsing.html#tokenization)

use alloc::string::String;
use alloc::vec::Vec;

/// A token produced by the tokenizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A DOCTYPE token.
    Doctype {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
    },
    /// A start tag token.
    StartTag {
        name: String,
        attributes: Vec<(String, String)>,
        self_closing: bool,
    },
    /// An end tag token.
    EndTag { name: String },
    /// A comment token.
    Comment(String),
    /// A character token (text content).
    Character(char),
    /// End of file.
    Eof,
}

/// The tokenizer state.
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    SelfClosingStartTag,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    MarkupDeclarationOpen,
    CommentStart,
    Comment,
    CommentEnd,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    BogusComment,
}

/// HTML5 tokenizer.
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: core::iter::Peekable<core::str::CharIndices<'a>>,
    state: State,
    current_tag_name: String,
    current_tag_is_end: bool,
    current_tag_self_closing: bool,
    current_attr_name: String,
    current_attr_value: String,
    current_attrs: Vec<(String, String)>,
    current_comment: String,
    current_doctype_name: Option<String>,
    pending_tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer for the given input.
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            state: State::Data,
            current_tag_name: String::new(),
            current_tag_is_end: false,
            current_tag_self_closing: false,
            current_attr_name: String::new(),
            current_attr_value: String::new(),
            current_attrs: Vec::new(),
            current_comment: String::new(),
            current_doctype_name: None,
            pending_tokens: Vec::new(),
        }
    }

    fn consume(&mut self) -> Option<char> {
        self.chars.next().map(|(_, c)| c)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
    }

    fn emit_current_tag(&mut self) -> Token {
        let name = core::mem::take(&mut self.current_tag_name).to_ascii_lowercase();
        let attrs = core::mem::take(&mut self.current_attrs);
        let self_closing = self.current_tag_self_closing;
        let is_end = self.current_tag_is_end;
        self.current_tag_self_closing = false;
        self.current_tag_is_end = false;

        if is_end {
            Token::EndTag { name }
        } else {
            Token::StartTag {
                name,
                attributes: attrs,
                self_closing,
            }
        }
    }

    fn emit_current_attr(&mut self) {
        if !self.current_attr_name.is_empty() {
            let name = core::mem::take(&mut self.current_attr_name).to_ascii_lowercase();
            let value = core::mem::take(&mut self.current_attr_value);
            self.current_attrs.push((name, value));
        }
    }

    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    fn next_token(&mut self) -> Option<Token> {
        // Return pending tokens first
        if !self.pending_tokens.is_empty() {
            return Some(self.pending_tokens.remove(0));
        }

        loop {
            match self.state {
                State::Data => match self.consume() {
                    Some('<') => self.state = State::TagOpen,
                    Some(c) => return Some(Token::Character(c)),
                    None => return Some(Token::Eof),
                },

                State::TagOpen => match self.peek() {
                    Some('!') => {
                        self.consume();
                        self.state = State::MarkupDeclarationOpen;
                    }
                    Some('/') => {
                        self.consume();
                        self.state = State::EndTagOpen;
                    }
                    Some(c) if c.is_ascii_alphabetic() => {
                        self.state = State::TagName;
                    }
                    _ => {
                        self.state = State::Data;
                        return Some(Token::Character('<'));
                    }
                },

                State::EndTagOpen => match self.peek() {
                    Some(c) if c.is_ascii_alphabetic() => {
                        self.current_tag_is_end = true;
                        self.state = State::TagName;
                    }
                    Some('>') => {
                        self.consume();
                        self.state = State::Data;
                    }
                    _ => {
                        self.state = State::BogusComment;
                    }
                },

                State::TagName => match self.consume() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.state = State::BeforeAttributeName;
                    }
                    Some('/') => {
                        self.state = State::SelfClosingStartTag;
                    }
                    Some('>') => {
                        self.state = State::Data;
                        return Some(self.emit_current_tag());
                    }
                    Some(c) => {
                        self.current_tag_name.push(c);
                    }
                    None => {
                        self.state = State::Data;
                        return Some(Token::Eof);
                    }
                },

                State::SelfClosingStartTag => match self.consume() {
                    Some('>') => {
                        self.current_tag_self_closing = true;
                        self.state = State::Data;
                        return Some(self.emit_current_tag());
                    }
                    _ => {
                        self.state = State::BeforeAttributeName;
                    }
                },

                State::BeforeAttributeName => match self.peek() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.consume();
                    }
                    Some('/' | '>') | None => {
                        self.state = State::AfterAttributeName;
                    }
                    Some('=') => {
                        self.consume();
                        self.current_attr_name.push('=');
                        self.state = State::AttributeName;
                    }
                    _ => {
                        self.state = State::AttributeName;
                    }
                },

                State::AttributeName => match self.peek() {
                    Some('\t' | '\n' | '\x0C' | ' ' | '/' | '>') => {
                        self.state = State::AfterAttributeName;
                    }
                    Some('=') => {
                        self.consume();
                        self.state = State::BeforeAttributeValue;
                    }
                    Some(c) => {
                        self.consume();
                        self.current_attr_name.push(c);
                    }
                    None => {
                        self.state = State::AfterAttributeName;
                    }
                },

                State::AfterAttributeName => match self.peek() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.consume();
                    }
                    Some('/') => {
                        self.emit_current_attr();
                        self.consume();
                        self.state = State::SelfClosingStartTag;
                    }
                    Some('=') => {
                        self.consume();
                        self.state = State::BeforeAttributeValue;
                    }
                    Some('>') => {
                        self.emit_current_attr();
                        self.consume();
                        self.state = State::Data;
                        return Some(self.emit_current_tag());
                    }
                    _ => {
                        self.emit_current_attr();
                        self.state = State::AttributeName;
                    }
                },

                State::BeforeAttributeValue => match self.peek() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.consume();
                    }
                    Some('"') => {
                        self.consume();
                        self.state = State::AttributeValueDoubleQuoted;
                    }
                    Some('\'') => {
                        self.consume();
                        self.state = State::AttributeValueSingleQuoted;
                    }
                    Some('>') => {
                        self.emit_current_attr();
                        self.consume();
                        self.state = State::Data;
                        return Some(self.emit_current_tag());
                    }
                    _ => {
                        self.state = State::AttributeValueUnquoted;
                    }
                },

                State::AttributeValueDoubleQuoted => match self.consume() {
                    Some('"') => {
                        self.emit_current_attr();
                        self.state = State::AfterAttributeValueQuoted;
                    }
                    Some(c) => {
                        self.current_attr_value.push(c);
                    }
                    None => {
                        self.emit_current_attr();
                        self.state = State::Data;
                        return Some(Token::Eof);
                    }
                },

                State::AttributeValueSingleQuoted => match self.consume() {
                    Some('\'') => {
                        self.emit_current_attr();
                        self.state = State::AfterAttributeValueQuoted;
                    }
                    Some(c) => {
                        self.current_attr_value.push(c);
                    }
                    None => {
                        self.emit_current_attr();
                        self.state = State::Data;
                        return Some(Token::Eof);
                    }
                },

                State::AttributeValueUnquoted => match self.peek() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.emit_current_attr();
                        self.consume();
                        self.state = State::BeforeAttributeName;
                    }
                    Some('>') => {
                        self.emit_current_attr();
                        self.consume();
                        self.state = State::Data;
                        return Some(self.emit_current_tag());
                    }
                    Some(c) => {
                        self.consume();
                        self.current_attr_value.push(c);
                    }
                    None => {
                        self.emit_current_attr();
                        self.state = State::Data;
                        return Some(Token::Eof);
                    }
                },

                State::AfterAttributeValueQuoted => match self.peek() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.consume();
                        self.state = State::BeforeAttributeName;
                    }
                    Some('/') => {
                        self.consume();
                        self.state = State::SelfClosingStartTag;
                    }
                    Some('>') => {
                        self.consume();
                        self.state = State::Data;
                        return Some(self.emit_current_tag());
                    }
                    _ => {
                        self.state = State::BeforeAttributeName;
                    }
                },

                State::MarkupDeclarationOpen => {
                    // Check for DOCTYPE or comment
                    let remaining =
                        &self.input[self.chars.peek().map_or(self.input.len(), |(i, _)| *i)..];

                    if remaining.starts_with("--") {
                        self.consume(); // -
                        self.consume(); // -
                        self.state = State::CommentStart;
                    } else if remaining.to_ascii_uppercase().starts_with("DOCTYPE") {
                        for _ in 0..7 {
                            self.consume();
                        }
                        self.state = State::Doctype;
                    } else {
                        self.state = State::BogusComment;
                    }
                }

                State::CommentStart => match self.peek() {
                    Some('-') => {
                        self.consume();
                        self.state = State::CommentEnd;
                    }
                    Some('>') => {
                        self.consume();
                        self.state = State::Data;
                        return Some(Token::Comment(core::mem::take(&mut self.current_comment)));
                    }
                    _ => {
                        self.state = State::Comment;
                    }
                },

                State::Comment => match self.consume() {
                    Some('-') => {
                        self.state = State::CommentEnd;
                    }
                    Some(c) => {
                        self.current_comment.push(c);
                    }
                    None => {
                        self.state = State::Data;
                        return Some(Token::Comment(core::mem::take(&mut self.current_comment)));
                    }
                },

                State::CommentEnd => match self.consume() {
                    Some('-') => {
                        if self.peek() == Some('>') {
                            self.consume();
                            self.state = State::Data;
                            return Some(Token::Comment(core::mem::take(
                                &mut self.current_comment,
                            )));
                        }
                        self.current_comment.push('-');
                    }
                    Some(c) => {
                        self.current_comment.push('-');
                        self.current_comment.push(c);
                        self.state = State::Comment;
                    }
                    None => {
                        self.state = State::Data;
                        return Some(Token::Comment(core::mem::take(&mut self.current_comment)));
                    }
                },

                State::Doctype => match self.peek() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.consume();
                        self.state = State::BeforeDoctypeName;
                    }
                    Some('>') => {
                        self.state = State::BeforeDoctypeName;
                    }
                    _ => {
                        self.state = State::BeforeDoctypeName;
                    }
                },

                State::BeforeDoctypeName => match self.peek() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.consume();
                    }
                    Some('>') => {
                        self.consume();
                        self.state = State::Data;
                        return Some(Token::Doctype {
                            name: self.current_doctype_name.take(),
                            public_id: None,
                            system_id: None,
                        });
                    }
                    Some(_) => {
                        self.current_doctype_name = Some(String::new());
                        self.state = State::DoctypeName;
                    }
                    None => {
                        self.state = State::Data;
                        return Some(Token::Doctype {
                            name: None,
                            public_id: None,
                            system_id: None,
                        });
                    }
                },

                State::DoctypeName => match self.consume() {
                    Some('\t' | '\n' | '\x0C' | ' ') => {
                        self.state = State::AfterDoctypeName;
                    }
                    Some('>') => {
                        self.state = State::Data;
                        return Some(Token::Doctype {
                            name: self
                                .current_doctype_name
                                .take()
                                .map(|s| s.to_ascii_lowercase()),
                            public_id: None,
                            system_id: None,
                        });
                    }
                    Some(c) => {
                        if let Some(ref mut name) = self.current_doctype_name {
                            name.push(c);
                        }
                    }
                    None => {
                        self.state = State::Data;
                        return Some(Token::Doctype {
                            name: self.current_doctype_name.take(),
                            public_id: None,
                            system_id: None,
                        });
                    }
                },

                State::AfterDoctypeName => {
                    match self.peek() {
                        Some('\t' | '\n' | '\x0C' | ' ') => {
                            self.consume();
                        }
                        Some('>') => {
                            self.consume();
                            self.state = State::Data;
                            return Some(Token::Doctype {
                                name: self
                                    .current_doctype_name
                                    .take()
                                    .map(|s| s.to_ascii_lowercase()),
                                public_id: None,
                                system_id: None,
                            });
                        }
                        None => {
                            self.state = State::Data;
                            return Some(Token::Doctype {
                                name: self.current_doctype_name.take(),
                                public_id: None,
                                system_id: None,
                            });
                        }
                        _ => {
                            // Skip PUBLIC/SYSTEM identifiers for now
                            while let Some(c) = self.peek() {
                                if c == '>' {
                                    break;
                                }
                                self.consume();
                            }
                            self.consume(); // consume >
                            self.state = State::Data;
                            return Some(Token::Doctype {
                                name: self
                                    .current_doctype_name
                                    .take()
                                    .map(|s| s.to_ascii_lowercase()),
                                public_id: None,
                                system_id: None,
                            });
                        }
                    }
                }

                State::BogusComment => match self.consume() {
                    Some('>') => {
                        self.state = State::Data;
                        return Some(Token::Comment(core::mem::take(&mut self.current_comment)));
                    }
                    Some(c) => {
                        self.current_comment.push(c);
                    }
                    None => {
                        self.state = State::Data;
                        return Some(Token::Comment(core::mem::take(&mut self.current_comment)));
                    }
                },
            }
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Some(Token::Eof) => None,
            token => token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_simple_element() {
        let mut tokenizer = Tokenizer::new("<div></div>");
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "div".into(),
                attributes: vec![],
                self_closing: false,
            })
        );
        assert_eq!(tokenizer.next(), Some(Token::EndTag { name: "div".into() }));
    }

    #[test]
    fn test_element_with_text() {
        let mut tokenizer = Tokenizer::new("<p>Hello</p>");
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "p".into(),
                attributes: vec![],
                self_closing: false,
            })
        );
        assert_eq!(tokenizer.next(), Some(Token::Character('H')));
        assert_eq!(tokenizer.next(), Some(Token::Character('e')));
        assert_eq!(tokenizer.next(), Some(Token::Character('l')));
        assert_eq!(tokenizer.next(), Some(Token::Character('l')));
        assert_eq!(tokenizer.next(), Some(Token::Character('o')));
        assert_eq!(tokenizer.next(), Some(Token::EndTag { name: "p".into() }));
    }

    #[test]
    fn test_attributes() {
        let mut tokenizer = Tokenizer::new(r#"<div class="container" id="main">"#);
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "div".into(),
                attributes: vec![
                    ("class".into(), "container".into()),
                    ("id".into(), "main".into()),
                ],
                self_closing: false,
            })
        );
    }

    #[test]
    fn test_self_closing() {
        let mut tokenizer = Tokenizer::new("<br/>");
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "br".into(),
                attributes: vec![],
                self_closing: true,
            })
        );
    }

    #[test]
    fn test_doctype() {
        let mut tokenizer = Tokenizer::new("<!DOCTYPE html>");
        assert_eq!(
            tokenizer.next(),
            Some(Token::Doctype {
                name: Some("html".into()),
                public_id: None,
                system_id: None,
            })
        );
    }

    #[test]
    fn test_comment() {
        let mut tokenizer = Tokenizer::new("<!-- This is a comment -->");
        assert_eq!(
            tokenizer.next(),
            Some(Token::Comment(" This is a comment ".into()))
        );
    }

    #[test]
    fn test_boolean_attribute() {
        let mut tokenizer = Tokenizer::new("<input disabled>");
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "input".into(),
                attributes: vec![("disabled".into(), String::new())],
                self_closing: false,
            })
        );
    }

    #[test]
    fn test_unquoted_attribute() {
        let mut tokenizer = Tokenizer::new("<div class=container>");
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "div".into(),
                attributes: vec![("class".into(), "container".into())],
                self_closing: false,
            })
        );
    }
}
