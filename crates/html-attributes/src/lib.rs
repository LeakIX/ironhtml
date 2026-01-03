//! # html-attributes
//!
//! Type-safe HTML5 attributes following the
//! [WHATWG HTML Living Standard](https://html.spec.whatwg.org/).
//!
//! This crate provides traits for global and element-specific attributes,
//! with type-safe enums for constrained attribute values.
//!
//! ## Example
//!
//! ```rust
//! use html_attributes::{AttributeValue, InputType, Target, Loading, Method};
//!
//! // Use type-safe enums for attribute values
//! let input_type = InputType::Email;
//! assert_eq!(input_type.to_attr_value(), "email");
//!
//! let target = Target::Blank;
//! assert_eq!(target.to_attr_value(), "_blank");
//!
//! // Access attribute names as constants
//! use html_attributes::{global, anchor, img};
//! assert_eq!(global::CLASS, "class");
//! assert_eq!(anchor::HREF, "href");
//! assert_eq!(img::LOADING, "loading");
//! ```
//!
//! ## Specification References
//!
//! - [Global attributes](https://html.spec.whatwg.org/multipage/dom.html#global-attributes)
//! - [The `a` element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element)
//! - [The `img` element](https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element)
//! - [The `input` element](https://html.spec.whatwg.org/multipage/input.html#the-input-element)
//! - [The `form` element](https://html.spec.whatwg.org/multipage/forms.html#the-form-element)

#![no_std]

extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;

// =============================================================================
// Attribute Value Types
// =============================================================================

/// A trait for types that can be converted to an attribute value string.
pub trait AttributeValue {
    /// Convert to the attribute value string.
    fn to_attr_value(&self) -> Cow<'static, str>;
}

impl AttributeValue for &'static str {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(self)
    }
}

impl AttributeValue for String {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Owned(self.clone())
    }
}

impl AttributeValue for Cow<'static, str> {
    fn to_attr_value(&self) -> Cow<'static, str> {
        self.clone()
    }
}

impl AttributeValue for u32 {
    fn to_attr_value(&self) -> Cow<'static, str> {
        use alloc::string::ToString;
        Cow::Owned(self.to_string())
    }
}

impl AttributeValue for i32 {
    fn to_attr_value(&self) -> Cow<'static, str> {
        use alloc::string::ToString;
        Cow::Owned(self.to_string())
    }
}

impl AttributeValue for bool {
    fn to_attr_value(&self) -> Cow<'static, str> {
        if *self {
            Cow::Borrowed("true")
        } else {
            Cow::Borrowed("false")
        }
    }
}

// =============================================================================
// Global Attribute Enums
// =============================================================================

/// The `dir` attribute values for text direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    /// Left-to-right text.
    Ltr,
    /// Right-to-left text.
    Rtl,
    /// Auto-detect text direction.
    Auto,
}

impl AttributeValue for Dir {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Dir::Ltr => "ltr",
            Dir::Rtl => "rtl",
            Dir::Auto => "auto",
        })
    }
}

/// The `contenteditable` attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentEditable {
    /// Content is editable.
    True,
    /// Content is not editable.
    False,
    /// Inherit from parent.
    Inherit,
}

impl AttributeValue for ContentEditable {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            ContentEditable::True => "true",
            ContentEditable::False => "false",
            ContentEditable::Inherit => "inherit",
        })
    }
}

/// The `draggable` attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Draggable {
    /// Element is draggable.
    True,
    /// Element is not draggable.
    False,
}

impl AttributeValue for Draggable {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Draggable::True => "true",
            Draggable::False => "false",
        })
    }
}

/// The `hidden` attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hidden {
    /// Hidden until found.
    UntilFound,
    /// Completely hidden (boolean attribute).
    Hidden,
}

impl AttributeValue for Hidden {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Hidden::UntilFound => "until-found",
            Hidden::Hidden => "hidden",
        })
    }
}

/// The `spellcheck` attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spellcheck {
    /// Spellcheck enabled.
    True,
    /// Spellcheck disabled.
    False,
}

impl AttributeValue for Spellcheck {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Spellcheck::True => "true",
            Spellcheck::False => "false",
        })
    }
}

/// The `translate` attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Translate {
    /// Content should be translated.
    Yes,
    /// Content should not be translated.
    No,
}

impl AttributeValue for Translate {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Translate::Yes => "yes",
            Translate::No => "no",
        })
    }
}

// =============================================================================
// Element-Specific Attribute Enums
// =============================================================================

/// The `target` attribute values for links.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Open in same frame.
    Self_,
    /// Open in new window/tab.
    Blank,
    /// Open in parent frame.
    Parent,
    /// Open in full window.
    Top,
}

impl AttributeValue for Target {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Target::Self_ => "_self",
            Target::Blank => "_blank",
            Target::Parent => "_parent",
            Target::Top => "_top",
        })
    }
}

/// The `rel` attribute values for links.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rel {
    /// Alternate representation.
    Alternate,
    /// Author of the document.
    Author,
    /// Bookmark link.
    Bookmark,
    /// External resource.
    External,
    /// Help document.
    Help,
    /// License information.
    License,
    /// Next document in sequence.
    Next,
    /// No follow for search engines.
    Nofollow,
    /// No opener.
    Noopener,
    /// No referrer.
    Noreferrer,
    /// Previous document in sequence.
    Prev,
    /// Search tool.
    Search,
    /// Tag for the document.
    Tag,
}

impl AttributeValue for Rel {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Rel::Alternate => "alternate",
            Rel::Author => "author",
            Rel::Bookmark => "bookmark",
            Rel::External => "external",
            Rel::Help => "help",
            Rel::License => "license",
            Rel::Next => "next",
            Rel::Nofollow => "nofollow",
            Rel::Noopener => "noopener",
            Rel::Noreferrer => "noreferrer",
            Rel::Prev => "prev",
            Rel::Search => "search",
            Rel::Tag => "tag",
        })
    }
}

/// The `loading` attribute values for images and iframes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Loading {
    /// Load immediately.
    Eager,
    /// Lazy load when near viewport.
    Lazy,
}

impl AttributeValue for Loading {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Loading::Eager => "eager",
            Loading::Lazy => "lazy",
        })
    }
}

/// The `decoding` attribute values for images.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decoding {
    /// Synchronous decoding.
    Sync,
    /// Asynchronous decoding.
    Async,
    /// Browser decides.
    Auto,
}

impl AttributeValue for Decoding {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Decoding::Sync => "sync",
            Decoding::Async => "async",
            Decoding::Auto => "auto",
        })
    }
}

/// The `crossorigin` attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossOrigin {
    /// Anonymous CORS request.
    Anonymous,
    /// CORS request with credentials.
    UseCredentials,
}

impl AttributeValue for CrossOrigin {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            CrossOrigin::Anonymous => "anonymous",
            CrossOrigin::UseCredentials => "use-credentials",
        })
    }
}

/// The `referrerpolicy` attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferrerPolicy {
    /// No referrer.
    NoReferrer,
    /// No referrer when downgrade.
    NoReferrerWhenDowngrade,
    /// Origin only.
    Origin,
    /// Origin when cross-origin.
    OriginWhenCrossOrigin,
    /// Same origin only.
    SameOrigin,
    /// Strict origin.
    StrictOrigin,
    /// Strict origin when cross-origin.
    StrictOriginWhenCrossOrigin,
    /// Full URL.
    UnsafeUrl,
}

impl AttributeValue for ReferrerPolicy {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            ReferrerPolicy::NoReferrer => "no-referrer",
            ReferrerPolicy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            ReferrerPolicy::Origin => "origin",
            ReferrerPolicy::OriginWhenCrossOrigin => "origin-when-cross-origin",
            ReferrerPolicy::SameOrigin => "same-origin",
            ReferrerPolicy::StrictOrigin => "strict-origin",
            ReferrerPolicy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            ReferrerPolicy::UnsafeUrl => "unsafe-url",
        })
    }
}

/// The `type` attribute values for input elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    /// Text input.
    Text,
    /// Password input.
    Password,
    /// Email input.
    Email,
    /// URL input.
    Url,
    /// Telephone input.
    Tel,
    /// Number input.
    Number,
    /// Range input.
    Range,
    /// Date input.
    Date,
    /// Time input.
    Time,
    /// Datetime-local input.
    DatetimeLocal,
    /// Month input.
    Month,
    /// Week input.
    Week,
    /// Color input.
    Color,
    /// Checkbox input.
    Checkbox,
    /// Radio input.
    Radio,
    /// File input.
    File,
    /// Submit button.
    Submit,
    /// Reset button.
    Reset,
    /// Button input.
    Button,
    /// Image input.
    Image,
    /// Hidden input.
    Hidden,
    /// Search input.
    Search,
}

impl AttributeValue for InputType {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Url => "url",
            InputType::Tel => "tel",
            InputType::Number => "number",
            InputType::Range => "range",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DatetimeLocal => "datetime-local",
            InputType::Month => "month",
            InputType::Week => "week",
            InputType::Color => "color",
            InputType::Checkbox => "checkbox",
            InputType::Radio => "radio",
            InputType::File => "file",
            InputType::Submit => "submit",
            InputType::Reset => "reset",
            InputType::Button => "button",
            InputType::Image => "image",
            InputType::Hidden => "hidden",
            InputType::Search => "search",
        })
    }
}

/// The `type` attribute values for button elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    /// Submit button.
    Submit,
    /// Reset button.
    Reset,
    /// Regular button.
    Button,
}

impl AttributeValue for ButtonType {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Button => "button",
        })
    }
}

/// The `autocomplete` attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Autocomplete {
    /// Autocomplete on.
    On,
    /// Autocomplete off.
    Off,
    /// Name field.
    Name,
    /// Email field.
    Email,
    /// Username field.
    Username,
    /// New password.
    NewPassword,
    /// Current password.
    CurrentPassword,
    /// One-time code.
    OneTimeCode,
    /// Organization.
    Organization,
    /// Street address.
    StreetAddress,
    /// Country.
    Country,
    /// Postal code.
    PostalCode,
    /// Credit card number.
    CcNumber,
    /// Credit card expiration.
    CcExp,
    /// Credit card CSC.
    CcCsc,
    /// Telephone.
    Tel,
    /// URL.
    Url,
}

impl AttributeValue for Autocomplete {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Autocomplete::On => "on",
            Autocomplete::Off => "off",
            Autocomplete::Name => "name",
            Autocomplete::Email => "email",
            Autocomplete::Username => "username",
            Autocomplete::NewPassword => "new-password",
            Autocomplete::CurrentPassword => "current-password",
            Autocomplete::OneTimeCode => "one-time-code",
            Autocomplete::Organization => "organization",
            Autocomplete::StreetAddress => "street-address",
            Autocomplete::Country => "country",
            Autocomplete::PostalCode => "postal-code",
            Autocomplete::CcNumber => "cc-number",
            Autocomplete::CcExp => "cc-exp",
            Autocomplete::CcCsc => "cc-csc",
            Autocomplete::Tel => "tel",
            Autocomplete::Url => "url",
        })
    }
}

/// The `method` attribute values for forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// GET method.
    Get,
    /// POST method.
    Post,
    /// Dialog method.
    Dialog,
}

impl AttributeValue for Method {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Method::Get => "get",
            Method::Post => "post",
            Method::Dialog => "dialog",
        })
    }
}

/// The `enctype` attribute values for forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enctype {
    /// URL encoded.
    UrlEncoded,
    /// Multipart form data.
    Multipart,
    /// Plain text.
    Plain,
}

impl AttributeValue for Enctype {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Enctype::UrlEncoded => "application/x-www-form-urlencoded",
            Enctype::Multipart => "multipart/form-data",
            Enctype::Plain => "text/plain",
        })
    }
}

/// The `wrap` attribute values for textarea.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrap {
    /// Hard wrap.
    Hard,
    /// Soft wrap.
    Soft,
    /// No wrap.
    Off,
}

impl AttributeValue for Wrap {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Wrap::Hard => "hard",
            Wrap::Soft => "soft",
            Wrap::Off => "off",
        })
    }
}

/// The `scope` attribute values for table headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// Row scope.
    Row,
    /// Column scope.
    Col,
    /// Row group scope.
    Rowgroup,
    /// Column group scope.
    Colgroup,
}

impl AttributeValue for Scope {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Scope::Row => "row",
            Scope::Col => "col",
            Scope::Rowgroup => "rowgroup",
            Scope::Colgroup => "colgroup",
        })
    }
}

/// The `preload` attribute values for media elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preload {
    /// No preload.
    None,
    /// Preload metadata only.
    Metadata,
    /// Preload entire resource.
    Auto,
}

impl AttributeValue for Preload {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Preload::None => "none",
            Preload::Metadata => "metadata",
            Preload::Auto => "auto",
        })
    }
}

/// The `kind` attribute values for track elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackKind {
    /// Subtitles.
    Subtitles,
    /// Captions.
    Captions,
    /// Descriptions.
    Descriptions,
    /// Chapters.
    Chapters,
    /// Metadata.
    Metadata,
}

impl AttributeValue for TrackKind {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            TrackKind::Subtitles => "subtitles",
            TrackKind::Captions => "captions",
            TrackKind::Descriptions => "descriptions",
            TrackKind::Chapters => "chapters",
            TrackKind::Metadata => "metadata",
        })
    }
}

/// The `sandbox` attribute values for iframes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sandbox {
    /// Allow forms.
    AllowForms,
    /// Allow modals.
    AllowModals,
    /// Allow orientation lock.
    AllowOrientationLock,
    /// Allow pointer lock.
    AllowPointerLock,
    /// Allow popups.
    AllowPopups,
    /// Allow popups to escape sandbox.
    AllowPopupsToEscapeSandbox,
    /// Allow presentation.
    AllowPresentation,
    /// Allow same origin.
    AllowSameOrigin,
    /// Allow scripts.
    AllowScripts,
    /// Allow top navigation.
    AllowTopNavigation,
    /// Allow top navigation by user activation.
    AllowTopNavigationByUserActivation,
}

impl AttributeValue for Sandbox {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Sandbox::AllowForms => "allow-forms",
            Sandbox::AllowModals => "allow-modals",
            Sandbox::AllowOrientationLock => "allow-orientation-lock",
            Sandbox::AllowPointerLock => "allow-pointer-lock",
            Sandbox::AllowPopups => "allow-popups",
            Sandbox::AllowPopupsToEscapeSandbox => "allow-popups-to-escape-sandbox",
            Sandbox::AllowPresentation => "allow-presentation",
            Sandbox::AllowSameOrigin => "allow-same-origin",
            Sandbox::AllowScripts => "allow-scripts",
            Sandbox::AllowTopNavigation => "allow-top-navigation",
            Sandbox::AllowTopNavigationByUserActivation => {
                "allow-top-navigation-by-user-activation"
            }
        })
    }
}

// =============================================================================
// Global Attributes
// =============================================================================

/// Global attributes available on all HTML elements.
/// See: https://html.spec.whatwg.org/multipage/dom.html#global-attributes
pub mod global {
    /// Global attribute names as static strings.
    pub const CLASS: &str = "class";
    pub const ID: &str = "id";
    pub const STYLE: &str = "style";
    pub const TITLE: &str = "title";
    pub const LANG: &str = "lang";
    pub const DIR: &str = "dir";
    pub const TABINDEX: &str = "tabindex";
    pub const ACCESSKEY: &str = "accesskey";
    pub const CONTENTEDITABLE: &str = "contenteditable";
    pub const DRAGGABLE: &str = "draggable";
    pub const HIDDEN: &str = "hidden";
    pub const SPELLCHECK: &str = "spellcheck";
    pub const TRANSLATE: &str = "translate";
    pub const ROLE: &str = "role";
    pub const SLOT: &str = "slot";
}

// =============================================================================
// Element-Specific Attribute Names
// =============================================================================

/// Attribute names for anchor (`<a>`) elements.
pub mod anchor {
    pub const HREF: &str = "href";
    pub const TARGET: &str = "target";
    pub const REL: &str = "rel";
    pub const DOWNLOAD: &str = "download";
    pub const HREFLANG: &str = "hreflang";
    pub const TYPE: &str = "type";
    pub const REFERRERPOLICY: &str = "referrerpolicy";
}

/// Attribute names for image (`<img>`) elements.
pub mod img {
    pub const SRC: &str = "src";
    pub const ALT: &str = "alt";
    pub const WIDTH: &str = "width";
    pub const HEIGHT: &str = "height";
    pub const LOADING: &str = "loading";
    pub const DECODING: &str = "decoding";
    pub const CROSSORIGIN: &str = "crossorigin";
    pub const REFERRERPOLICY: &str = "referrerpolicy";
    pub const SRCSET: &str = "srcset";
    pub const SIZES: &str = "sizes";
    pub const USEMAP: &str = "usemap";
    pub const ISMAP: &str = "ismap";
}

/// Attribute names for input (`<input>`) elements.
pub mod input {
    pub const TYPE: &str = "type";
    pub const NAME: &str = "name";
    pub const VALUE: &str = "value";
    pub const PLACEHOLDER: &str = "placeholder";
    pub const REQUIRED: &str = "required";
    pub const DISABLED: &str = "disabled";
    pub const READONLY: &str = "readonly";
    pub const CHECKED: &str = "checked";
    pub const AUTOCOMPLETE: &str = "autocomplete";
    pub const AUTOFOCUS: &str = "autofocus";
    pub const MIN: &str = "min";
    pub const MAX: &str = "max";
    pub const STEP: &str = "step";
    pub const MINLENGTH: &str = "minlength";
    pub const MAXLENGTH: &str = "maxlength";
    pub const PATTERN: &str = "pattern";
    pub const SIZE: &str = "size";
    pub const ACCEPT: &str = "accept";
    pub const MULTIPLE: &str = "multiple";
    pub const LIST: &str = "list";
    pub const FORM: &str = "form";
}

/// Attribute names for button (`<button>`) elements.
pub mod button {
    pub const TYPE: &str = "type";
    pub const NAME: &str = "name";
    pub const VALUE: &str = "value";
    pub const DISABLED: &str = "disabled";
    pub const AUTOFOCUS: &str = "autofocus";
    pub const FORM: &str = "form";
    pub const FORMACTION: &str = "formaction";
    pub const FORMMETHOD: &str = "formmethod";
    pub const FORMENCTYPE: &str = "formenctype";
    pub const FORMNOVALIDATE: &str = "formnovalidate";
    pub const FORMTARGET: &str = "formtarget";
}

/// Attribute names for form (`<form>`) elements.
pub mod form {
    pub const ACTION: &str = "action";
    pub const METHOD: &str = "method";
    pub const ENCTYPE: &str = "enctype";
    pub const TARGET: &str = "target";
    pub const NOVALIDATE: &str = "novalidate";
    pub const AUTOCOMPLETE: &str = "autocomplete";
    pub const NAME: &str = "name";
    pub const ACCEPTCHARSET: &str = "accept-charset";
}

/// Attribute names for textarea (`<textarea>`) elements.
pub mod textarea {
    pub const NAME: &str = "name";
    pub const PLACEHOLDER: &str = "placeholder";
    pub const REQUIRED: &str = "required";
    pub const DISABLED: &str = "disabled";
    pub const READONLY: &str = "readonly";
    pub const AUTOCOMPLETE: &str = "autocomplete";
    pub const AUTOFOCUS: &str = "autofocus";
    pub const ROWS: &str = "rows";
    pub const COLS: &str = "cols";
    pub const MINLENGTH: &str = "minlength";
    pub const MAXLENGTH: &str = "maxlength";
    pub const WRAP: &str = "wrap";
    pub const FORM: &str = "form";
}

/// Attribute names for select (`<select>`) elements.
pub mod select {
    pub const NAME: &str = "name";
    pub const REQUIRED: &str = "required";
    pub const DISABLED: &str = "disabled";
    pub const AUTOFOCUS: &str = "autofocus";
    pub const MULTIPLE: &str = "multiple";
    pub const SIZE: &str = "size";
    pub const FORM: &str = "form";
    pub const AUTOCOMPLETE: &str = "autocomplete";
}

/// Attribute names for option (`<option>`) elements.
pub mod option {
    pub const VALUE: &str = "value";
    pub const SELECTED: &str = "selected";
    pub const DISABLED: &str = "disabled";
    pub const LABEL: &str = "label";
}

/// Attribute names for label (`<label>`) elements.
pub mod label {
    pub const FOR: &str = "for";
}

/// Attribute names for table (`<table>`) elements.
pub mod table {
    pub const BORDER: &str = "border";
}

/// Attribute names for table cell (`<td>`, `<th>`) elements.
pub mod tablecell {
    pub const COLSPAN: &str = "colspan";
    pub const ROWSPAN: &str = "rowspan";
    pub const HEADERS: &str = "headers";
    pub const SCOPE: &str = "scope";
}

/// Attribute names for media (`<audio>`, `<video>`) elements.
pub mod media {
    pub const SRC: &str = "src";
    pub const CONTROLS: &str = "controls";
    pub const AUTOPLAY: &str = "autoplay";
    pub const LOOP: &str = "loop";
    pub const MUTED: &str = "muted";
    pub const PRELOAD: &str = "preload";
    pub const POSTER: &str = "poster";
    pub const WIDTH: &str = "width";
    pub const HEIGHT: &str = "height";
    pub const CROSSORIGIN: &str = "crossorigin";
}

/// Attribute names for source (`<source>`) elements.
pub mod source {
    pub const SRC: &str = "src";
    pub const TYPE: &str = "type";
    pub const SRCSET: &str = "srcset";
    pub const SIZES: &str = "sizes";
    pub const MEDIA: &str = "media";
}

/// Attribute names for track (`<track>`) elements.
pub mod track {
    pub const SRC: &str = "src";
    pub const KIND: &str = "kind";
    pub const SRCLANG: &str = "srclang";
    pub const LABEL: &str = "label";
    pub const DEFAULT: &str = "default";
}

/// Attribute names for iframe (`<iframe>`) elements.
pub mod iframe {
    pub const SRC: &str = "src";
    pub const SRCDOC: &str = "srcdoc";
    pub const NAME: &str = "name";
    pub const WIDTH: &str = "width";
    pub const HEIGHT: &str = "height";
    pub const LOADING: &str = "loading";
    pub const SANDBOX: &str = "sandbox";
    pub const ALLOW: &str = "allow";
    pub const REFERRERPOLICY: &str = "referrerpolicy";
}

/// Attribute names for meta (`<meta>`) elements.
pub mod meta {
    pub const CHARSET: &str = "charset";
    pub const NAME: &str = "name";
    pub const CONTENT: &str = "content";
    pub const HTTPEQUIV: &str = "http-equiv";
}

/// Attribute names for link (`<link>`) elements.
pub mod link {
    pub const HREF: &str = "href";
    pub const REL: &str = "rel";
    pub const TYPE: &str = "type";
    pub const MEDIA: &str = "media";
    pub const CROSSORIGIN: &str = "crossorigin";
    pub const INTEGRITY: &str = "integrity";
    pub const REFERRERPOLICY: &str = "referrerpolicy";
    pub const SIZES: &str = "sizes";
    pub const AS: &str = "as";
}

/// Attribute names for script (`<script>`) elements.
pub mod script {
    pub const SRC: &str = "src";
    pub const TYPE: &str = "type";
    pub const ASYNC: &str = "async";
    pub const DEFER: &str = "defer";
    pub const CROSSORIGIN: &str = "crossorigin";
    pub const INTEGRITY: &str = "integrity";
    pub const NOMODULE: &str = "nomodule";
    pub const REFERRERPOLICY: &str = "referrerpolicy";
}

/// Attribute names for style (`<style>`) elements.
pub mod style {
    pub const MEDIA: &str = "media";
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_values() {
        assert_eq!(Dir::Ltr.to_attr_value(), "ltr");
        assert_eq!(Dir::Rtl.to_attr_value(), "rtl");
        assert_eq!(Dir::Auto.to_attr_value(), "auto");
    }

    #[test]
    fn test_target_values() {
        assert_eq!(Target::Self_.to_attr_value(), "_self");
        assert_eq!(Target::Blank.to_attr_value(), "_blank");
        assert_eq!(Target::Parent.to_attr_value(), "_parent");
        assert_eq!(Target::Top.to_attr_value(), "_top");
    }

    #[test]
    fn test_input_type_values() {
        assert_eq!(InputType::Text.to_attr_value(), "text");
        assert_eq!(InputType::Password.to_attr_value(), "password");
        assert_eq!(InputType::Email.to_attr_value(), "email");
        assert_eq!(InputType::Checkbox.to_attr_value(), "checkbox");
        assert_eq!(InputType::Submit.to_attr_value(), "submit");
    }

    #[test]
    fn test_loading_values() {
        assert_eq!(Loading::Eager.to_attr_value(), "eager");
        assert_eq!(Loading::Lazy.to_attr_value(), "lazy");
    }

    #[test]
    fn test_method_values() {
        assert_eq!(Method::Get.to_attr_value(), "get");
        assert_eq!(Method::Post.to_attr_value(), "post");
        assert_eq!(Method::Dialog.to_attr_value(), "dialog");
    }

    #[test]
    fn test_numeric_attribute_values() {
        assert_eq!(42u32.to_attr_value(), "42");
        assert_eq!((-10i32).to_attr_value(), "-10");
    }

    #[test]
    fn test_global_attribute_names() {
        assert_eq!(global::CLASS, "class");
        assert_eq!(global::ID, "id");
        assert_eq!(global::STYLE, "style");
    }

    #[test]
    fn test_element_attribute_names() {
        assert_eq!(anchor::HREF, "href");
        assert_eq!(img::SRC, "src");
        assert_eq!(img::ALT, "alt");
        assert_eq!(input::TYPE, "type");
        assert_eq!(form::METHOD, "method");
    }
}
