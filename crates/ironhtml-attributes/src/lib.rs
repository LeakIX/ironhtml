//! # ironhtml-attributes
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
//! use ironhtml_attributes::{AttributeValue, InputType, Target, Loading, Method};
//!
//! // Use type-safe enums for attribute values
//! let input_type = InputType::Email;
//! assert_eq!(input_type.to_attr_value(), "email");
//!
//! let target = Target::Blank;
//! assert_eq!(target.to_attr_value(), "_blank");
//!
//! // Access attribute names as constants
//! use ironhtml_attributes::{global, anchor, img};
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

/// The `dir` attribute values for text directionality.
///
/// # Purpose
/// Controls the text directionality of an element's content, which is critical
/// for proper rendering of multilingual content and right-to-left languages.
///
/// # Usage Context
/// - Used with: All HTML elements (global attribute)
/// - Affects: Text rendering direction, visual layout, and text alignment
///
/// # Valid Values
/// - `Ltr`: Left-to-right text direction (default for most languages)
/// - `Rtl`: Right-to-left text direction (for languages like Arabic, Hebrew)
/// - `Auto`: Browser determines direction based on content
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Dir};
/// let direction = Dir::Rtl;
/// assert_eq!(direction.to_attr_value(), "rtl");
/// ```
///
/// ```html
/// <p dir="rtl">مرحبا بك</p>
/// <p dir="ltr">Hello world</p>
/// <p dir="auto">Auto-detected text</p>
/// ```
///
/// # WHATWG Specification
/// - [The `dir` attribute](https://html.spec.whatwg.org/multipage/dom.html#the-dir-attribute)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    /// Left-to-right text direction, used for most Western languages
    /// (English, Spanish, French, etc.).
    Ltr,
    /// Right-to-left text direction, used for languages like Arabic,
    /// Hebrew, Persian, and Urdu.
    Rtl,
    /// Automatically determines text direction based on the first
    /// strongly-typed directional character in the content.
    Auto,
}

impl AttributeValue for Dir {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
            Self::Auto => "auto",
        })
    }
}

/// The `contenteditable` attribute values.
///
/// # Purpose
/// Controls whether an element's content can be edited by the user, enabling
/// rich text editing and in-place content modification in web applications.
///
/// # Usage Context
/// - Used with: All HTML elements (global attribute)
/// - Common use: Rich text editors, inline editing, CMS interfaces
///
/// # Valid Values
/// - `True`: Element content is editable by the user
/// - `False`: Element content is not editable
/// - `Inherit`: Inherits editability from parent element
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, ContentEditable};
/// let editable = ContentEditable::True;
/// assert_eq!(editable.to_attr_value(), "true");
/// ```
///
/// ```html
/// <div contenteditable="true">Edit this text directly!</div>
/// <div contenteditable="false">This cannot be edited</div>
/// <span contenteditable="inherit">Inherits from parent</span>
/// ```
///
/// # WHATWG Specification
/// - [The `contenteditable` attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-contenteditable)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentEditable {
    /// Content is editable by the user. The element becomes a rich text
    /// editing host.
    True,
    /// Content is explicitly not editable. This overrides any inherited
    /// editability.
    False,
    /// Inherits the `contenteditable` state from the parent element.
    Inherit,
}

impl AttributeValue for ContentEditable {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::True => "true",
            Self::False => "false",
            Self::Inherit => "inherit",
        })
    }
}

/// The `draggable` attribute values.
///
/// # Purpose
/// Indicates whether an element can be dragged using the HTML5 drag-and-drop
/// API, enabling rich drag-and-drop interactions in web applications.
///
/// # Usage Context
/// - Used with: All HTML elements (global attribute)
/// - Default: Links and images are draggable by default; other elements are not
/// - Common use: File upload interfaces, sortable lists, drag-and-drop games
///
/// # Valid Values
/// - `True`: Element is draggable
/// - `False`: Element is not draggable (overrides default behavior)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Draggable};
/// let draggable = Draggable::True;
/// assert_eq!(draggable.to_attr_value(), "true");
/// ```
///
/// ```html
/// <div draggable="true">Drag me!</div>
/// <img src="photo.jpg" draggable="false" alt="Cannot drag this image">
/// ```
///
/// # WHATWG Specification
/// - [The `draggable` attribute](https://html.spec.whatwg.org/multipage/dnd.html#the-draggable-attribute)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Draggable {
    /// Element is draggable and can be used with the drag-and-drop API.
    True,
    /// Element is not draggable, overriding any default draggable behavior.
    False,
}

impl AttributeValue for Draggable {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::True => "true",
            Self::False => "false",
        })
    }
}

/// The `hidden` attribute values.
///
/// # Purpose
/// Controls element visibility and search-in-page behavior, allowing elements
/// to be hidden from rendering while remaining in the DOM.
///
/// # Usage Context
/// - Used with: All HTML elements (global attribute)
/// - Effect: Hides element from rendering and accessibility tree
/// - Note: Can be overridden by CSS `display` property
///
/// # Valid Values
/// - `UntilFound`: Element is hidden but searchable and will be revealed when found
/// - `Hidden`: Element is completely hidden (boolean attribute style)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Hidden};
/// let hidden = Hidden::UntilFound;
/// assert_eq!(hidden.to_attr_value(), "until-found");
/// ```
///
/// ```html
/// <div hidden="until-found">Search will reveal this content</div>
/// <div hidden>Completely hidden content</div>
/// ```
///
/// # WHATWG Specification
/// - [The `hidden` attribute](https://html.spec.whatwg.org/multipage/interaction.html#the-hidden-attribute)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hidden {
    /// Element is hidden but can be revealed by browser find-in-page or
    /// fragment navigation. The element remains hidden until found.
    UntilFound,
    /// Element is completely hidden from rendering and the accessibility tree.
    /// Used as a boolean attribute.
    Hidden,
}

impl AttributeValue for Hidden {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::UntilFound => "until-found",
            Self::Hidden => "hidden",
        })
    }
}

/// The `spellcheck` attribute values.
///
/// # Purpose
/// Controls whether the browser's spell-checking feature is enabled for
/// editable elements, helping users catch typing errors.
///
/// # Usage Context
/// - Used with: All HTML elements (global attribute)
/// - Primarily useful for: Editable elements (`contenteditable`, `<input>`, `<textarea>`)
/// - Default: Browser-dependent, typically enabled for editable text
///
/// # Valid Values
/// - `True`: Enable spell-checking for this element
/// - `False`: Disable spell-checking for this element
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Spellcheck};
/// let check = Spellcheck::False;
/// assert_eq!(check.to_attr_value(), "false");
/// ```
///
/// ```html
/// <textarea spellcheck="true">Enable spell-checking here</textarea>
/// <input type="text" spellcheck="false" placeholder="Code (no spellcheck)">
/// ```
///
/// # WHATWG Specification
/// - [The `spellcheck` attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-spellcheck)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spellcheck {
    /// Enable spell-checking for this element. The browser will check
    /// spelling and mark errors.
    True,
    /// Disable spell-checking for this element. Useful for code editors,
    /// usernames, and technical content.
    False,
}

impl AttributeValue for Spellcheck {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::True => "true",
            Self::False => "false",
        })
    }
}

/// The `translate` attribute values.
///
/// # Purpose
/// Indicates whether an element's text content and attribute values should be
/// translated when the page is localized, useful for multilingual applications.
///
/// # Usage Context
/// - Used with: All HTML elements (global attribute)
/// - Common use: Marking technical terms, brand names, or code that shouldn't be translated
/// - Default: `yes` (content is translatable)
///
/// # Valid Values
/// - `Yes`: Content should be translated when localizing the page
/// - `No`: Content should not be translated (e.g., brand names, code)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Translate};
/// let trans = Translate::No;
/// assert_eq!(trans.to_attr_value(), "no");
/// ```
///
/// ```html
/// <p>Welcome to <span translate="no">GitHub</span>!</p>
/// <code translate="no">const x = 42;</code>
/// <p translate="yes">This text can be translated</p>
/// ```
///
/// # WHATWG Specification
/// - [The `translate` attribute](https://html.spec.whatwg.org/multipage/dom.html#attr-translate)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Translate {
    /// Element's text and attributes should be translated when the page
    /// is localized.
    Yes,
    /// Element's text and attributes should not be translated. Use for
    /// brand names, code samples, or technical terms.
    No,
}

impl AttributeValue for Translate {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Yes => "yes",
            Self::No => "no",
        })
    }
}

// =============================================================================
// Element-Specific Attribute Enums
// =============================================================================

/// The `target` attribute values for hyperlinks and forms.
///
/// # Purpose
/// Specifies where to display the linked URL or form response, controlling
/// whether content opens in a new window, the same frame, or a specific context.
///
/// # Usage Context
/// - Used with: `<a>`, `<area>`, `<form>`, `<base>` elements
/// - Security: Use with `rel="noopener"` when opening in new window
///
/// # Valid Values
/// - `Self_`: Open in the same browsing context (default)
/// - `Blank`: Open in a new window or tab
/// - `Parent`: Open in the parent browsing context
/// - `Top`: Open in the top-most browsing context
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Target};
/// let target = Target::Blank;
/// assert_eq!(target.to_attr_value(), "_blank");
/// ```
///
/// ```html
/// <a href="https://example.com" target="_blank">Open in new tab</a>
/// <a href="/page" target="_self">Open in same tab</a>
/// <form action="/submit" target="_parent" method="post">...</form>
/// ```
///
/// # WHATWG Specification
/// - [Browsing context names](https://html.spec.whatwg.org/multipage/browsers.html#valid-browsing-context-name-or-keyword)
/// - [The `target` attribute](https://html.spec.whatwg.org/multipage/links.html#attr-hyperlink-target)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Open in the same browsing context (frame/tab). This is the default
    /// behavior when target is not specified.
    Self_,
    /// Open in a new, unnamed browsing context (typically a new tab or window).
    /// For security, use with `rel="noopener noreferrer"`.
    Blank,
    /// Open in the parent browsing context. If no parent exists, behaves
    /// like `_self`.
    Parent,
    /// Open in the top-level browsing context (the highest-level ancestor).
    /// If no ancestors exist, behaves like `_self`.
    Top,
}

impl AttributeValue for Target {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Self_ => "_self",
            Self::Blank => "_blank",
            Self::Parent => "_parent",
            Self::Top => "_top",
        })
    }
}

/// The `rel` attribute values for link relationships.
///
/// # Purpose
/// Defines the relationship between the current document and the linked resource,
/// providing semantic meaning for links and enabling special browser behaviors.
///
/// # Usage Context
/// - Used with: `<a>`, `<area>`, `<link>`, `<form>` elements
/// - Multiple values: Can be space-separated (e.g., "noopener noreferrer")
/// - SEO impact: Values like `nofollow` affect search engine crawling
///
/// # Valid Values
/// - `Alternate`: Alternate representation of the document
/// - `Author`: Link to the document's author
/// - `Bookmark`: Permalink for the nearest ancestor section
/// - `External`: Link to a different website
/// - `Help`: Link to context-sensitive help
/// - `License`: Link to copyright/license information
/// - `Next`: Next document in a sequence
/// - `Nofollow`: Do not follow this link for SEO purposes
/// - `Noopener`: Prevents window.opener access (security)
/// - `Noreferrer`: Don't send referer header (privacy)
/// - `Prev`: Previous document in a sequence
/// - `Search`: Link to a search tool
/// - `Tag`: Tag/keyword for the current document
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Rel};
/// let rel = Rel::Noopener;
/// assert_eq!(rel.to_attr_value(), "noopener");
/// ```
///
/// ```html
/// <a href="https://external.com" rel="external nofollow">External Site</a>
/// <a href="/help" rel="help">Help Documentation</a>
/// <link rel="alternate" href="/feed.xml" type="application/rss+xml">
/// <a href="https://example.com" target="_blank" rel="noopener noreferrer">Safe Link</a>
/// ```
///
/// # WHATWG Specification
/// - [Link types](https://html.spec.whatwg.org/multipage/links.html#linkTypes)
/// - [The `rel` attribute](https://html.spec.whatwg.org/multipage/links.html#attr-hyperlink-rel)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rel {
    /// Indicates an alternate representation of the current document,
    /// such as translations, RSS feeds, or print versions.
    Alternate,
    /// Link to information about the author of the document.
    Author,
    /// Provides a permalink to the nearest ancestor section.
    Bookmark,
    /// Indicates the link references a resource on a different site.
    External,
    /// Link to context-sensitive help information.
    Help,
    /// Link to copyright, license, or legal information for the document.
    License,
    /// Indicates the next document in a sequence (pagination, slideshows).
    Next,
    /// Instructs search engines not to follow this link for ranking purposes.
    Nofollow,
    /// Prevents the new browsing context from accessing `window.opener`.
    /// Critical security feature for `target="_blank"` links.
    Noopener,
    /// Prevents the browser from sending the Referer header. Enhances privacy.
    Noreferrer,
    /// Indicates the previous document in a sequence (pagination, slideshows).
    Prev,
    /// Link to a search tool or interface for the current document.
    Search,
    /// Indicates the link represents a tag or keyword for the current document.
    Tag,
}

impl AttributeValue for Rel {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Alternate => "alternate",
            Self::Author => "author",
            Self::Bookmark => "bookmark",
            Self::External => "external",
            Self::Help => "help",
            Self::License => "license",
            Self::Next => "next",
            Self::Nofollow => "nofollow",
            Self::Noopener => "noopener",
            Self::Noreferrer => "noreferrer",
            Self::Prev => "prev",
            Self::Search => "search",
            Self::Tag => "tag",
        })
    }
}

/// The `loading` attribute values for lazy-loading resources.
///
/// # Purpose
/// Controls when the browser should load images and iframes, enabling
/// performance optimization through lazy loading of off-screen content.
///
/// # Usage Context
/// - Used with: `<img>`, `<iframe>` elements
/// - Performance: Lazy loading can significantly improve initial page load
/// - Default: Browser-dependent (typically `eager`)
///
/// # Valid Values
/// - `Eager`: Load the resource immediately, regardless of viewport position
/// - `Lazy`: Defer loading until the resource is near the viewport
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Loading};
/// let loading = Loading::Lazy;
/// assert_eq!(loading.to_attr_value(), "lazy");
/// ```
///
/// ```html
/// <img src="above-fold.jpg" loading="eager" alt="Loads immediately">
/// <img src="below-fold.jpg" loading="lazy" alt="Loads when near viewport">
/// <iframe src="widget.html" loading="lazy"></iframe>
/// ```
///
/// # WHATWG Specification
/// - [The `loading` attribute](https://html.spec.whatwg.org/multipage/urls-and-fetching.html#lazy-loading-attributes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Loading {
    /// Load the resource immediately, without deferring. Use for
    /// above-the-fold or critical content.
    Eager,
    /// Defer loading the resource until it is calculated to be near the
    /// viewport. Improves performance for below-the-fold content.
    Lazy,
}

impl AttributeValue for Loading {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Eager => "eager",
            Self::Lazy => "lazy",
        })
    }
}

/// The `decoding` attribute values for image decoding.
///
/// # Purpose
/// Provides a hint to the browser about how to decode the image, allowing
/// optimization of the decoding strategy for better user experience.
///
/// # Usage Context
/// - Used with: `<img>` elements
/// - Performance: Affects when/how images are decoded relative to page rendering
/// - Default: Browser-dependent
///
/// # Valid Values
/// - `Sync`: Decode the image synchronously for atomic presentation with other content
/// - `Async`: Decode the image asynchronously to avoid delaying other content
/// - `Auto`: Let the browser decide the optimal decoding mode
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Decoding};
/// let decoding = Decoding::Async;
/// assert_eq!(decoding.to_attr_value(), "async");
/// ```
///
/// ```html
/// <img src="hero.jpg" decoding="sync" alt="Decode with page content">
/// <img src="gallery.jpg" decoding="async" alt="Decode asynchronously">
/// <img src="photo.jpg" decoding="auto" alt="Browser decides">
/// ```
///
/// # WHATWG Specification
/// - [The `decoding` attribute](https://html.spec.whatwg.org/multipage/embedded-content.html#dom-img-decoding)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decoding {
    /// Decode the image synchronously along with other page content for
    /// atomic presentation. May delay rendering.
    Sync,
    /// Decode the image asynchronously to reduce delay in presenting other
    /// content. Image may render after initial page load.
    Async,
    /// Allow the browser to choose the decoding strategy. This is the
    /// recommended default.
    Auto,
}

impl AttributeValue for Decoding {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Sync => "sync",
            Self::Async => "async",
            Self::Auto => "auto",
        })
    }
}

/// The `crossorigin` attribute values for CORS requests.
///
/// # Purpose
/// Controls how cross-origin requests are made for resources, determining
/// whether credentials are sent and enabling CORS validation.
///
/// # Usage Context
/// - Used with: `<img>`, `<script>`, `<link>`, `<audio>`, `<video>` elements
/// - Security: Required for accessing cross-origin resources with canvas/WebGL
/// - CORS: Server must send appropriate Access-Control headers
///
/// # Valid Values
/// - `Anonymous`: CORS request without credentials
/// - `UseCredentials`: CORS request with credentials (cookies, certificates)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, CrossOrigin};
/// let cors = CrossOrigin::Anonymous;
/// assert_eq!(cors.to_attr_value(), "anonymous");
/// ```
///
/// ```html
/// <img src="https://cdn.example.com/image.jpg" crossorigin="anonymous">
/// <script src="https://cdn.example.com/lib.js" crossorigin="use-credentials"></script>
/// <link rel="stylesheet" href="https://cdn.example.com/style.css" crossorigin="anonymous">
/// ```
///
/// # WHATWG Specification
/// - [CORS settings attributes](https://html.spec.whatwg.org/multipage/urls-and-fetching.html#cors-settings-attributes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossOrigin {
    /// Request uses CORS without credentials. Cookies and client certificates
    /// are not sent. This is the most common value.
    Anonymous,
    /// Request uses CORS with credentials. Cookies, client certificates, and
    /// authorization headers are included in the request.
    UseCredentials,
}

impl AttributeValue for CrossOrigin {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Anonymous => "anonymous",
            Self::UseCredentials => "use-credentials",
        })
    }
}

/// The `referrerpolicy` attribute values.
///
/// # Purpose
/// Controls how much referrer information is sent with requests, enabling
/// privacy control over what information is shared with linked resources.
///
/// # Usage Context
/// - Used with: `<a>`, `<area>`, `<img>`, `<iframe>`, `<link>`, `<script>` elements
/// - Privacy: Determines what URL information is sent in the Referer header
/// - Security: Prevents leaking sensitive URLs to third parties
///
/// # Valid Values
/// - `NoReferrer`: Never send referrer information
/// - `NoReferrerWhenDowngrade`: Send referrer only on same security level
/// - `Origin`: Send only the origin (scheme, host, port)
/// - `OriginWhenCrossOrigin`: Send full URL for same-origin, origin for cross-origin
/// - `SameOrigin`: Send referrer only for same-origin requests
/// - `StrictOrigin`: Send origin, but not when downgrading HTTPS→HTTP
/// - `StrictOriginWhenCrossOrigin`: Full URL same-origin, origin cross-origin, none on downgrade
/// - `UnsafeUrl`: Always send the full URL (not recommended)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, ReferrerPolicy};
/// let policy = ReferrerPolicy::NoReferrer;
/// assert_eq!(policy.to_attr_value(), "no-referrer");
/// ```
///
/// ```html
/// <a href="https://example.com" referrerpolicy="no-referrer">Private Link</a>
/// <img src="https://example.com/img.jpg" referrerpolicy="origin">
/// <iframe src="https://example.com" referrerpolicy="strict-origin-when-cross-origin"></iframe>
/// ```
///
/// # WHATWG Specification
/// - [Referrer policy](https://w3c.github.io/webappsec-referrer-policy/)
/// - [The `referrerpolicy` attribute](https://html.spec.whatwg.org/multipage/urls-and-fetching.html#referrer-policy-attributes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferrerPolicy {
    /// Never send referrer information. Maximum privacy.
    NoReferrer,
    /// Send referrer only when not downgrading from HTTPS to HTTP.
    /// This is often the browser default.
    NoReferrerWhenDowngrade,
    /// Send only the origin (no path or query string).
    Origin,
    /// Send full URL for same-origin requests, only origin for cross-origin.
    OriginWhenCrossOrigin,
    /// Send referrer only for same-origin requests, nothing for cross-origin.
    SameOrigin,
    /// Send only origin, and not when downgrading from HTTPS to HTTP.
    StrictOrigin,
    /// Send full URL for same-origin, origin for cross-origin, nothing when
    /// downgrading. Recommended for most use cases.
    StrictOriginWhenCrossOrigin,
    /// Always send the full URL as referrer. Not recommended due to privacy
    /// and security concerns.
    UnsafeUrl,
}

impl AttributeValue for ReferrerPolicy {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::NoReferrer => "no-referrer",
            Self::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            Self::Origin => "origin",
            Self::OriginWhenCrossOrigin => "origin-when-cross-origin",
            Self::SameOrigin => "same-origin",
            Self::StrictOrigin => "strict-origin",
            Self::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            Self::UnsafeUrl => "unsafe-url",
        })
    }
}

/// The `type` attribute values for `<input>` elements.
///
/// # Purpose
/// Defines the type of input control, determining the behavior, validation,
/// and user interface for collecting user input in forms.
///
/// # Usage Context
/// - Used with: `<input>` elements only
/// - Default: `text` if not specified
/// - Validation: Many types provide built-in validation (email, url, number)
/// - Mobile UX: Types affect virtual keyboard layout on mobile devices
///
/// # Valid Values
/// - `Text`: Single-line text input
/// - `Password`: Password input (characters obscured)
/// - `Email`: Email address with validation
/// - `Url`: URL with validation
/// - `Tel`: Telephone number
/// - `Number`: Numeric input with spinner controls
/// - `Range`: Slider control for numeric range
/// - `Date`: Date picker
/// - `Time`: Time picker
/// - `DatetimeLocal`: Date and time picker (no timezone)
/// - `Month`: Month and year picker
/// - `Week`: Week and year picker
/// - `Color`: Color picker
/// - `Checkbox`: Checkbox for boolean values
/// - `Radio`: Radio button for single selection from group
/// - `File`: File upload control
/// - `Submit`: Submit button for forms
/// - `Reset`: Reset button to clear form
/// - `Button`: Generic button (no default behavior)
/// - `Image`: Graphical submit button
/// - `Hidden`: Hidden input (not displayed)
/// - `Search`: Search input with platform-specific styling
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, InputType};
/// let input_type = InputType::Email;
/// assert_eq!(input_type.to_attr_value(), "email");
/// ```
///
/// ```html
/// <input type="text" placeholder="Name">
/// <input type="email" placeholder="user@example.com">
/// <input type="password" placeholder="Password">
/// <input type="number" min="0" max="100">
/// <input type="date">
/// <input type="checkbox" id="agree">
/// <input type="submit" value="Submit">
/// ```
///
/// # WHATWG Specification
/// - [The `input` element](https://html.spec.whatwg.org/multipage/input.html#the-input-element)
/// - [Input types](https://html.spec.whatwg.org/multipage/input.html#attr-input-type)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    /// Single-line text input. Default type if not specified.
    Text,
    /// Password input where characters are obscured for security.
    Password,
    /// Email address input with built-in validation.
    Email,
    /// URL input with validation for proper URL format.
    Url,
    /// Telephone number input. Mobile devices show numeric keyboard.
    Tel,
    /// Numeric input with optional min, max, and step constraints.
    Number,
    /// Slider control for selecting from a numeric range.
    Range,
    /// Date picker control (year, month, day).
    Date,
    /// Time picker control (hours and minutes).
    Time,
    /// Date and time picker without timezone information.
    DatetimeLocal,
    /// Month and year picker control.
    Month,
    /// Week and year picker control.
    Week,
    /// Color picker returning a hex color value.
    Color,
    /// Checkbox for boolean or multi-selection inputs.
    Checkbox,
    /// Radio button for single selection from a group.
    Radio,
    /// File upload control with optional accept and multiple attributes.
    File,
    /// Submit button that submits the form.
    Submit,
    /// Reset button that clears the form to default values.
    Reset,
    /// Generic button with no default behavior (use with JavaScript).
    Button,
    /// Image-based submit button with click coordinates.
    Image,
    /// Hidden input not displayed to users but submitted with form.
    Hidden,
    /// Search input with platform-specific search styling.
    Search,
}

impl AttributeValue for InputType {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Email => "email",
            Self::Url => "url",
            Self::Tel => "tel",
            Self::Number => "number",
            Self::Range => "range",
            Self::Date => "date",
            Self::Time => "time",
            Self::DatetimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::Color => "color",
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
            Self::File => "file",
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Button => "button",
            Self::Image => "image",
            Self::Hidden => "hidden",
            Self::Search => "search",
        })
    }
}

/// The `type` attribute values for `<button>` elements.
///
/// # Purpose
/// Defines the behavior of a button element, determining how it interacts
/// with forms and what action it performs when activated.
///
/// # Usage Context
/// - Used with: `<button>` elements only
/// - Default: `submit` if not specified and button is in a form
/// - Form association: Submit and reset affect the associated form
///
/// # Valid Values
/// - `Submit`: Submits the form when clicked
/// - `Reset`: Resets the form to default values
/// - `Button`: No default behavior (for custom JavaScript)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, ButtonType};
/// let btn_type = ButtonType::Submit;
/// assert_eq!(btn_type.to_attr_value(), "submit");
/// ```
///
/// ```html
/// <button type="submit">Submit Form</button>
/// <button type="reset">Reset Form</button>
/// <button type="button" onclick="handleClick()">Custom Action</button>
/// ```
///
/// # WHATWG Specification
/// - [The `button` element](https://html.spec.whatwg.org/multipage/form-elements.html#the-button-element)
/// - [Button type attribute](https://html.spec.whatwg.org/multipage/form-elements.html#attr-button-type)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    /// Submit button that submits the form when activated. This is the
    /// default if the button is within a form.
    Submit,
    /// Reset button that resets all form controls to their initial values.
    Reset,
    /// Regular button with no default behavior. Use with JavaScript event
    /// handlers for custom functionality.
    Button,
}

impl AttributeValue for ButtonType {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Button => "button",
        })
    }
}

/// The `autocomplete` attribute values.
///
/// # Purpose
/// Controls browser autofill behavior for form fields, specifying what type
/// of data the browser should suggest based on the user's stored information.
///
/// # Usage Context
/// - Used with: `<input>`, `<textarea>`, `<select>`, `<form>` elements
/// - Privacy: Users control what data is stored for autofill
/// - UX: Improves form completion speed and accuracy
///
/// # Valid Values
/// - `On`: Enable autofill with browser's default heuristics
/// - `Off`: Disable autofill for this field
/// - `Name`: Full name
/// - `Email`: Email address
/// - `Username`: Username or account name
/// - `NewPassword`: New password (e.g., registration, password change)
/// - `CurrentPassword`: Current password for login
/// - `OneTimeCode`: One-time verification code (2FA, SMS)
/// - `Organization`: Organization or company name
/// - `StreetAddress`: Full street address
/// - `Country`: Country or region code
/// - `PostalCode`: ZIP or postal code
/// - `CcNumber`: Credit card number
/// - `CcExp`: Credit card expiration date
/// - `CcCsc`: Credit card security code (CVV/CVC)
/// - `Tel`: Telephone number
/// - `Url`: URL or website address
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Autocomplete};
/// let autocomplete = Autocomplete::Email;
/// assert_eq!(autocomplete.to_attr_value(), "email");
/// ```
///
/// ```html
/// <input type="email" autocomplete="email">
/// <input type="password" autocomplete="current-password">
/// <input type="text" autocomplete="name">
/// <input type="tel" autocomplete="tel">
/// <input type="text" autocomplete="street-address">
/// <input type="text" autocomplete="off">
/// ```
///
/// # WHATWG Specification
/// - [Autofill](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#autofill)
/// - [Autocomplete attribute](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#attr-fe-autocomplete)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Autocomplete {
    /// Enable autofill with browser's default behavior.
    On,
    /// Disable autofill for this field (sensitive data, unique IDs).
    Off,
    /// Full name (combined given and family names).
    Name,
    /// Email address.
    Email,
    /// Username or account identifier.
    Username,
    /// New password being set (registration or password change forms).
    NewPassword,
    /// Current password for authentication (login forms).
    CurrentPassword,
    /// One-time code for two-factor authentication (SMS, authenticator app).
    OneTimeCode,
    /// Company or organization name.
    Organization,
    /// Full street address (may include multiple lines).
    StreetAddress,
    /// Country or region name/code.
    Country,
    /// Postal code or ZIP code.
    PostalCode,
    /// Credit card number.
    CcNumber,
    /// Credit card expiration date (month and year).
    CcExp,
    /// Credit card security code (CVV, CVC, CVV2).
    CcCsc,
    /// Telephone number including country code.
    Tel,
    /// URL or website address.
    Url,
}

impl AttributeValue for Autocomplete {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Name => "name",
            Self::Email => "email",
            Self::Username => "username",
            Self::NewPassword => "new-password",
            Self::CurrentPassword => "current-password",
            Self::OneTimeCode => "one-time-code",
            Self::Organization => "organization",
            Self::StreetAddress => "street-address",
            Self::Country => "country",
            Self::PostalCode => "postal-code",
            Self::CcNumber => "cc-number",
            Self::CcExp => "cc-exp",
            Self::CcCsc => "cc-csc",
            Self::Tel => "tel",
            Self::Url => "url",
        })
    }
}

/// The `method` attribute values for `<form>` elements.
///
/// # Purpose
/// Specifies the HTTP method used to submit the form data to the server,
/// controlling how data is sent and how it affects server state.
///
/// # Usage Context
/// - Used with: `<form>` elements
/// - Default: `get` if not specified
/// - Security: Use `post` for sensitive data or state-changing operations
///
/// # Valid Values
/// - `Get`: Submit data as URL query parameters (idempotent, cacheable)
/// - `Post`: Submit data in request body (for state changes, sensitive data)
/// - `Dialog`: Close dialog and submit without HTTP request
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Method};
/// let method = Method::Post;
/// assert_eq!(method.to_attr_value(), "post");
/// ```
///
/// ```html
/// <form action="/search" method="get">...</form>
/// <form action="/login" method="post">...</form>
/// <form method="dialog">...</form>
/// ```
///
/// # WHATWG Specification
/// - [Form submission](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-algorithm)
/// - [The `method` attribute](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#attr-fs-method)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// HTTP GET method. Data is appended to URL as query parameters.
    /// Use for searches, filters, and read-only operations.
    Get,
    /// HTTP POST method. Data is sent in request body. Use for creating
    /// or modifying data, and for sensitive information.
    Post,
    /// Dialog method. Closes the dialog containing the form without
    /// sending an HTTP request.
    Dialog,
}

impl AttributeValue for Method {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Dialog => "dialog",
        })
    }
}

/// The `enctype` attribute values for form encoding.
///
/// # Purpose
/// Specifies how form data should be encoded before sending to the server,
/// which is critical for proper handling of different types of form content.
///
/// # Usage Context
/// - Used with: `<form>` elements (and `<input>`/`<button>` with `formenctype`)
/// - Only relevant: When `method="post"`
/// - Default: `application/x-www-form-urlencoded`
///
/// # Valid Values
/// - `UrlEncoded`: Standard URL-encoded format (default)
/// - `Multipart`: Multipart form data (required for file uploads)
/// - `Plain`: Plain text format (rarely used, debugging only)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Enctype};
/// let enctype = Enctype::Multipart;
/// assert_eq!(enctype.to_attr_value(), "multipart/form-data");
/// ```
///
/// ```html
/// <form action="/submit" method="post" enctype="application/x-www-form-urlencoded">...</form>
/// <form action="/upload" method="post" enctype="multipart/form-data">
///   <input type="file" name="document">
/// </form>
/// <form action="/feedback" method="post" enctype="text/plain">...</form>
/// ```
///
/// # WHATWG Specification
/// - [Form submission](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-algorithm)
/// - [The `enctype` attribute](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#attr-fs-enctype)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enctype {
    /// URL-encoded format: `application/x-www-form-urlencoded`.
    /// Default encoding for forms. Key-value pairs are URL-encoded.
    UrlEncoded,
    /// Multipart format: `multipart/form-data`.
    /// Required for forms containing file uploads (`<input type="file">`).
    Multipart,
    /// Plain text format: `text/plain`.
    /// Data sent as plain text. Rarely used except for debugging.
    Plain,
}

impl AttributeValue for Enctype {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::UrlEncoded => "application/x-www-form-urlencoded",
            Self::Multipart => "multipart/form-data",
            Self::Plain => "text/plain",
        })
    }
}

/// The `wrap` attribute values for `<textarea>` elements.
///
/// # Purpose
/// Controls how text wrapping is handled when submitting textarea content,
/// determining whether hard line breaks are inserted at wrap points.
///
/// # Usage Context
/// - Used with: `<textarea>` elements only
/// - Default: `soft` if not specified
/// - Form submission: Affects submitted text formatting
///
/// # Valid Values
/// - `Hard`: Insert newlines at wrap points when submitting
/// - `Soft`: No newlines inserted (visual wrapping only)
/// - `Off`: Disable wrapping entirely
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Wrap};
/// let wrap = Wrap::Hard;
/// assert_eq!(wrap.to_attr_value(), "hard");
/// ```
///
/// ```html
/// <textarea wrap="soft" cols="40">Visual wrapping only</textarea>
/// <textarea wrap="hard" cols="40">Newlines inserted at wrap</textarea>
/// <textarea wrap="off">No wrapping</textarea>
/// ```
///
/// # WHATWG Specification
/// - [The `textarea` element](https://html.spec.whatwg.org/multipage/form-elements.html#the-textarea-element)
/// - [The `wrap` attribute](https://html.spec.whatwg.org/multipage/form-elements.html#attr-textarea-wrap)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrap {
    /// Hard wrapping. Browser inserts newline characters (CR+LF) at line
    /// wrap points when submitting the form. Requires `cols` attribute.
    Hard,
    /// Soft wrapping (default). Text wraps visually but no newlines are
    /// inserted in the submitted value.
    Soft,
    /// No wrapping. Text does not wrap; horizontal scrolling may occur.
    Off,
}

impl AttributeValue for Wrap {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Hard => "hard",
            Self::Soft => "soft",
            Self::Off => "off",
        })
    }
}

/// The `scope` attribute values for table header cells.
///
/// # Purpose
/// Specifies which cells a header (`<th>`) element applies to, improving
/// table accessibility by defining the relationship between headers and data.
///
/// # Usage Context
/// - Used with: `<th>` elements only
/// - Accessibility: Critical for screen readers to understand table structure
/// - Required: For complex tables with multiple header levels
///
/// # Valid Values
/// - `Row`: Header applies to all cells in the same row
/// - `Col`: Header applies to all cells in the same column
/// - `Rowgroup`: Header applies to row group (`<tbody>`, `<thead>`, `<tfoot>`)
/// - `Colgroup`: Header applies to column group (`<colgroup>`)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Scope};
/// let scope = Scope::Col;
/// assert_eq!(scope.to_attr_value(), "col");
/// ```
///
/// ```html
/// <table>
///   <thead>
///     <tr>
///       <th scope="col">Name</th>
///       <th scope="col">Age</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <th scope="row">John</th>
///       <td>30</td>
///     </tr>
///   </tbody>
/// </table>
/// ```
///
/// # WHATWG Specification
/// - [The `th` element](https://html.spec.whatwg.org/multipage/tables.html#the-th-element)
/// - [The `scope` attribute](https://html.spec.whatwg.org/multipage/tables.html#attr-th-scope)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// Header applies to all cells in the same row.
    Row,
    /// Header applies to all cells in the same column.
    Col,
    /// Header applies to all cells in the row group (`<tbody>`, `<thead>`,
    /// or `<tfoot>`).
    Rowgroup,
    /// Header applies to all cells in the column group (`<colgroup>`).
    Colgroup,
}

impl AttributeValue for Scope {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Row => "row",
            Self::Col => "col",
            Self::Rowgroup => "rowgroup",
            Self::Colgroup => "colgroup",
        })
    }
}

/// The `preload` attribute values for media elements.
///
/// # Purpose
/// Provides a hint to the browser about what media data to preload, balancing
/// user experience with bandwidth and resource consumption.
///
/// # Usage Context
/// - Used with: `<audio>`, `<video>` elements
/// - Performance: Controls bandwidth usage and buffering behavior
/// - Default: Browser-dependent (typically `metadata`)
///
/// # Valid Values
/// - `None`: Don't preload any data
/// - `Metadata`: Preload only metadata (duration, dimensions, first frame)
/// - `Auto`: Browser decides how much to preload (may load entire file)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Preload};
/// let preload = Preload::Metadata;
/// assert_eq!(preload.to_attr_value(), "metadata");
/// ```
///
/// ```html
/// <video src="movie.mp4" preload="none" controls></video>
/// <video src="tutorial.mp4" preload="metadata" controls></video>
/// <audio src="music.mp3" preload="auto" controls></audio>
/// ```
///
/// # WHATWG Specification
/// - [Media elements](https://html.spec.whatwg.org/multipage/media.html#media-elements)
/// - [The `preload` attribute](https://html.spec.whatwg.org/multipage/media.html#attr-media-preload)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preload {
    /// Don't preload any data. Minimizes bandwidth usage. User must
    /// explicitly start playback.
    None,
    /// Preload only metadata (duration, dimensions, track list, first frame).
    /// Good balance between UX and bandwidth.
    Metadata,
    /// Browser decides whether to preload data. May download the entire
    /// resource. Optimizes for user experience.
    Auto,
}

impl AttributeValue for Preload {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::None => "none",
            Self::Metadata => "metadata",
            Self::Auto => "auto",
        })
    }
}

/// The `kind` attribute values for `<track>` elements.
///
/// # Purpose
/// Specifies the type of text track, defining how the track content should
/// be interpreted and displayed for media accessibility and enhancement.
///
/// # Usage Context
/// - Used with: `<track>` elements (within `<audio>` or `<video>`)
/// - Accessibility: Subtitles and captions are crucial for accessibility
/// - Format: Tracks typically use `WebVTT` format
///
/// # Valid Values
/// - `Subtitles`: Transcription or translation for dialogue
/// - `Captions`: Transcription including sound effects (for hearing impaired)
/// - `Descriptions`: Text descriptions of visual content (for visually impaired)
/// - `Chapters`: Chapter titles for media navigation
/// - `Metadata`: Track for scripts (not displayed to user)
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, TrackKind};
/// let kind = TrackKind::Subtitles;
/// assert_eq!(kind.to_attr_value(), "subtitles");
/// ```
///
/// ```html
/// <video src="movie.mp4" controls>
///   <track kind="subtitles" src="subs-en.vtt" srclang="en" label="English">
///   <track kind="captions" src="caps-en.vtt" srclang="en" label="English CC">
///   <track kind="descriptions" src="desc.vtt" srclang="en">
///   <track kind="chapters" src="chapters.vtt" srclang="en">
/// </video>
/// ```
///
/// # WHATWG Specification
/// - [The `track` element](https://html.spec.whatwg.org/multipage/media.html#the-track-element)
/// - [Text track kind](https://html.spec.whatwg.org/multipage/media.html#attr-track-kind)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackKind {
    /// Subtitles provide translation of dialogue and text for users who
    /// understand the audio but need translation.
    Subtitles,
    /// Captions provide transcription and possibly translation of audio,
    /// including sound effects, music, and other audio information.
    Captions,
    /// Descriptions provide textual descriptions of video content for
    /// visually impaired users.
    Descriptions,
    /// Chapters provide chapter titles for media navigation and structure.
    Chapters,
    /// Metadata tracks are not displayed but used by scripts for enhanced
    /// interactivity.
    Metadata,
}

impl AttributeValue for TrackKind {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Subtitles => "subtitles",
            Self::Captions => "captions",
            Self::Descriptions => "descriptions",
            Self::Chapters => "chapters",
            Self::Metadata => "metadata",
        })
    }
}

/// The `sandbox` attribute values for `<iframe>` elements.
///
/// # Purpose
/// Enables extra restrictions on iframe content for security, allowing fine-grained
/// control over what capabilities the embedded content has access to.
///
/// # Usage Context
/// - Used with: `<iframe>` elements
/// - Security: Applies strict sandbox by default; flags allow specific capabilities
/// - Multiple values: Space-separated list of allowed capabilities
/// - Default: Empty sandbox (most restrictive)
///
/// # Valid Values
/// - `AllowForms`: Allow form submission
/// - `AllowModals`: Allow opening modal windows (alert, confirm, print)
/// - `AllowOrientationLock`: Allow screen orientation lock
/// - `AllowPointerLock`: Allow Pointer Lock API
/// - `AllowPopups`: Allow popups (window.open, target="_blank")
/// - `AllowPopupsToEscapeSandbox`: Allow popups without sandbox restrictions
/// - `AllowPresentation`: Allow Presentation API
/// - `AllowSameOrigin`: Treat content as same-origin (use with caution)
/// - `AllowScripts`: Allow JavaScript execution
/// - `AllowTopNavigation`: Allow navigating top-level browsing context
/// - `AllowTopNavigationByUserActivation`: Allow top navigation only from user gesture
///
/// # Example
/// ```rust
/// use ironhtml_attributes::{AttributeValue, Sandbox};
/// let sandbox = Sandbox::AllowScripts;
/// assert_eq!(sandbox.to_attr_value(), "allow-scripts");
/// ```
///
/// ```html
/// <iframe src="untrusted.html" sandbox></iframe>
/// <iframe src="widget.html" sandbox="allow-scripts allow-same-origin"></iframe>
/// <iframe src="game.html" sandbox="allow-scripts allow-pointer-lock"></iframe>
/// ```
///
/// # WHATWG Specification
/// - [The `iframe` element](https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-iframe-element)
/// - [The `sandbox` attribute](https://html.spec.whatwg.org/multipage/iframe-embed-object.html#attr-iframe-sandbox)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sandbox {
    /// Allow form submission from the sandboxed content.
    AllowForms,
    /// Allow the sandboxed content to open modal windows (alert, confirm,
    /// print, etc.).
    AllowModals,
    /// Allow the sandboxed content to lock the screen orientation.
    AllowOrientationLock,
    /// Allow the sandboxed content to use the Pointer Lock API.
    AllowPointerLock,
    /// Allow the sandboxed content to open popup windows.
    AllowPopups,
    /// Allow popups opened by the sandboxed content to not inherit the
    /// sandbox restrictions.
    AllowPopupsToEscapeSandbox,
    /// Allow the sandboxed content to use the Presentation API.
    AllowPresentation,
    /// Allow the content to be treated as being from its normal origin.
    /// WARNING: Dangerous when combined with `allow-scripts`.
    AllowSameOrigin,
    /// Allow the sandboxed content to run scripts (but not create popups).
    AllowScripts,
    /// Allow the sandboxed content to navigate the top-level browsing
    /// context (the full window).
    AllowTopNavigation,
    /// Allow top-level navigation only when triggered by user activation
    /// (safer than `allow-top-navigation`).
    AllowTopNavigationByUserActivation,
}

impl AttributeValue for Sandbox {
    fn to_attr_value(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::AllowForms => "allow-forms",
            Self::AllowModals => "allow-modals",
            Self::AllowOrientationLock => "allow-orientation-lock",
            Self::AllowPointerLock => "allow-pointer-lock",
            Self::AllowPopups => "allow-popups",
            Self::AllowPopupsToEscapeSandbox => "allow-popups-to-escape-sandbox",
            Self::AllowPresentation => "allow-presentation",
            Self::AllowSameOrigin => "allow-same-origin",
            Self::AllowScripts => "allow-scripts",
            Self::AllowTopNavigation => "allow-top-navigation",
            Self::AllowTopNavigationByUserActivation => "allow-top-navigation-by-user-activation",
        })
    }
}

// =============================================================================
// Global Attributes
// =============================================================================

/// Attribute names for global HTML attributes.
///
/// # Purpose
/// Global attributes can be used on any HTML element, providing common
/// functionality like styling, identification, accessibility, and interaction
/// control across all elements.
///
/// # Common Attributes
/// - `class`: Space-separated CSS class names for styling
/// - `id`: Unique identifier for the element
/// - `style`: Inline CSS styles
/// - `title`: Advisory information (tooltip text)
/// - `lang`: Language of the element's content
/// - `dir`: Text directionality (ltr, rtl, auto)
/// - `tabindex`: Tab order for keyboard navigation
/// - `hidden`: Hides element from rendering
/// - `contenteditable`: Makes element content editable
/// - `draggable`: Enables drag-and-drop
/// - `role`: ARIA role for accessibility
///
/// # Example
/// ```html
/// <div class="container" id="main" lang="en" dir="ltr">
///   <p class="text" title="Helpful tooltip" tabindex="0">Content</p>
///   <span role="button" draggable="true">Drag me</span>
/// </div>
/// ```
///
/// # WHATWG Specification
/// - [Global attributes](https://html.spec.whatwg.org/multipage/dom.html#global-attributes)
pub mod global {
    /// The `class` attribute.
    ///
    /// Space-separated list of CSS class names for styling and JavaScript selection.
    pub const CLASS: &str = "class";

    /// The `id` attribute.
    ///
    /// Unique identifier for the element within the document.
    pub const ID: &str = "id";

    /// The `style` attribute.
    ///
    /// Inline CSS style declarations for the element.
    pub const STYLE: &str = "style";

    /// The `title` attribute.
    ///
    /// Advisory information displayed as a tooltip on hover.
    pub const TITLE: &str = "title";

    /// The `lang` attribute.
    ///
    /// Language of the element's content (e.g., "en", "es", "fr").
    pub const LANG: &str = "lang";

    /// The `dir` attribute.
    ///
    /// Text directionality: "ltr" (left-to-right), "rtl" (right-to-left), or "auto".
    pub const DIR: &str = "dir";

    /// The `tabindex` attribute.
    ///
    /// Controls tab order for keyboard navigation. Values: positive (explicit order),
    /// 0 (natural order), -1 (programmatically focusable only).
    pub const TABINDEX: &str = "tabindex";

    /// The `accesskey` attribute.
    ///
    /// Keyboard shortcut to activate or focus the element.
    pub const ACCESSKEY: &str = "accesskey";

    /// The `contenteditable` attribute.
    ///
    /// Indicates whether the element's content is editable: "true", "false", or "inherit".
    pub const CONTENTEDITABLE: &str = "contenteditable";

    /// The `draggable` attribute.
    ///
    /// Indicates whether the element can be dragged: "true" or "false".
    pub const DRAGGABLE: &str = "draggable";

    /// The `hidden` attribute.
    ///
    /// Boolean attribute that hides the element from rendering and accessibility tree.
    /// Can also use "until-found" for searchable hidden content.
    pub const HIDDEN: &str = "hidden";

    /// The `spellcheck` attribute.
    ///
    /// Controls spell-checking for editable content: "true" or "false".
    pub const SPELLCHECK: &str = "spellcheck";

    /// The `translate` attribute.
    ///
    /// Indicates whether content should be translated: "yes" or "no".
    pub const TRANSLATE: &str = "translate";

    /// The `role` attribute.
    ///
    /// ARIA role for accessibility, defining the element's purpose for assistive technologies.
    pub const ROLE: &str = "role";

    /// The `slot` attribute.
    ///
    /// Assigns the element to a named slot in a shadow DOM template.
    pub const SLOT: &str = "slot";
}

// =============================================================================
// Element-Specific Attribute Names
// =============================================================================

/// Attribute names for anchor (`<a>`) elements.
///
/// # Purpose
/// The `<a>` element creates hyperlinks to other pages, locations within the
/// same page, files, email addresses, or any other URL.
///
/// # Common Attributes
/// - `href`: The URL that the hyperlink points to
/// - `target`: Where to display the linked URL (_blank, _self, _parent, _top)
/// - `rel`: Relationship between current and linked document
/// - `download`: Suggests downloading the linked resource
/// - `referrerpolicy`: Controls referrer information sent with requests
///
/// # Example
/// ```html
/// <a href="https://example.com" target="_blank" rel="noopener noreferrer">
///   External Link
/// </a>
/// <a href="/page" hreflang="en">Internal Link</a>
/// <a href="document.pdf" download="filename.pdf">Download PDF</a>
/// ```
///
/// # WHATWG Specification
/// - [The `a` element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element)
pub mod anchor {
    /// The `href` attribute.
    ///
    /// URL or URL fragment that the hyperlink points to.
    pub const HREF: &str = "href";

    /// The `target` attribute.
    ///
    /// Browsing context for link navigation: "_blank", "_self", "_parent", "_top",
    /// or a named context.
    pub const TARGET: &str = "target";

    /// The `rel` attribute.
    ///
    /// Relationship between current document and linked resource (e.g., "noopener",
    /// "noreferrer", "nofollow").
    pub const REL: &str = "rel";

    /// The `download` attribute.
    ///
    /// Prompts to download the linked resource. Value suggests the filename.
    pub const DOWNLOAD: &str = "download";

    /// The `hreflang` attribute.
    ///
    /// Language of the linked resource (e.g., "en", "es", "fr").
    pub const HREFLANG: &str = "hreflang";

    /// The `type` attribute.
    ///
    /// MIME type hint for the linked resource (e.g., "application/pdf").
    pub const TYPE: &str = "type";

    /// The `referrerpolicy` attribute.
    ///
    /// Controls how much referrer information is sent when following the link.
    pub const REFERRERPOLICY: &str = "referrerpolicy";
}

/// Attribute names for image (`<img>`) elements.
///
/// # Purpose
/// The `<img>` element embeds images into documents. It supports responsive
/// images, lazy loading, and various performance optimization features.
///
/// # Common Attributes
/// - `src`: Image source URL (required)
/// - `alt`: Alternative text for accessibility (required)
/// - `width`/`height`: Dimensions to prevent layout shift
/// - `loading`: Lazy loading behavior (eager/lazy)
/// - `srcset`: Responsive image sources
/// - `sizes`: Image sizes for different viewport widths
///
/// # Example
/// ```html
/// <img src="photo.jpg" alt="A beautiful sunset" width="800" height="600">
/// <img src="hero.jpg" alt="Hero" loading="eager">
/// <img src="gallery.jpg" alt="Gallery" loading="lazy"
///      srcset="small.jpg 480w, large.jpg 1024w" sizes="(max-width: 600px) 480px, 1024px">
/// ```
///
/// # WHATWG Specification
/// - [The `img` element](https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element)
pub mod img {
    /// The `src` attribute.
    ///
    /// URL of the image resource. Required unless `srcset` is used.
    pub const SRC: &str = "src";

    /// The `alt` attribute.
    ///
    /// Alternative text description for accessibility. Required for valid HTML.
    pub const ALT: &str = "alt";

    /// The `width` attribute.
    ///
    /// Intrinsic width of the image in pixels. Helps prevent layout shift.
    pub const WIDTH: &str = "width";

    /// The `height` attribute.
    ///
    /// Intrinsic height of the image in pixels. Helps prevent layout shift.
    pub const HEIGHT: &str = "height";

    /// The `loading` attribute.
    ///
    /// Loading behavior: "eager" (immediate) or "lazy" (when near viewport).
    pub const LOADING: &str = "loading";

    /// The `decoding` attribute.
    ///
    /// Image decoding hint: "sync", "async", or "auto".
    pub const DECODING: &str = "decoding";

    /// The `crossorigin` attribute.
    ///
    /// CORS settings: "anonymous" or "use-credentials".
    pub const CROSSORIGIN: &str = "crossorigin";

    /// The `referrerpolicy` attribute.
    ///
    /// Controls referrer information sent when fetching the image.
    pub const REFERRERPOLICY: &str = "referrerpolicy";

    /// The `srcset` attribute.
    ///
    /// Comma-separated list of image sources with width or pixel density descriptors
    /// for responsive images.
    pub const SRCSET: &str = "srcset";

    /// The `sizes` attribute.
    ///
    /// Media conditions and image sizes for responsive images (used with `srcset`).
    pub const SIZES: &str = "sizes";

    /// The `usemap` attribute.
    ///
    /// Associates the image with an image map (`<map>` element).
    pub const USEMAP: &str = "usemap";

    /// The `ismap` attribute.
    ///
    /// Boolean attribute indicating server-side image map.
    pub const ISMAP: &str = "ismap";
}

/// Attribute names for input (`<input>`) elements.
///
/// # Purpose
/// The `<input>` element creates interactive form controls for collecting user
/// data. It supports numerous input types from text to files, with extensive
/// validation and constraint capabilities.
///
/// # Common Attributes
/// - `type`: Input control type (text, email, password, etc.)
/// - `name`: Form control name for submission
/// - `value`: Current value of the control
/// - `placeholder`: Hint text displayed when empty
/// - `required`: Boolean indicating required field
/// - `pattern`: Regular expression for validation
/// - `min`/`max`: Range constraints for numeric/date inputs
///
/// # Example
/// ```html
/// <input type="text" name="username" placeholder="Enter username" required>
/// <input type="email" name="email" autocomplete="email">
/// <input type="password" name="pass" minlength="8" autocomplete="current-password">
/// <input type="number" name="age" min="0" max="120" step="1">
/// <input type="file" name="upload" accept="image/*" multiple>
/// ```
///
/// # WHATWG Specification
/// - [The `input` element](https://html.spec.whatwg.org/multipage/input.html#the-input-element)
pub mod input {
    /// The `type` attribute.
    ///
    /// Specifies the input control type (text, email, password, number, etc.).
    pub const TYPE: &str = "type";

    /// The `name` attribute.
    ///
    /// Name of the form control, submitted with the form data.
    pub const NAME: &str = "name";

    /// The `value` attribute.
    ///
    /// Current value of the form control.
    pub const VALUE: &str = "value";

    /// The `placeholder` attribute.
    ///
    /// Hint text displayed when the input is empty.
    pub const PLACEHOLDER: &str = "placeholder";

    /// The `required` attribute.
    ///
    /// Boolean indicating the field must be filled before form submission.
    pub const REQUIRED: &str = "required";

    /// The `disabled` attribute.
    ///
    /// Boolean disabling the input (not submitted, not editable).
    pub const DISABLED: &str = "disabled";

    /// The `readonly` attribute.
    ///
    /// Boolean making the input read-only (submitted but not editable).
    pub const READONLY: &str = "readonly";

    /// The `checked` attribute.
    ///
    /// Boolean for checkbox/radio inputs indicating checked state.
    pub const CHECKED: &str = "checked";

    /// The `autocomplete` attribute.
    ///
    /// Controls autofill behavior (on, off, email, username, etc.).
    pub const AUTOCOMPLETE: &str = "autocomplete";

    /// The `autofocus` attribute.
    ///
    /// Boolean to automatically focus this input when page loads.
    pub const AUTOFOCUS: &str = "autofocus";

    /// The `min` attribute.
    ///
    /// Minimum value for numeric, date, or time inputs.
    pub const MIN: &str = "min";

    /// The `max` attribute.
    ///
    /// Maximum value for numeric, date, or time inputs.
    pub const MAX: &str = "max";

    /// The `step` attribute.
    ///
    /// Increment step for numeric or date/time inputs.
    pub const STEP: &str = "step";

    /// The `minlength` attribute.
    ///
    /// Minimum character length for text inputs.
    pub const MINLENGTH: &str = "minlength";

    /// The `maxlength` attribute.
    ///
    /// Maximum character length for text inputs.
    pub const MAXLENGTH: &str = "maxlength";

    /// The `pattern` attribute.
    ///
    /// Regular expression for input validation.
    pub const PATTERN: &str = "pattern";

    /// The `size` attribute.
    ///
    /// Visual width of the input in characters.
    pub const SIZE: &str = "size";

    /// The `accept` attribute.
    ///
    /// File types accepted by file inputs (MIME types or extensions).
    pub const ACCEPT: &str = "accept";

    /// The `multiple` attribute.
    ///
    /// Boolean allowing multiple values (for file or email inputs).
    pub const MULTIPLE: &str = "multiple";

    /// The `list` attribute.
    ///
    /// ID of a `<datalist>` element providing autocomplete suggestions.
    pub const LIST: &str = "list";

    /// The `form` attribute.
    ///
    /// ID of the form this input belongs to (if outside the form element).
    pub const FORM: &str = "form";
}

/// Attribute names for button (`<button>`) elements.
///
/// # Purpose
/// The `<button>` element represents a clickable button that can submit forms,
/// reset forms, or trigger custom JavaScript actions.
///
/// # Common Attributes
/// - `type`: Button behavior (submit, reset, button)
/// - `name`: Form control name for submission
/// - `value`: Value submitted with form
/// - `disabled`: Disables the button
/// - `form`: Associates button with a form by ID
///
/// # Example
/// ```html
/// <button type="submit" name="action" value="save">Save</button>
/// <button type="reset">Reset Form</button>
/// <button type="button" onclick="handleClick()">Click Me</button>
/// <button type="submit" formaction="/alt" formmethod="post">Alternative Submit</button>
/// ```
///
/// # WHATWG Specification
/// - [The `button` element](https://html.spec.whatwg.org/multipage/form-elements.html#the-button-element)
pub mod button {
    /// The `type` attribute.
    ///
    /// Button behavior: "submit", "reset", or "button".
    pub const TYPE: &str = "type";

    /// The `name` attribute.
    ///
    /// Name of the button, submitted with form data when clicked.
    pub const NAME: &str = "name";

    /// The `value` attribute.
    ///
    /// Value submitted with the form when this button is clicked.
    pub const VALUE: &str = "value";

    /// The `disabled` attribute.
    ///
    /// Boolean disabling the button (not interactive, not submitted).
    pub const DISABLED: &str = "disabled";

    /// The `autofocus` attribute.
    ///
    /// Boolean to automatically focus this button when page loads.
    pub const AUTOFOCUS: &str = "autofocus";

    /// The `form` attribute.
    ///
    /// ID of the form this button is associated with.
    pub const FORM: &str = "form";

    /// The `formaction` attribute.
    ///
    /// URL to submit the form to (overrides form's action).
    pub const FORMACTION: &str = "formaction";

    /// The `formmethod` attribute.
    ///
    /// HTTP method for form submission (overrides form's method).
    pub const FORMMETHOD: &str = "formmethod";

    /// The `formenctype` attribute.
    ///
    /// Form encoding type (overrides form's enctype).
    pub const FORMENCTYPE: &str = "formenctype";

    /// The `formnovalidate` attribute.
    ///
    /// Boolean to skip form validation when submitting.
    pub const FORMNOVALIDATE: &str = "formnovalidate";

    /// The `formtarget` attribute.
    ///
    /// Browsing context for form submission (overrides form's target).
    pub const FORMTARGET: &str = "formtarget";
}

/// Attribute names for form (`<form>`) elements.
///
/// # Purpose
/// The `<form>` element represents a document section containing interactive
/// controls for submitting information to a server.
///
/// # Common Attributes
/// - `action`: URL to submit the form data to
/// - `method`: HTTP method (GET or POST)
/// - `enctype`: Encoding type for form data
/// - `novalidate`: Skip browser validation
/// - `autocomplete`: Controls autofill for all form fields
///
/// # Example
/// ```html
/// <form action="/submit" method="post" enctype="multipart/form-data">
///   <input type="file" name="upload">
///   <button type="submit">Upload</button>
/// </form>
/// <form action="/search" method="get" autocomplete="off">
///   <input type="text" name="q">
/// </form>
/// ```
///
/// # WHATWG Specification
/// - [The `form` element](https://html.spec.whatwg.org/multipage/forms.html#the-form-element)
pub mod form {
    /// The `action` attribute.
    ///
    /// URL where the form data will be submitted.
    pub const ACTION: &str = "action";

    /// The `method` attribute.
    ///
    /// HTTP method for form submission: "get", "post", or "dialog".
    pub const METHOD: &str = "method";

    /// The `enctype` attribute.
    ///
    /// Encoding type for form data: "application/x-www-form-urlencoded",
    /// "multipart/form-data", or "text/plain".
    pub const ENCTYPE: &str = "enctype";

    /// The `target` attribute.
    ///
    /// Browsing context for displaying the response (_blank, _self, etc.).
    pub const TARGET: &str = "target";

    /// The `novalidate` attribute.
    ///
    /// Boolean to skip HTML5 form validation on submission.
    pub const NOVALIDATE: &str = "novalidate";

    /// The `autocomplete` attribute.
    ///
    /// Controls autofill for all form controls: "on" or "off".
    pub const AUTOCOMPLETE: &str = "autocomplete";

    /// The `name` attribute.
    ///
    /// Name of the form for document.forms collection.
    pub const NAME: &str = "name";

    /// The `accept-charset` attribute.
    ///
    /// Character encodings accepted for form submission.
    pub const ACCEPTCHARSET: &str = "accept-charset";
}

/// Attribute names for textarea (`<textarea>`) elements.
///
/// # Purpose
/// The `<textarea>` element represents a multi-line plain text editing control,
/// useful for collecting longer text input from users.
///
/// # Common Attributes
/// - `name`: Form control name for submission
/// - `rows`/`cols`: Visual dimensions in characters
/// - `placeholder`: Hint text when empty
/// - `maxlength`: Maximum character limit
/// - `wrap`: Text wrapping behavior (soft, hard, off)
/// - `required`: Makes the field mandatory
///
/// # Example
/// ```html
/// <textarea name="comments" rows="4" cols="50" placeholder="Enter comments"
///           maxlength="500" required></textarea>
/// <textarea name="code" wrap="off" spellcheck="false"></textarea>
/// ```
///
/// # WHATWG Specification
/// - [The `textarea` element](https://html.spec.whatwg.org/multipage/form-elements.html#the-textarea-element)
pub mod textarea {
    /// The `name` attribute.
    ///
    /// Name of the form control, submitted with the form data.
    pub const NAME: &str = "name";

    /// The `placeholder` attribute.
    ///
    /// Hint text displayed when the textarea is empty.
    pub const PLACEHOLDER: &str = "placeholder";

    /// The `required` attribute.
    ///
    /// Boolean indicating the field must be filled before form submission.
    pub const REQUIRED: &str = "required";

    /// The `disabled` attribute.
    ///
    /// Boolean disabling the textarea (not submitted, not editable).
    pub const DISABLED: &str = "disabled";

    /// The `readonly` attribute.
    ///
    /// Boolean making the textarea read-only (submitted but not editable).
    pub const READONLY: &str = "readonly";

    /// The `autocomplete` attribute.
    ///
    /// Controls autofill behavior for the textarea.
    pub const AUTOCOMPLETE: &str = "autocomplete";

    /// The `autofocus` attribute.
    ///
    /// Boolean to automatically focus this textarea when page loads.
    pub const AUTOFOCUS: &str = "autofocus";

    /// The `rows` attribute.
    ///
    /// Number of visible text lines (height).
    pub const ROWS: &str = "rows";

    /// The `cols` attribute.
    ///
    /// Visible width of the textarea in average character widths.
    pub const COLS: &str = "cols";

    /// The `minlength` attribute.
    ///
    /// Minimum character length for validation.
    pub const MINLENGTH: &str = "minlength";

    /// The `maxlength` attribute.
    ///
    /// Maximum character length allowed.
    pub const MAXLENGTH: &str = "maxlength";

    /// The `wrap` attribute.
    ///
    /// Text wrapping behavior: "soft", "hard", or "off".
    pub const WRAP: &str = "wrap";

    /// The `form` attribute.
    ///
    /// ID of the form this textarea belongs to.
    pub const FORM: &str = "form";
}

/// Attribute names for select (`<select>`) elements.
///
/// # Purpose
/// The `<select>` element represents a control for selecting from a set of
/// options, displayed as a dropdown menu or list box.
///
/// # Common Attributes
/// - `name`: Form control name for submission
/// - `multiple`: Allows selecting multiple options
/// - `size`: Number of visible options (list vs dropdown)
/// - `required`: Makes selection mandatory
/// - `disabled`: Disables the control
///
/// # Example
/// ```html
/// <select name="country" required>
///   <option value="">Select a country</option>
///   <option value="us">United States</option>
///   <option value="uk">United Kingdom</option>
/// </select>
/// <select name="tags" multiple size="5">
///   <option>JavaScript</option>
///   <option>Python</option>
///   <option>Rust</option>
/// </select>
/// ```
///
/// # WHATWG Specification
/// - [The `select` element](https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element)
pub mod select {
    /// The `name` attribute.
    ///
    /// Name of the form control, submitted with the form data.
    pub const NAME: &str = "name";

    /// The `required` attribute.
    ///
    /// Boolean indicating an option must be selected before form submission.
    pub const REQUIRED: &str = "required";

    /// The `disabled` attribute.
    ///
    /// Boolean disabling the select control (not submitted, not interactive).
    pub const DISABLED: &str = "disabled";

    /// The `autofocus` attribute.
    ///
    /// Boolean to automatically focus this select when page loads.
    pub const AUTOFOCUS: &str = "autofocus";

    /// The `multiple` attribute.
    ///
    /// Boolean allowing multiple option selection.
    pub const MULTIPLE: &str = "multiple";

    /// The `size` attribute.
    ///
    /// Number of visible options. Values > 1 show as list box instead of dropdown.
    pub const SIZE: &str = "size";

    /// The `form` attribute.
    ///
    /// ID of the form this select belongs to.
    pub const FORM: &str = "form";

    /// The `autocomplete` attribute.
    ///
    /// Controls autofill behavior for the select control.
    pub const AUTOCOMPLETE: &str = "autocomplete";
}

/// Attribute names for option (`<option>`) elements.
///
/// # Purpose
/// The `<option>` element defines an option in a `<select>`, `<optgroup>`,
/// or `<datalist>` element, representing a choice available to the user.
///
/// # Common Attributes
/// - `value`: Value submitted when option is selected
/// - `selected`: Pre-selects the option
/// - `disabled`: Disables the option
/// - `label`: Text label (defaults to element content)
///
/// # Example
/// ```html
/// <select name="size">
///   <option value="">Choose size</option>
///   <option value="s">Small</option>
///   <option value="m" selected>Medium</option>
///   <option value="l">Large</option>
///   <option value="xl" disabled>Extra Large (Out of stock)</option>
/// </select>
/// ```
///
/// # WHATWG Specification
/// - [The `option` element](https://html.spec.whatwg.org/multipage/form-elements.html#the-option-element)
pub mod option {
    /// The `value` attribute.
    ///
    /// Value submitted with the form when this option is selected.
    /// If omitted, the text content is used.
    pub const VALUE: &str = "value";

    /// The `selected` attribute.
    ///
    /// Boolean indicating the option is initially selected.
    pub const SELECTED: &str = "selected";

    /// The `disabled` attribute.
    ///
    /// Boolean disabling the option (cannot be selected).
    pub const DISABLED: &str = "disabled";

    /// The `label` attribute.
    ///
    /// Label text for the option. If omitted, element content is used.
    pub const LABEL: &str = "label";
}

/// Attribute names for label (`<label>`) elements.
///
/// # Purpose
/// The `<label>` element represents a caption for a form control, improving
/// accessibility and usability by associating descriptive text with inputs.
///
/// # Common Attributes
/// - `for`: ID of the associated form control
///
/// # Example
/// ```html
/// <label for="email">Email Address:</label>
/// <input type="email" id="email" name="email">
///
/// <label>
///   Username:
///   <input type="text" name="username">
/// </label>
/// ```
///
/// # WHATWG Specification
/// - [The `label` element](https://html.spec.whatwg.org/multipage/forms.html#the-label-element)
pub mod label {
    /// The `for` attribute.
    ///
    /// ID of the form control this label is associated with. Clicking the
    /// label activates the control. Alternative: nest the control inside the label.
    pub const FOR: &str = "for";
}

/// Attribute names for table (`<table>`) elements.
///
/// # Purpose
/// The `<table>` element represents tabular data displayed in a grid of rows
/// and columns. Use semantic table elements for proper structure.
///
/// # Common Attributes
/// - `border`: Border width (deprecated, use CSS instead)
///
/// # Example
/// ```html
/// <table>
///   <thead>
///     <tr>
///       <th>Name</th>
///       <th>Age</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Alice</td>
///       <td>30</td>
///     </tr>
///   </tbody>
/// </table>
/// ```
///
/// # WHATWG Specification
/// - [The `table` element](https://html.spec.whatwg.org/multipage/tables.html#the-table-element)
pub mod table {
    /// The `border` attribute.
    ///
    /// Deprecated. Specifies border width. Use CSS `border` property instead.
    pub const BORDER: &str = "border";
}

/// Attribute names for table cell (`<td>`, `<th>`) elements.
///
/// # Purpose
/// Table data (`<td>`) and header (`<th>`) cells can span multiple rows or
/// columns and be associated with headers for accessibility.
///
/// # Common Attributes
/// - `colspan`: Number of columns the cell spans
/// - `rowspan`: Number of rows the cell spans
/// - `headers`: Space-separated list of header cell IDs
/// - `scope`: Scope of header cell (row, col, rowgroup, colgroup)
///
/// # Example
/// ```html
/// <table>
///   <tr>
///     <th id="name" scope="col">Name</th>
///     <th id="age" scope="col">Age</th>
///   </tr>
///   <tr>
///     <td headers="name">Alice</td>
///     <td headers="age">30</td>
///   </tr>
///   <tr>
///     <td colspan="2">Merged cell across two columns</td>
///   </tr>
/// </table>
/// ```
///
/// # WHATWG Specification
/// - [The `td` element](https://html.spec.whatwg.org/multipage/tables.html#the-td-element)
/// - [The `th` element](https://html.spec.whatwg.org/multipage/tables.html#the-th-element)
pub mod tablecell {
    /// The `colspan` attribute.
    ///
    /// Number of columns the cell spans. Default is 1.
    pub const COLSPAN: &str = "colspan";

    /// The `rowspan` attribute.
    ///
    /// Number of rows the cell spans. Default is 1.
    pub const ROWSPAN: &str = "rowspan";

    /// The `headers` attribute.
    ///
    /// Space-separated list of header cell IDs this cell is associated with.
    /// Improves accessibility for complex tables.
    pub const HEADERS: &str = "headers";

    /// The `scope` attribute.
    ///
    /// For `<th>` elements: specifies which cells the header applies to
    /// ("row", "col", "rowgroup", "colgroup").
    pub const SCOPE: &str = "scope";
}

/// Attribute names for media (`<audio>`, `<video>`) elements.
///
/// # Purpose
/// Media elements embed audio or video content with built-in playback controls.
/// They support multiple sources, captions, and extensive playback control.
///
/// # Common Attributes
/// - `src`: Media resource URL
/// - `controls`: Show browser's default playback controls
/// - `autoplay`: Automatically start playback (often restricted by browsers)
/// - `loop`: Restart playback when finished
/// - `muted`: Start muted (required for autoplay in many browsers)
/// - `preload`: How much to preload (none, metadata, auto)
///
/// # Example
/// ```html
/// <video src="movie.mp4" controls width="640" height="360" poster="thumb.jpg">
///   Your browser doesn't support video.
/// </video>
/// <audio src="music.mp3" controls loop preload="metadata"></audio>
/// <video controls autoplay muted loop>
///   <source src="video.webm" type="video/webm">
///   <source src="video.mp4" type="video/mp4">
/// </video>
/// ```
///
/// # WHATWG Specification
/// - [The `video` element](https://html.spec.whatwg.org/multipage/media.html#the-video-element)
/// - [The `audio` element](https://html.spec.whatwg.org/multipage/media.html#the-audio-element)
pub mod media {
    /// The `src` attribute.
    ///
    /// URL of the media resource. Alternative: use `<source>` child elements.
    pub const SRC: &str = "src";

    /// The `controls` attribute.
    ///
    /// Boolean to display browser's default playback controls.
    pub const CONTROLS: &str = "controls";

    /// The `autoplay` attribute.
    ///
    /// Boolean to automatically start playback. Many browsers require `muted`
    /// for autoplay to work.
    pub const AUTOPLAY: &str = "autoplay";

    /// The `loop` attribute.
    ///
    /// Boolean to restart playback from the beginning when finished.
    pub const LOOP: &str = "loop";

    /// The `muted` attribute.
    ///
    /// Boolean to start with audio muted. Required for autoplay in many browsers.
    pub const MUTED: &str = "muted";

    /// The `preload` attribute.
    ///
    /// Hint for how much to preload: "none", "metadata", or "auto".
    pub const PRELOAD: &str = "preload";

    /// The `poster` attribute.
    ///
    /// For `<video>`: URL of an image to show before playback starts.
    pub const POSTER: &str = "poster";

    /// The `width` attribute.
    ///
    /// For `<video>`: display width in pixels.
    pub const WIDTH: &str = "width";

    /// The `height` attribute.
    ///
    /// For `<video>`: display height in pixels.
    pub const HEIGHT: &str = "height";

    /// The `crossorigin` attribute.
    ///
    /// CORS settings for fetching the media: "anonymous" or "use-credentials".
    pub const CROSSORIGIN: &str = "crossorigin";
}

/// Attribute names for source (`<source>`) elements.
///
/// # Purpose
/// The `<source>` element specifies multiple media resources for `<picture>`,
/// `<audio>`, or `<video>` elements, enabling responsive images and fallback
/// media formats.
///
/// # Common Attributes
/// - `src`: Resource URL
/// - `type`: MIME type of the resource
/// - `srcset`: Responsive image sources (for `<picture>`)
/// - `sizes`: Image sizes for different viewports (for `<picture>`)
/// - `media`: Media query for when this source applies
///
/// # Example
/// ```html
/// <video controls>
///   <source src="video.webm" type="video/webm">
///   <source src="video.mp4" type="video/mp4">
/// </video>
/// <picture>
///   <source srcset="large.jpg" media="(min-width: 800px)">
///   <source srcset="small.jpg" media="(max-width: 799px)">
///   <img src="fallback.jpg" alt="Image">
/// </picture>
/// ```
///
/// # WHATWG Specification
/// - [The `source` element](https://html.spec.whatwg.org/multipage/embedded-content.html#the-source-element)
pub mod source {
    /// The `src` attribute.
    ///
    /// URL of the media resource (for media elements).
    pub const SRC: &str = "src";

    /// The `type` attribute.
    ///
    /// MIME type of the resource, helping the browser choose appropriate format.
    pub const TYPE: &str = "type";

    /// The `srcset` attribute.
    ///
    /// For `<picture>`: comma-separated list of image URLs with width/density descriptors.
    pub const SRCSET: &str = "srcset";

    /// The `sizes` attribute.
    ///
    /// For `<picture>`: media conditions and image sizes for responsive images.
    pub const SIZES: &str = "sizes";

    /// The `media` attribute.
    ///
    /// Media query determining when this source should be used.
    pub const MEDIA: &str = "media";
}

/// Attribute names for track (`<track>`) elements.
///
/// # Purpose
/// The `<track>` element provides timed text tracks (like subtitles, captions,
/// or chapters) for `<audio>` and `<video>` elements, improving accessibility
/// and user experience.
///
/// # Common Attributes
/// - `src`: URL of the track file (`WebVTT` format)
/// - `kind`: Type of track (subtitles, captions, descriptions, chapters, metadata)
/// - `srclang`: Language of the track text
/// - `label`: User-visible label for the track
/// - `default`: Marks this track as default
///
/// # Example
/// ```html
/// <video src="movie.mp4" controls>
///   <track kind="subtitles" src="en.vtt" srclang="en" label="English" default>
///   <track kind="subtitles" src="es.vtt" srclang="es" label="Español">
///   <track kind="captions" src="en-cc.vtt" srclang="en" label="English CC">
///   <track kind="chapters" src="chapters.vtt" srclang="en">
/// </video>
/// ```
///
/// # WHATWG Specification
/// - [The `track` element](https://html.spec.whatwg.org/multipage/media.html#the-track-element)
pub mod track {
    /// The `src` attribute.
    ///
    /// URL of the track file, typically in `WebVTT` format.
    pub const SRC: &str = "src";

    /// The `kind` attribute.
    ///
    /// Type of text track: "subtitles", "captions", "descriptions",
    /// "chapters", or "metadata".
    pub const KIND: &str = "kind";

    /// The `srclang` attribute.
    ///
    /// Language of the track text data (BCP 47 language tag, e.g., "en", "es").
    pub const SRCLANG: &str = "srclang";

    /// The `label` attribute.
    ///
    /// User-visible title for the track, shown in the track selection menu.
    pub const LABEL: &str = "label";

    /// The `default` attribute.
    ///
    /// Boolean indicating this track should be enabled by default unless
    /// user preferences indicate otherwise.
    pub const DEFAULT: &str = "default";
}

/// Attribute names for iframe (`<iframe>`) elements.
///
/// # Purpose
/// The `<iframe>` element embeds another HTML document within the current page,
/// commonly used for third-party widgets, embedded content, or sandboxed applications.
///
/// # Common Attributes
/// - `src`: URL of the document to embed
/// - `width`/`height`: Dimensions of the iframe
/// - `sandbox`: Security restrictions for the embedded content
/// - `loading`: Lazy loading behavior
/// - `allow`: Feature policy permissions
///
/// # Example
/// ```html
/// <iframe src="https://example.com/widget" width="600" height="400"
///         sandbox="allow-scripts allow-same-origin"
///         loading="lazy"></iframe>
/// <iframe src="untrusted.html" sandbox></iframe>
/// <iframe srcdoc="<p>Inline HTML content</p>" sandbox="allow-scripts"></iframe>
/// ```
///
/// # WHATWG Specification
/// - [The `iframe` element](https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-iframe-element)
pub mod iframe {
    /// The `src` attribute.
    ///
    /// URL of the page to embed in the iframe.
    pub const SRC: &str = "src";

    /// The `srcdoc` attribute.
    ///
    /// Inline HTML content to display in the iframe (overrides `src`).
    pub const SRCDOC: &str = "srcdoc";

    /// The `name` attribute.
    ///
    /// Name of the browsing context, can be used as target for links/forms.
    pub const NAME: &str = "name";

    /// The `width` attribute.
    ///
    /// Width of the iframe in pixels.
    pub const WIDTH: &str = "width";

    /// The `height` attribute.
    ///
    /// Height of the iframe in pixels.
    pub const HEIGHT: &str = "height";

    /// The `loading` attribute.
    ///
    /// Loading behavior: "eager" (immediate) or "lazy" (when near viewport).
    pub const LOADING: &str = "loading";

    /// The `sandbox` attribute.
    ///
    /// Space-separated security restrictions. Empty value applies all restrictions.
    /// Values: "allow-forms", "allow-scripts", "allow-same-origin", etc.
    pub const SANDBOX: &str = "sandbox";

    /// The `allow` attribute.
    ///
    /// Feature policy for the iframe (e.g., "camera; microphone; geolocation").
    pub const ALLOW: &str = "allow";

    /// The `referrerpolicy` attribute.
    ///
    /// Controls referrer information sent when loading the iframe.
    pub const REFERRERPOLICY: &str = "referrerpolicy";
}

/// Attribute names for meta (`<meta>`) elements.
///
/// # Purpose
/// The `<meta>` element represents metadata that cannot be represented by
/// other HTML meta elements, such as character encoding, viewport settings,
/// and document metadata.
///
/// # Common Attributes
/// - `charset`: Character encoding declaration (e.g., "utf-8")
/// - `name`: Metadata name (e.g., "viewport", "description", "keywords")
/// - `content`: Metadata value (used with `name` or `http-equiv`)
/// - `http-equiv`: HTTP header to emulate
///
/// # Example
/// ```html
/// <meta charset="utf-8">
/// <meta name="viewport" content="width=device-width, initial-scale=1">
/// <meta name="description" content="Page description for SEO">
/// <meta name="keywords" content="html, css, javascript">
/// <meta http-equiv="X-UA-Compatible" content="IE=edge">
/// ```
///
/// # WHATWG Specification
/// - [The `meta` element](https://html.spec.whatwg.org/multipage/semantics.html#the-meta-element)
pub mod meta {
    /// The `charset` attribute.
    ///
    /// Character encoding declaration for the document. Should be "utf-8" in
    /// modern HTML documents.
    pub const CHARSET: &str = "charset";

    /// The `name` attribute.
    ///
    /// Name of the metadata property (e.g., "viewport", "description", "author",
    /// "keywords", "theme-color").
    pub const NAME: &str = "name";

    /// The `content` attribute.
    ///
    /// Value of the metadata property specified by `name` or `http-equiv`.
    pub const CONTENT: &str = "content";

    /// The `http-equiv` attribute.
    ///
    /// Pragma directive, emulating an HTTP response header (e.g., "refresh",
    /// "X-UA-Compatible", "Content-Security-Policy").
    pub const HTTPEQUIV: &str = "http-equiv";
}

/// Attribute names for link (`<link>`) elements.
///
/// # Purpose
/// The `<link>` element specifies relationships between the current document
/// and external resources, most commonly used to link stylesheets, icons,
/// and web app manifests.
///
/// # Common Attributes
/// - `href`: URL of the linked resource
/// - `rel`: Relationship type (stylesheet, icon, preload, etc.)
/// - `type`: MIME type hint for the resource
/// - `media`: Media query for when the resource applies
/// - `integrity`: Subresource integrity hash for security
///
/// # Example
/// ```html
/// <link rel="stylesheet" href="styles.css">
/// <link rel="icon" href="favicon.ico" type="image/x-icon">
/// <link rel="preload" href="font.woff2" as="font" type="font/woff2" crossorigin>
/// <link rel="manifest" href="manifest.json">
/// <link rel="stylesheet" href="print.css" media="print">
/// ```
///
/// # WHATWG Specification
/// - [The `link` element](https://html.spec.whatwg.org/multipage/semantics.html#the-link-element)
pub mod link {
    /// The `href` attribute.
    ///
    /// URL of the linked resource.
    pub const HREF: &str = "href";

    /// The `rel` attribute.
    ///
    /// Relationship type: "stylesheet", "icon", "preload", "prefetch",
    /// "dns-prefetch", "manifest", "alternate", etc.
    pub const REL: &str = "rel";

    /// The `type` attribute.
    ///
    /// MIME type hint for the linked resource (e.g., "text/css", "image/png").
    pub const TYPE: &str = "type";

    /// The `media` attribute.
    ///
    /// Media query specifying when the resource applies (e.g., "print",
    /// "(max-width: 600px)").
    pub const MEDIA: &str = "media";

    /// The `crossorigin` attribute.
    ///
    /// CORS settings for fetching the resource: "anonymous" or "use-credentials".
    pub const CROSSORIGIN: &str = "crossorigin";

    /// The `integrity` attribute.
    ///
    /// Subresource Integrity (SRI) hash for verifying resource integrity.
    pub const INTEGRITY: &str = "integrity";

    /// The `referrerpolicy` attribute.
    ///
    /// Controls referrer information sent when fetching the resource.
    pub const REFERRERPOLICY: &str = "referrerpolicy";

    /// The `sizes` attribute.
    ///
    /// For `rel="icon"`: icon sizes (e.g., "16x16", "32x32", "any").
    pub const SIZES: &str = "sizes";

    /// The `as` attribute.
    ///
    /// For `rel="preload"`: type of content being loaded (e.g., "script",
    /// "style", "font", "image").
    pub const AS: &str = "as";
}

/// Attribute names for script (`<script>`) elements.
///
/// # Purpose
/// The `<script>` element embeds or references executable JavaScript code,
/// enabling client-side scripting and dynamic behavior in web pages.
///
/// # Common Attributes
/// - `src`: URL of external script file
/// - `type`: MIME type or module type ("text/javascript", "module")
/// - `async`: Load and execute asynchronously
/// - `defer`: Defer execution until document is parsed
/// - `integrity`: Subresource integrity hash for security
/// - `crossorigin`: CORS settings for external scripts
///
/// # Example
/// ```html
/// <script src="script.js"></script>
/// <script src="script.js" defer></script>
/// <script src="script.js" async></script>
/// <script type="module" src="app.js"></script>
/// <script src="https://cdn.example.com/lib.js"
///         integrity="sha384-..."
///         crossorigin="anonymous"></script>
/// <script>
///   console.log('Inline JavaScript');
/// </script>
/// ```
///
/// # WHATWG Specification
/// - [The `script` element](https://html.spec.whatwg.org/multipage/scripting.html#the-script-element)
pub mod script {
    /// The `src` attribute.
    ///
    /// URL of an external script file. If present, element content is ignored.
    pub const SRC: &str = "src";

    /// The `type` attribute.
    ///
    /// MIME type of the script. Use "module" for ES6 modules. Default is
    /// JavaScript if omitted.
    pub const TYPE: &str = "type";

    /// The `async` attribute.
    ///
    /// Boolean to load and execute the script asynchronously. Script executes
    /// as soon as it's available, potentially before DOM is ready.
    pub const ASYNC: &str = "async";

    /// The `defer` attribute.
    ///
    /// Boolean to defer script execution until after document parsing. Scripts
    /// execute in order. Ignored for inline scripts.
    pub const DEFER: &str = "defer";

    /// The `crossorigin` attribute.
    ///
    /// CORS settings for fetching external scripts: "anonymous" or "use-credentials".
    pub const CROSSORIGIN: &str = "crossorigin";

    /// The `integrity` attribute.
    ///
    /// Subresource Integrity (SRI) hash for verifying script integrity.
    pub const INTEGRITY: &str = "integrity";

    /// The `nomodule` attribute.
    ///
    /// Boolean to prevent execution in browsers that support ES6 modules.
    /// Used for legacy fallback scripts.
    pub const NOMODULE: &str = "nomodule";

    /// The `referrerpolicy` attribute.
    ///
    /// Controls referrer information sent when fetching the script.
    pub const REFERRERPOLICY: &str = "referrerpolicy";
}

/// Attribute names for style (`<style>`) elements.
///
/// # Purpose
/// The `<style>` element contains CSS style information for the document,
/// allowing inline stylesheet definitions within HTML.
///
/// # Common Attributes
/// - `media`: Media query for when the styles apply
///
/// # Example
/// ```html
/// <style>
///   body { font-family: sans-serif; }
///   .container { max-width: 1200px; margin: 0 auto; }
/// </style>
/// <style media="print">
///   body { font-size: 12pt; }
///   .no-print { display: none; }
/// </style>
/// <style media="(max-width: 600px)">
///   .mobile-hidden { display: none; }
/// </style>
/// ```
///
/// # WHATWG Specification
/// - [The `style` element](https://html.spec.whatwg.org/multipage/semantics.html#the-style-element)
pub mod style {
    /// The `media` attribute.
    ///
    /// Media query specifying when the styles should apply (e.g., "print",
    /// "screen", "(max-width: 600px)").
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
