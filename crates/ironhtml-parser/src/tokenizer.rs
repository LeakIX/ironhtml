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
    RawText,
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
    /// Tag name for raw text mode (script, style, textarea, title, etc.)
    raw_text_tag: String,
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
            raw_text_tag: String::new(),
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
            // Switch to raw text mode for elements whose content is not HTML
            if !self_closing && Self::is_raw_text_element(&name) {
                self.raw_text_tag.clone_from(&name);
                self.state = State::RawText;
            }

            Token::StartTag {
                name,
                attributes: attrs,
                self_closing,
            }
        }
    }

    /// Check if an element uses raw text content (no child HTML parsing).
    fn is_raw_text_element(tag: &str) -> bool {
        matches!(tag, "script" | "style" | "textarea" | "title")
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

                State::RawText => {
                    // Look for `</tagname>` (case-insensitive) to end raw text
                    let remaining =
                        &self.input[self.chars.peek().map_or(self.input.len(), |(i, _)| *i)..];
                    let close_tag = {
                        let mut s = String::from("</");
                        s.push_str(&self.raw_text_tag);
                        s
                    };
                    if let Some(pos) = remaining.to_ascii_lowercase().find(&close_tag) {
                        // Check that the char after the tag name is '>' or whitespace or '/'
                        let after = pos + close_tag.len();
                        let valid_end = after >= remaining.len()
                            || matches!(
                                remaining.as_bytes().get(after),
                                Some(b'>' | b' ' | b'\t' | b'\n' | b'/' | 0x0C)
                            );

                        if valid_end {
                            // Emit all characters before the close tag
                            for _ in 0..pos {
                                if let Some(c) = self.consume() {
                                    self.pending_tokens.push(Token::Character(c));
                                }
                            }
                            // Now let the normal tokenizer handle `</tagname>`
                            self.raw_text_tag.clear();
                            self.state = State::Data;

                            if !self.pending_tokens.is_empty() {
                                return Some(self.pending_tokens.remove(0));
                            }
                            continue;
                        }
                    }

                    // No close tag found — emit rest as characters
                    if let Some(c) = self.consume() {
                        return Some(Token::Character(c));
                    }
                    self.raw_text_tag.clear();
                    self.state = State::Data;
                    return Some(Token::Eof);
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

    // ── Raw text element tests ──────────────────────────────────────

    #[test]
    fn test_script_raw_text() {
        let tokens: Vec<_> = Tokenizer::new("<script>var x = '<div>';</script>").collect();
        assert_eq!(
            tokens[0],
            Token::StartTag {
                name: "script".into(),
                attributes: vec![],
                self_closing: false,
            }
        );
        // Content should be raw characters, not parsed as tags
        let text: String = tokens[1..tokens.len() - 1]
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(text, "var x = '<div>';");
        assert_eq!(
            *tokens.last().unwrap(),
            Token::EndTag {
                name: "script".into()
            }
        );
    }

    #[test]
    fn test_style_raw_text() {
        let tokens: Vec<_> = Tokenizer::new("<style>p > .cls { color: red; }</style>").collect();
        assert_eq!(
            tokens[0],
            Token::StartTag {
                name: "style".into(),
                attributes: vec![],
                self_closing: false,
            }
        );
        let text: String = tokens[1..tokens.len() - 1]
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(text, "p > .cls { color: red; }");
    }

    #[test]
    fn test_script_with_inner_script_reference() {
        // <script> containing "</script" as part of a string should only
        // close on a real "</script>" end tag
        let tokens: Vec<_> =
            Tokenizer::new(r#"<script>var s = "<b>not a tag</b>";</script>"#).collect();
        let text: String = tokens[1..tokens.len() - 1]
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(text, r#"var s = "<b>not a tag</b>";"#);
    }

    #[test]
    fn test_textarea_raw_text() {
        let tokens: Vec<_> = Tokenizer::new("<textarea><b>bold</b> & stuff</textarea>").collect();
        let text: String = tokens[1..tokens.len() - 1]
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(text, "<b>bold</b> & stuff");
    }

    #[test]
    fn test_title_raw_text() {
        let tokens: Vec<_> = Tokenizer::new("<title>A <em>page</em></title>").collect();
        let text: String = tokens[1..tokens.len() - 1]
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(text, "A <em>page</em>");
    }
}
