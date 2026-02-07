//! HTML validation.
//!
//! This module provides validation for parsed HTML documents,
//! checking for common issues like missing required attributes.
//!
//! ## Reference
//!
//! - [HTML Validation](https://html.spec.whatwg.org/multipage/dom.html#content-models)

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::dom::{Document, Element, Node};

/// A validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// The type of error.
    pub kind: ValidationErrorKind,
    /// The tag name of the element with the error.
    pub element: String,
    /// A human-readable description of the error.
    pub message: String,
}

/// Types of validation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationErrorKind {
    /// A required attribute is missing.
    MissingRequiredAttribute,
    /// An attribute has an invalid value.
    InvalidAttributeValue,
    /// An element is deprecated.
    DeprecatedElement,
    /// An invalid parent-child relationship.
    InvalidNesting,
    /// Duplicate ID found.
    DuplicateId,
}

/// Validation result containing all errors.
pub type ValidationResult = Vec<ValidationError>;

/// HTML validator.
pub struct Validator {
    /// Collected errors.
    errors: Vec<ValidationError>,
    /// Seen IDs for duplicate detection.
    seen_ids: Vec<String>,
}

impl Validator {
    /// Create a new validator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            errors: Vec::new(),
            seen_ids: Vec::new(),
        }
    }

    /// Validate a document.
    #[must_use]
    pub fn validate(mut self, doc: &Document) -> Vec<ValidationError> {
        self.validate_element(&doc.root);
        self.errors
    }

    /// Validate a list of nodes (for fragments).
    #[must_use]
    pub fn validate_nodes(mut self, nodes: &[Node]) -> Vec<ValidationError> {
        for node in nodes {
            if let Node::Element(elem) = node {
                self.validate_element(elem);
            }
        }
        self.errors
    }

    fn validate_element(&mut self, elem: &Element) {
        // Check for deprecated elements
        self.check_deprecated(elem);

        // Check required attributes
        self.check_required_attributes(elem);

        // Check for duplicate IDs
        self.check_duplicate_id(elem);

        // Check attribute values
        self.check_attribute_values(elem);

        // Recursively validate children
        for child in &elem.children {
            if let Node::Element(child_elem) = child {
                self.validate_element(child_elem);
            }
        }
    }

    fn check_deprecated(&mut self, elem: &Element) {
        let deprecated = matches!(
            elem.tag_name.as_str(),
            "acronym"
                | "applet"
                | "basefont"
                | "bgsound"
                | "big"
                | "blink"
                | "center"
                | "font"
                | "frame"
                | "frameset"
                | "isindex"
                | "keygen"
                | "listing"
                | "marquee"
                | "menuitem"
                | "multicol"
                | "nextid"
                | "nobr"
                | "noembed"
                | "noframes"
                | "plaintext"
                | "rb"
                | "rtc"
                | "spacer"
                | "strike"
                | "tt"
                | "xmp"
        );

        if deprecated {
            self.errors.push(ValidationError {
                kind: ValidationErrorKind::DeprecatedElement,
                element: elem.tag_name.clone(),
                message: alloc::format!(
                    "The <{}> element is deprecated and should not be used",
                    elem.tag_name,
                ),
            });
        }
    }

    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    fn check_required_attributes(&mut self, elem: &Element) {
        match elem.tag_name.as_str() {
            "img" => {
                if !elem.has_attribute("src") {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::MissingRequiredAttribute,
                        element: elem.tag_name.clone(),
                        message: "The <img> element requires a 'src' attribute".into(),
                    });
                }
                if !elem.has_attribute("alt") {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::MissingRequiredAttribute,
                        element: elem.tag_name.clone(),
                        message:
                            "The <img> element should have an 'alt' attribute for accessibility"
                                .into(),
                    });
                }
            }
            "a" => {
                // href is not strictly required (can be a placeholder link)
                // but we could warn if it's missing
            }
            "input" => {
                // Check for label association if not a hidden input
                if elem.get_attribute("type") != Some("hidden") && !elem.has_attribute("id") {
                    // This is a soft warning - input should have id for label association
                }
            }
            "script" => {
                // script requires either src or inline content
            }
            "link" => {
                if elem.get_attribute("rel") == Some("stylesheet") && !elem.has_attribute("href") {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::MissingRequiredAttribute,
                        element: elem.tag_name.clone(),
                        message:
                            "The <link rel=\"stylesheet\"> element requires an 'href' attribute"
                                .into(),
                    });
                }
            }
            "form" => {
                if !elem.has_attribute("action") {
                    // action is technically optional in HTML5, defaults to current URL
                }
            }
            "iframe" => {
                if !elem.has_attribute("src") && !elem.has_attribute("srcdoc") {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::MissingRequiredAttribute,
                        element: elem.tag_name.clone(),
                        message: "The <iframe> element requires either 'src' or 'srcdoc' attribute"
                            .into(),
                    });
                }
            }
            "video" | "audio" => {
                // Should have src attribute or source children
                if !elem.has_attribute("src")
                    && !elem
                        .children
                        .iter()
                        .any(|c| matches!(c, Node::Element(e) if e.tag_name == "source"))
                {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::MissingRequiredAttribute,
                        element: elem.tag_name.clone(),
                        message: alloc::format!(
                            "The <{}> element requires either 'src' attribute or <source> children",
                            elem.tag_name
                        ),
                    });
                }
            }
            "meta" => {
                // meta should have either charset, name+content, http-equiv+content, or itemprop
                let has_charset = elem.has_attribute("charset");
                let has_name = elem.has_attribute("name");
                let has_http_equiv = elem.has_attribute("http-equiv");
                let has_content = elem.has_attribute("content");
                let has_itemprop = elem.has_attribute("itemprop");

                if !has_charset && !has_itemprop && (has_name || has_http_equiv) && !has_content {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::MissingRequiredAttribute,
                        element: elem.tag_name.clone(),
                        message: "The <meta> element with 'name' or 'http-equiv' requires a 'content' attribute".into(),
                    });
                }
            }
            "area" => {
                if !elem.has_attribute("alt") {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::MissingRequiredAttribute,
                        element: elem.tag_name.clone(),
                        message: "The <area> element requires an 'alt' attribute".into(),
                    });
                }
            }
            "optgroup" => {
                if !elem.has_attribute("label") {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::MissingRequiredAttribute,
                        element: elem.tag_name.clone(),
                        message: "The <optgroup> element requires a 'label' attribute".into(),
                    });
                }
            }
            "progress" => {
                // value and max are optional but recommended
            }
            "time" => {
                // datetime attribute is recommended if content is not machine-readable
            }
            _ => {}
        }
    }

    fn check_duplicate_id(&mut self, elem: &Element) {
        if let Some(id) = elem.id() {
            if self.seen_ids.contains(&id.to_string()) {
                self.errors.push(ValidationError {
                    kind: ValidationErrorKind::DuplicateId,
                    element: elem.tag_name.clone(),
                    message: alloc::format!("Duplicate id '{id}' found"),
                });
            } else {
                self.seen_ids.push(id.to_string());
            }
        }
    }

    fn check_attribute_values(&mut self, elem: &Element) {
        // Check for empty required values
        if let Some(id) = elem.get_attribute("id") {
            if id.is_empty() {
                self.errors.push(ValidationError {
                    kind: ValidationErrorKind::InvalidAttributeValue,
                    element: elem.tag_name.clone(),
                    message: "The 'id' attribute must not be empty".into(),
                });
            } else if id.contains(char::is_whitespace) {
                self.errors.push(ValidationError {
                    kind: ValidationErrorKind::InvalidAttributeValue,
                    element: elem.tag_name.clone(),
                    message: "The 'id' attribute must not contain whitespace".into(),
                });
            }
        }

        // Check input type values
        if elem.tag_name == "input" {
            if let Some(input_type) = elem.get_attribute("type") {
                let valid_types = [
                    "button",
                    "checkbox",
                    "color",
                    "date",
                    "datetime-local",
                    "email",
                    "file",
                    "hidden",
                    "image",
                    "month",
                    "number",
                    "password",
                    "radio",
                    "range",
                    "reset",
                    "search",
                    "submit",
                    "tel",
                    "text",
                    "time",
                    "url",
                    "week",
                ];
                if !valid_types.contains(&input_type) {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::InvalidAttributeValue,
                        element: elem.tag_name.clone(),
                        message: alloc::format!("Invalid input type '{input_type}'"),
                    });
                }
            }
        }

        // Check target values for anchors
        if elem.tag_name == "a" || elem.tag_name == "form" {
            if let Some(target) = elem.get_attribute("target") {
                let valid_targets = ["_self", "_blank", "_parent", "_top"];
                if !target.starts_with('_') || valid_targets.contains(&target) {
                    // Valid: either a frame name or a reserved keyword
                } else if target.starts_with('_') && !valid_targets.contains(&target) {
                    self.errors.push(ValidationError {
                        kind: ValidationErrorKind::InvalidAttributeValue,
                        element: elem.tag_name.clone(),
                        message: alloc::format!("Invalid target '{target}'"),
                    });
                }
            }
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse, parse_fragment};

    #[test]
    fn test_missing_img_alt() {
        let doc = parse("<img src=\"test.jpg\">");
        let errors = Validator::new().validate(&doc);
        assert!(errors
            .iter()
            .any(|e| e.kind == ValidationErrorKind::MissingRequiredAttribute
                && e.message.contains("alt")));
    }

    #[test]
    fn test_missing_img_src() {
        let doc = parse("<img>");
        let errors = Validator::new().validate(&doc);
        assert!(errors
            .iter()
            .any(|e| e.kind == ValidationErrorKind::MissingRequiredAttribute
                && e.message.contains("src")));
    }

    #[test]
    fn test_valid_img() {
        let nodes = parse_fragment("<img src=\"test.jpg\" alt=\"Test image\">");
        let errors = Validator::new().validate_nodes(&nodes);
        assert!(!errors.iter().any(|e| e.element == "img"));
    }

    #[test]
    fn test_deprecated_element() {
        let doc = parse("<center>Content</center>");
        let errors = Validator::new().validate(&doc);
        assert!(errors
            .iter()
            .any(|e| e.kind == ValidationErrorKind::DeprecatedElement && e.element == "center"));
    }

    #[test]
    fn test_duplicate_id() {
        let doc = parse(r#"<div id="same"></div><div id="same"></div>"#);
        let errors = Validator::new().validate(&doc);
        assert!(errors
            .iter()
            .any(|e| e.kind == ValidationErrorKind::DuplicateId));
    }

    #[test]
    fn test_empty_id() {
        let doc = parse(r#"<div id=""></div>"#);
        let errors = Validator::new().validate(&doc);
        assert!(errors
            .iter()
            .any(|e| e.kind == ValidationErrorKind::InvalidAttributeValue
                && e.message.contains("id")));
    }

    #[test]
    fn test_invalid_input_type() {
        let doc = parse(r#"<input type="invalid">"#);
        let errors = Validator::new().validate(&doc);
        assert!(errors
            .iter()
            .any(|e| e.kind == ValidationErrorKind::InvalidAttributeValue
                && e.message.contains("input type")));
    }
}
