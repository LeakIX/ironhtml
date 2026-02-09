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

use crate::entities;

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

    /// Return the unconsumed portion of the input starting at the
    /// current peek position.
    fn remaining(&mut self) -> &'a str {
        let offset = self.chars.peek().map_or(self.input.len(), |(i, _)| *i);
        &self.input[offset..]
    }

    /// Try to consume an HTML character reference starting *after*
    /// the `&` that has already been consumed.
    ///
    /// Returns decoded characters on success, or `None` if the
    /// sequence is not a valid reference (caller should emit `&`
    /// literally).
    fn consume_character_reference(&mut self, in_attribute: bool) -> Option<&'static [char]> {
        match self.peek() {
            Some('#') => {
                self.consume(); // '#'
                self.consume_numeric_reference().map(core::slice::from_ref)
            }
            Some(c) if c.is_ascii_alphanumeric() => self.consume_named_reference(in_attribute),
            _ => None,
        }
    }

    /// Consume a numeric character reference (`&#...;` or `&#x...;`).
    /// The `#` has already been consumed.
    fn consume_numeric_reference(&mut self) -> Option<&'static char> {
        let hex = matches!(self.peek(), Some('x' | 'X'));
        if hex {
            self.consume(); // 'x' or 'X'
        }

        let remaining = self.remaining();
        let mut len = 0;
        for ch in remaining.chars() {
            let valid = if hex {
                ch.is_ascii_hexdigit()
            } else {
                ch.is_ascii_digit()
            };
            if valid {
                len += ch.len_utf8();
            } else {
                break;
            }
        }

        if len == 0 {
            return None;
        }

        let digits = &remaining[..len];
        let codepoint = if hex {
            u32::from_str_radix(digits, 16).ok()?
        } else {
            digits.parse::<u32>().ok()?
        };

        // Consume the digit characters
        for _ in digits.chars() {
            self.consume();
        }

        // Consume trailing ';' if present
        if self.peek() == Some(';') {
            self.consume();
        }

        // Per WHATWG: null → U+FFFD, surrogates/out-of-range → U+FFFD
        let ch = if codepoint == 0 {
            '\u{FFFD}'
        } else {
            char::from_u32(codepoint).unwrap_or('\u{FFFD}')
        };

        // Leak a single-char allocation so we can return &'static.
        // This is a small, bounded cost: numeric refs are rare and
        // the set of distinct codepoints encountered is finite.
        Some(alloc::boxed::Box::leak(alloc::boxed::Box::new(ch)))
    }

    /// Consume a named character reference. The first alphanumeric
    /// character has NOT been consumed yet (it was only peeked).
    fn consume_named_reference(&mut self, in_attribute: bool) -> Option<&'static [char]> {
        let remaining = self.remaining();

        // Collect the longest alphanumeric prefix
        let mut name_len = 0;
        for ch in remaining.chars() {
            if ch.is_ascii_alphanumeric() {
                name_len += ch.len_utf8();
            } else {
                break;
            }
        }

        if name_len == 0 {
            return None;
        }

        // Try longest-match first, shrink until we find one
        let name_str = &remaining[..name_len];
        let mut match_len = name_str.len();
        while match_len > 0 {
            let candidate = &name_str[..match_len];
            if let Some(chars) = entities::lookup(candidate) {
                // Check for trailing ';'
                let has_semi = remaining.as_bytes().get(match_len) == Some(&b';');

                // WHATWG: in attributes, if no ';' and next char
                // is '=' or alphanumeric, don't decode
                if in_attribute && !has_semi {
                    let next = remaining.as_bytes().get(match_len);
                    if matches!(next, Some(b'=' | b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9')) {
                        return None;
                    }
                }

                // Consume the matched name chars
                for _ in candidate.chars() {
                    self.consume();
                }
                // Consume ';' if present
                if has_semi {
                    self.consume();
                }

                return Some(chars);
            }

            // Shrink: try next shorter prefix
            match_len = candidate[..match_len]
                .char_indices()
                .rev()
                .nth(0)
                .map_or(0, |(i, _)| i);
        }

        None
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
                    Some('&') => {
                        if let Some(chars) = self.consume_character_reference(false) {
                            let first = chars[0];
                            for &ch in &chars[1..] {
                                self.pending_tokens.push(Token::Character(ch));
                            }
                            return Some(Token::Character(first));
                        }
                        return Some(Token::Character('&'));
                    }
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
                    Some('&') => {
                        if let Some(chars) = self.consume_character_reference(true) {
                            for &ch in chars {
                                self.current_attr_value.push(ch);
                            }
                        } else {
                            self.current_attr_value.push('&');
                        }
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
                    Some('&') => {
                        if let Some(chars) = self.consume_character_reference(true) {
                            for &ch in chars {
                                self.current_attr_value.push(ch);
                            }
                        } else {
                            self.current_attr_value.push('&');
                        }
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
                    Some('&') => {
                        self.consume();
                        if let Some(chars) = self.consume_character_reference(true) {
                            for &ch in chars {
                                self.current_attr_value.push(ch);
                            }
                        } else {
                            self.current_attr_value.push('&');
                        }
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

    // ── character reference tests ────────────────────────────────────

    #[test]
    fn test_named_entity_in_text() {
        let mut tokenizer = Tokenizer::new("a&amp;b");
        assert_eq!(tokenizer.next(), Some(Token::Character('a')));
        assert_eq!(tokenizer.next(), Some(Token::Character('&')));
        assert_eq!(tokenizer.next(), Some(Token::Character('b')));
    }

    #[test]
    fn test_named_entity_lt_gt() {
        let tokens: Vec<_> = Tokenizer::new("&lt;div&gt;").collect();
        let chars: alloc::string::String = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(chars, "<div>");
    }

    #[test]
    fn test_numeric_decimal_entity() {
        let mut tokenizer = Tokenizer::new("&#65;");
        assert_eq!(tokenizer.next(), Some(Token::Character('A')));
    }

    #[test]
    fn test_numeric_hex_entity() {
        let mut tokenizer = Tokenizer::new("&#x41;");
        assert_eq!(tokenizer.next(), Some(Token::Character('A')));
    }

    #[test]
    fn test_numeric_hex_uppercase() {
        let mut tokenizer = Tokenizer::new("&#X41;");
        assert_eq!(tokenizer.next(), Some(Token::Character('A')));
    }

    #[test]
    fn test_entity_without_semicolon() {
        // &amp without ; should still decode (legacy compat)
        let tokens: Vec<_> = Tokenizer::new("&amp ").collect();
        let chars: alloc::string::String = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(chars, "& ");
    }

    #[test]
    fn test_unknown_entity_passthrough() {
        // &unknown; should pass through as literal text
        let tokens: Vec<_> = Tokenizer::new("&unknown;").collect();
        let chars: alloc::string::String = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(chars, "&unknown;");
    }

    #[test]
    fn test_ampersand_alone() {
        let mut tokenizer = Tokenizer::new("&");
        assert_eq!(tokenizer.next(), Some(Token::Character('&')));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_ampersand_followed_by_space() {
        let mut tokenizer = Tokenizer::new("& ");
        assert_eq!(tokenizer.next(), Some(Token::Character('&')));
        assert_eq!(tokenizer.next(), Some(Token::Character(' ')));
    }

    #[test]
    fn test_entity_in_double_quoted_attr() {
        let mut tokenizer = Tokenizer::new(r#"<a href="?a=1&amp;b=2">"#);
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "a".into(),
                attributes: vec![("href".into(), "?a=1&b=2".into()),],
                self_closing: false,
            })
        );
    }

    #[test]
    fn test_entity_in_single_quoted_attr() {
        let mut tokenizer = Tokenizer::new("<a href='?a=1&amp;b=2'>");
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "a".into(),
                attributes: vec![("href".into(), "?a=1&b=2".into()),],
                self_closing: false,
            })
        );
    }

    #[test]
    fn test_entity_in_unquoted_attr() {
        let mut tokenizer = Tokenizer::new("<input value=a&amp;b>");
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "input".into(),
                attributes: vec![("value".into(), "a&b".into()),],
                self_closing: false,
            })
        );
    }

    #[test]
    fn test_numeric_entity_in_attr() {
        let mut tokenizer = Tokenizer::new(r#"<span data-x="&#169;">"#);
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "span".into(),
                attributes: vec![("data-x".into(), "\u{00A9}".into()),],
                self_closing: false,
            })
        );
    }

    #[test]
    fn test_attr_entity_no_semi_before_equals() {
        // WHATWG: in attribute, &notit= should NOT decode as ¬it=
        // because next char after "not" is 'i' (alphanumeric)
        let mut tokenizer = Tokenizer::new(r#"<a href="?notit=1">"#);
        assert_eq!(
            tokenizer.next(),
            Some(Token::StartTag {
                name: "a".into(),
                attributes: vec![("href".into(), "?notit=1".into()),],
                self_closing: false,
            })
        );
    }

    #[test]
    fn test_numeric_zero_replacement() {
        // &#0; → U+FFFD replacement character (per WHATWG spec)
        let mut tokenizer = Tokenizer::new("&#0;");
        assert_eq!(tokenizer.next(), Some(Token::Character('\u{FFFD}')));
    }

    #[test]
    fn test_nbsp_entity() {
        let mut tokenizer = Tokenizer::new("&nbsp;");
        assert_eq!(tokenizer.next(), Some(Token::Character('\u{00A0}')));
    }

    #[test]
    fn test_multiple_entities() {
        let tokens: Vec<_> = Tokenizer::new("&lt;&amp;&gt;").collect();
        let chars: alloc::string::String = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Character(c) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(chars, "<&>");
    }
}
