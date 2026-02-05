//! # html-elements
//!
//! Type-safe HTML5 elements following the
//! [WHATWG HTML Living Standard](https://html.spec.whatwg.org/).
//!
//! This crate provides zero-sized types for all HTML5 elements with traits
//! for content categories, enabling compile-time validation of HTML structure.
//!
//! ## Example
//!
//! ```rust
//! use html_elements::{Div, Span, A, Img, HtmlElement};
//! use html_elements::{FlowContent, PhrasingContent, EmbeddedContent};
//!
//! // Access element tag names
//! assert_eq!(Div::TAG, "div");
//! assert_eq!(A::TAG, "a");
//!
//! // Check if an element is void (self-closing)
//! assert!(!Div::VOID);  // <div></div>
//! assert!(Img::VOID);   // <img />
//!
//! // Use traits to constrain valid element usage
//! fn accepts_flow<T: FlowContent>() {}
//! fn accepts_phrasing<T: PhrasingContent>() {}
//!
//! accepts_flow::<Div>();      // OK - div is flow content
//! accepts_flow::<Span>();     // OK - span is flow content
//! accepts_phrasing::<Span>(); // OK - span is phrasing content
//! // accepts_phrasing::<Div>(); // ERROR - div is not phrasing content
//! ```
//!
//! ## Specification References
//!
//! - [Content categories](https://html.spec.whatwg.org/multipage/dom.html#content-models)
//! - [Element index](https://html.spec.whatwg.org/multipage/indices.html#elements-3)
//! - [Void elements](https://html.spec.whatwg.org/multipage/syntax.html#void-elements)
//!
//! ## Design Decisions
//!
//! ### Zero-Sized Types (ZSTs) for Elements
//!
//! Each HTML element is defined as a unit struct with no fields.
//! This design provides zero runtime overhead - ZSTs occupy no memory:
//!
//! ```rust
//! use html_elements::{Div, Span, A};
//! use core::mem::size_of;
//!
//! // All element types are zero-sized
//! assert_eq!(size_of::<Div>(), 0);
//! assert_eq!(size_of::<Span>(), 0);
//! assert_eq!(size_of::<A>(), 0);
//! ```
//!
//! Benefits:
//!
//! - **Zero runtime overhead**: ZSTs occupy no memory at runtime. The type
//!   exists only at compile time for type checking.
//! - **Compile-time markers**: Elements serve as type-level tags that carry
//!   semantic meaning without any runtime representation.
//! - **Optimal codegen**: The Rust compiler can completely eliminate ZSTs
//!   during optimization, resulting in no additional instructions.
//!
//! ### Traits for Content Categories
//!
//! Content categories from the WHATWG spec are modeled as marker traits
//! with a hierarchy that reflects the specification:
//!
//! ```rust
//! use html_elements::{Div, Span, Article, H1};
//! use html_elements::{FlowContent, PhrasingContent, SectioningContent, HeadingContent};
//!
//! // Generic functions can require specific content categories
//! fn accepts_flow<T: FlowContent>() {}
//! fn accepts_phrasing<T: PhrasingContent>() {}
//! fn accepts_sectioning<T: SectioningContent>() {}
//!
//! // Div is flow content
//! accepts_flow::<Div>();
//!
//! // Span is phrasing content (and therefore also flow content)
//! accepts_phrasing::<Span>();
//! accepts_flow::<Span>();
//!
//! // Article is sectioning content
//! accepts_sectioning::<Article>();
//! ```
//!
//! This design enables:
//!
//! - **Compile-time constraint checking**: Generic functions can require
//!   elements to belong to specific content categories.
//! - **Trait hierarchy reflects spec**: The supertrait relationships
//!   (e.g., `PhrasingContent: FlowContent`) mirror the WHATWG specification
//!   where all phrasing content is also flow content.
//! - **Extensibility**: New element types can implement these traits to
//!   integrate with existing validation logic.
//!
//! ### The CanContain Pattern
//!
//! Parent-child relationships use a binary trait pattern:
//!
//! ```rust
//! use html_elements::{Div, Span, Ul, Li, Table, Tr, Td, P, CanContain};
//!
//! // Check valid parent-child relationships at compile time
//! fn can_nest<Parent: CanContain<Child>, Child>() {}
//!
//! // Lists: Ul can contain Li
//! can_nest::<Ul, Li>();
//!
//! // Tables: Table contains Tr, Tr contains Td
//! can_nest::<Tr, Td>();
//!
//! // Div can contain any flow content
//! can_nest::<Div, P>();
//! can_nest::<Div, Span>();
//! can_nest::<Div, Ul>();
//!
//! // Span can contain phrasing content
//! can_nest::<Span, Span>();
//! ```
//!
//! This design provides:
//!
//! - **Compile-time validation**: Invalid nesting like `<ul><div>` or
//!   `<p><div>` produces a compilation error rather than a runtime error.
//! - **Precise control**: Each parent-child relationship is explicitly
//!   declared, matching the WHATWG content model specification.
//! - **Generic implementations**: Using trait bounds like `impl<T: FlowContent>`
//!   avoids listing every valid child element individually while still
//!   providing type safety.
//!
//! ### Why This Approach?
//!
//! Traditional HTML builders use runtime validation or stringly-typed APIs
//! where errors are only discovered at runtime or not at all. For example,
//! `builder.element("div").child("invalid")` might error at runtime, or
//! invalid nesting like putting a `<li>` directly in a `<div>` might silently
//! produce malformed HTML.
//!
//! This crate instead leverages Rust's type system to make invalid HTML
//! structures impossible to represent. The compiler becomes your HTML
//! validator, catching errors before your code even runs.

#![no_std]

extern crate alloc;

// =============================================================================
// Content Categories (per WHATWG HTML Living Standard)
// https://html.spec.whatwg.org/multipage/dom.html#content-models
// =============================================================================

/// Metadata content: elements that set up the presentation or behavior of
/// the rest of the content, or set up relationships with other documents.
pub trait MetadataContent {}

/// Flow content: most elements used in the body of documents and applications.
pub trait FlowContent {}

/// Sectioning content: elements that define the scope of headings and footers.
pub trait SectioningContent: FlowContent {}

/// Heading content: elements that define the header of a section.
pub trait HeadingContent: FlowContent {}

/// Phrasing content: the text of the document and elements that mark up that text.
pub trait PhrasingContent: FlowContent {}

/// Embedded content: elements that import another resource into the document.
pub trait EmbeddedContent: PhrasingContent {}

/// Interactive content: elements specifically intended for user interaction.
pub trait InteractiveContent: FlowContent {}

/// Palpable content: content that is not empty or hidden.
pub trait PalpableContent {}

/// Script-supporting elements: elements that don't represent anything themselves.
pub trait ScriptSupporting {}

// =============================================================================
// Content Model Trait
// https://html.spec.whatwg.org/multipage/dom.html#content-models
// =============================================================================

/// Trait indicating that an element can contain another element as a child.
///
/// This trait enables compile-time validation of parent-child relationships
/// in HTML documents according to the WHATWG specification.
///
/// ## Example
///
/// ```rust
/// use html_elements::{CanContain, Div, Span, P, Ul, Li, Table, Tr, Td, Text};
///
/// // Check valid parent-child relationships
/// fn valid_child<Parent, Child>() where Parent: CanContain<Child> {}
///
/// valid_child::<Div, Span>();   // OK - div can contain span
/// valid_child::<Div, P>();      // OK - div can contain p
/// valid_child::<Ul, Li>();      // OK - ul can contain li
/// valid_child::<Tr, Td>();      // OK - tr can contain td
/// valid_child::<Div, Text>();   // OK - div can contain text
/// // valid_child::<P, Div>();   // ERROR - p cannot contain div (block in inline)
/// // valid_child::<Ul, Div>();  // ERROR - ul can only contain li
/// ```
pub trait CanContain<Child> {}

// =============================================================================
// Element Trait
// =============================================================================

/// Trait implemented by all HTML elements.
pub trait HtmlElement {
    /// The HTML tag name (e.g., "div", "span", "img").
    const TAG: &'static str;

    /// Whether this is a void element (self-closing, no children allowed).
    const VOID: bool = false;
}

// =============================================================================
// Text Node (special pseudo-element for content model)
// =============================================================================

/// Represents a text node in the DOM.
pub struct Text;

// =============================================================================
// Document Metadata Elements
// =============================================================================

/// The `<html>` element - the root element of an HTML document.
///
/// # Purpose
///
/// The `<html>` element represents the root (top-level element) of an HTML document.
/// All other elements must be descendants of this element. It establishes the document
/// as an HTML document and provides a container for the entire page content.
///
/// # Content Categories
///
/// - None (root element, not categorized)
///
/// # Permitted Content Model
///
/// - One `<head>` element followed by one `<body>` element.
///
/// # Common Use Cases
///
/// - Root container for every HTML document
/// - Container for setting document-wide attributes like `lang`
/// - Container for document-level metadata via `<head>`
///
/// # Key Attributes
///
/// - `lang`: Specifies the primary language of the document (e.g., `"en"`, `"es"`, `"fr"`)
/// - Global attributes
///
/// # Example
///
/// ```html
/// <!DOCTYPE html>
/// <html lang="en">
///   <head>
///     <meta charset="UTF-8">
///     <title>Document Title</title>
///   </head>
///   <body>
///     <h1>Hello, World!</h1>
///   </body>
/// </html>
/// ```
///
/// # WHATWG Specification
///
/// - [4.1.1 The html element](https://html.spec.whatwg.org/multipage/semantics.html#the-html-element)
pub struct Html;
impl HtmlElement for Html {
    const TAG: &'static str = "html";
}

/// The `<head>` element - container for document metadata.
///
/// # Purpose
///
/// The `<head>` element contains machine-readable metadata about the document,
/// including its title, scripts, stylesheets, and other meta information.
/// This content is not displayed to users but is essential for browsers,
/// search engines, and other services.
///
/// # Content Categories
///
/// - Metadata Content
///
/// # Permitted Content Model
///
/// - Zero or more metadata content elements (e.g., `<title>`, `<meta>`, `<link>`, `<style>`, `<script>`, `<base>`)
/// - Must include exactly one `<title>` element
///
/// # Common Use Cases
///
/// - Defining the document title
/// - Linking to stylesheets and scripts
/// - Specifying character encoding
/// - Adding SEO meta tags
/// - Providing social media metadata (Open Graph, Twitter Cards)
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <head>
///   <meta charset="UTF-8">
///   <meta name="viewport" content="width=device-width, initial-scale=1.0">
///   <title>My Web Page</title>
///   <link rel="stylesheet" href="styles.css">
///   <script src="app.js" defer></script>
/// </head>
/// ```
///
/// # WHATWG Specification
///
/// - [4.2.1 The head element](https://html.spec.whatwg.org/multipage/semantics.html#the-head-element)
pub struct Head;
impl HtmlElement for Head {
    const TAG: &'static str = "head";
}
impl MetadataContent for Head {}

/// The `<title>` element - document title.
///
/// # Purpose
///
/// The `<title>` element defines the title of the document, shown in the browser's
/// title bar or tab. It is also used by search engines as the page title in search
/// results and is important for SEO and accessibility.
///
/// # Content Categories
///
/// - Metadata Content
///
/// # Permitted Content Model
///
/// - Text content only (no child elements)
///
/// # Common Use Cases
///
/// - Setting the browser tab/window title
/// - Providing the page title for search engine results
/// - Defining the default bookmark name
/// - Displaying the page title when sharing on social media
///
/// # Key Attributes
///
/// - Global attributes only (rarely used)
///
/// # Example
///
/// ```html
/// <head>
///   <title>Introduction to HTML - Web Development Tutorial</title>
/// </head>
/// ```
///
/// # Accessibility
///
/// - The title is announced by screen readers when navigating to a page
/// - Should be descriptive and unique for each page
/// - Typically 50-60 characters for optimal display in search results
///
/// # WHATWG Specification
///
/// - [4.2.2 The title element](https://html.spec.whatwg.org/multipage/semantics.html#the-title-element)
pub struct Title;
impl HtmlElement for Title {
    const TAG: &'static str = "title";
}
impl MetadataContent for Title {}

/// The `<base>` element - document base URL.
///
/// # Purpose
///
/// The `<base>` element specifies the base URL to use for all relative URLs in a document.
/// There can be only one `<base>` element in a document, and it must be inside the `<head>` element.
///
/// # Content Categories
///
/// - Metadata Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Setting a base URL for all relative links in a document
/// - Specifying default target for all links
/// - Useful for single-page applications or sites with complex URL structures
///
/// # Key Attributes
///
/// - `href`: Base URL for relative URLs
/// - `target`: Default browsing context for links (e.g., `"_blank"`, `"_self"`)
///
/// # Example
///
/// ```html
/// <head>
///   <base href="https://example.com/" target="_blank">
///   <link rel="stylesheet" href="styles.css">
///   <!-- Resolves to https://example.com/styles.css -->
/// </head>
/// ```
///
/// # WHATWG Specification
///
/// - [4.2.3 The base element](https://html.spec.whatwg.org/multipage/semantics.html#the-base-element)
pub struct Base;
impl HtmlElement for Base {
    const TAG: &'static str = "base";
    const VOID: bool = true;
}
impl MetadataContent for Base {}

/// The `<link>` element - external resource link.
///
/// # Purpose
///
/// The `<link>` element specifies relationships between the current document and external resources.
/// Most commonly used to link to stylesheets, but also used for favicons, alternate versions,
/// preloading resources, and more.
///
/// # Content Categories
///
/// - Metadata Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Linking to external CSS stylesheets
/// - Specifying favicons and touch icons
/// - Defining alternate language versions of a page
/// - Preloading or prefetching resources
/// - Linking to RSS/Atom feeds
///
/// # Key Attributes
///
/// - `rel`: Relationship type (e.g., `"stylesheet"`, `"icon"`, `"preload"`, `"canonical"`)
/// - `href`: URL of the linked resource
/// - `type`: MIME type of the linked resource
/// - `media`: Media query for conditional loading (mainly for stylesheets)
/// - `as`: Type of content being preloaded (when `rel="preload"`)
///
/// # Example
///
/// ```html
/// <head>
///   <link rel="stylesheet" href="styles.css">
///   <link rel="icon" type="image/png" href="/favicon.png">
///   <link rel="preload" href="font.woff2" as="font" type="font/woff2" crossorigin>
///   <link rel="alternate" hreflang="es" href="https://example.com/es/">
/// </head>
/// ```
///
/// # WHATWG Specification
///
/// - [4.2.4 The link element](https://html.spec.whatwg.org/multipage/semantics.html#the-link-element)
pub struct Link;
impl HtmlElement for Link {
    const TAG: &'static str = "link";
    const VOID: bool = true;
}
impl MetadataContent for Link {}

/// The `<meta>` element - document metadata.
///
/// # Purpose
///
/// The `<meta>` element represents various kinds of metadata that cannot be represented
/// by other HTML meta-related elements (`<title>`, `<base>`, `<link>`, `<style>`).
/// Used for character encoding, viewport settings, SEO, and social media metadata.
///
/// # Content Categories
///
/// - Metadata Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Setting character encoding (`charset`)
/// - Configuring viewport for responsive design
/// - Providing page description for search engines
/// - Adding Open Graph tags for social media sharing
/// - Setting HTTP headers via `http-equiv`
///
/// # Key Attributes
///
/// - `charset`: Character encoding declaration (e.g., `"UTF-8"`)
/// - `name`: Metadata name (e.g., `"viewport"`, `"description"`, `"author"`)
/// - `content`: Metadata value (used with `name` or `http-equiv`)
/// - `http-equiv`: Pragma directive (e.g., `"refresh"`, `"content-security-policy"`)
///
/// # Example
///
/// ```html
/// <head>
///   <meta charset="UTF-8">
///   <meta name="viewport" content="width=device-width, initial-scale=1.0">
///   <meta name="description" content="A comprehensive guide to HTML5">
///   <meta property="og:title" content="HTML5 Guide">
///   <meta property="og:image" content="https://example.com/image.png">
/// </head>
/// ```
///
/// # WHATWG Specification
///
/// - [4.2.5 The meta element](https://html.spec.whatwg.org/multipage/semantics.html#the-meta-element)
pub struct Meta;
impl HtmlElement for Meta {
    const TAG: &'static str = "meta";
    const VOID: bool = true;
}
impl MetadataContent for Meta {}

/// The `<style>` element - embedded CSS styles.
///
/// # Purpose
///
/// The `<style>` element contains CSS style information for the document.
/// It allows you to embed styles directly in HTML without an external stylesheet.
///
/// # Content Categories
///
/// - Metadata Content
///
/// # Permitted Content Model
///
/// - Text content representing CSS rules
///
/// # Common Use Cases
///
/// - Embedding critical CSS for performance optimization
/// - Adding page-specific styles without a separate file
/// - Inline styles for email templates
/// - Dynamic styling that changes based on server-side conditions
///
/// # Key Attributes
///
/// - `media`: Media query for conditional application of styles
/// - `type`: MIME type (defaults to `"text/css"`, usually omitted)
/// - `nonce`: Cryptographic nonce for Content Security Policy
///
/// # Example
///
/// ```html
/// <head>
///   <style>
///     body {
///       font-family: sans-serif;
///       margin: 0;
///     }
///     .container {
///       max-width: 1200px;
///       margin: 0 auto;
///     }
///   </style>
///   <style media="print">
///     .no-print { display: none; }
///   </style>
/// </head>
/// ```
///
/// # WHATWG Specification
///
/// - [4.2.6 The style element](https://html.spec.whatwg.org/multipage/semantics.html#the-style-element)
pub struct Style;
impl HtmlElement for Style {
    const TAG: &'static str = "style";
}
impl MetadataContent for Style {}

// =============================================================================
// Sectioning Root
// =============================================================================

/// The `<body>` element - document body.
///
/// # Purpose
///
/// The `<body>` element represents the main content of an HTML document.
/// There can be only one `<body>` element per document, and it contains all
/// the visible content that is displayed to users.
///
/// # Content Categories
///
/// - Sectioning Root
///
/// # Permitted Content Model
///
/// - Flow content (most visible HTML elements)
///
/// # Common Use Cases
///
/// - Container for all visible page content
/// - Structuring the main layout of a webpage
/// - Applying document-wide styles via CSS
/// - Attaching document-level event handlers
///
/// # Key Attributes
///
/// - Event handler attributes (`onload`, `onunload`, `onbeforeunload`, etc.)
/// - Global attributes
///
/// # Example
///
/// ```html
/// <body>
///   <header>
///     <h1>My Website</h1>
///     <nav>...</nav>
///   </header>
///   <main>
///     <article>...</article>
///   </main>
///   <footer>
///     <p>&copy; 2024 My Website</p>
///   </footer>
/// </body>
/// ```
///
/// # WHATWG Specification
///
/// - [4.3.1 The body element](https://html.spec.whatwg.org/multipage/sections.html#the-body-element)
pub struct Body;
impl HtmlElement for Body {
    const TAG: &'static str = "body";
}

// =============================================================================
// Content Sectioning Elements
// =============================================================================

/// The `<article>` element - self-contained composition.
///
/// # Purpose
///
/// The `<article>` element represents a self-contained composition in a document,
/// page, application, or site, which is intended to be independently distributable
/// or reusable (e.g., in syndication). Examples include blog posts, news articles,
/// forum posts, product cards, user comments, and interactive widgets.
///
/// # Content Categories
///
/// - Flow Content
/// - Sectioning Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Blog posts and news articles
/// - Forum or comment posts
/// - Product cards in e-commerce sites
/// - Independently distributable widgets
/// - User-submitted content
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <article>
///   <header>
///     <h2>Understanding HTML5 Semantics</h2>
///     <p>Posted on <time datetime="2024-01-15">January 15, 2024</time></p>
///   </header>
///   <p>Semantic HTML elements provide meaning to web content...</p>
///   <footer>
///     <p>Written by Jane Doe</p>
///   </footer>
/// </article>
/// ```
///
/// # Accessibility
///
/// - Screen readers may announce article boundaries
/// - Consider using `<h1>`-`<h6>` for article headings
/// - Each article should be independently understandable
///
/// # WHATWG Specification
///
/// - [4.3.2 The article element](https://html.spec.whatwg.org/multipage/sections.html#the-article-element)
pub struct Article;
impl HtmlElement for Article {
    const TAG: &'static str = "article";
}
impl FlowContent for Article {}
impl SectioningContent for Article {}
impl PalpableContent for Article {}

/// The `<section>` element - thematic grouping of content.
///
/// # Purpose
///
/// The `<section>` element represents a generic standalone section of a document,
/// which doesn't have a more specific semantic element to represent it. Sections
/// should typically have a heading, and they group related content thematically.
///
/// # Content Categories
///
/// - Flow Content
/// - Sectioning Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Thematic grouping of content (chapters, tabs in a tabbed interface)
/// - Different sections of an article or document
/// - Grouping related content with a heading
/// - Creating document outlines with hierarchical sections
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <article>
///   <h1>The Complete Guide to Web Development</h1>
///   <section>
///     <h2>Introduction</h2>
///     <p>Web development encompasses...</p>
///   </section>
///   <section>
///     <h2>Frontend Technologies</h2>
///     <p>HTML, CSS, and JavaScript form...</p>
///   </section>
///   <section>
///     <h2>Backend Technologies</h2>
///     <p>Server-side programming...</p>
///   </section>
/// </article>
/// ```
///
/// # Accessibility
///
/// - Each section should have a heading for proper document outline
/// - Screen readers use sections to navigate content structure
///
/// # WHATWG Specification
///
/// - [4.3.3 The section element](https://html.spec.whatwg.org/multipage/sections.html#the-section-element)
pub struct Section;
impl HtmlElement for Section {
    const TAG: &'static str = "section";
}
impl FlowContent for Section {}
impl SectioningContent for Section {}
impl PalpableContent for Section {}

/// The `<nav>` element - navigation links.
///
/// # Purpose
///
/// The `<nav>` element represents a section of a page that contains navigation links,
/// either within the current document or to other documents. Not all groups of links
/// need to be in a `<nav>` element—only sections that consist of major navigation blocks.
///
/// # Content Categories
///
/// - Flow Content
/// - Sectioning Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content (but not `<main>` element)
///
/// # Common Use Cases
///
/// - Primary site navigation (header menu)
/// - Table of contents
/// - Pagination controls
/// - Breadcrumb navigation
/// - Footer site links
///
/// # Key Attributes
///
/// - Global attributes only
/// - `aria-label`: Distinguishes multiple nav elements on same page
///
/// # Example
///
/// ```html
/// <nav aria-label="Main navigation">
///   <ul>
///     <li><a href="/">Home</a></li>
///     <li><a href="/about">About</a></li>
///     <li><a href="/products">Products</a></li>
///     <li><a href="/contact">Contact</a></li>
///   </ul>
/// </nav>
/// ```
///
/// # Accessibility
///
/// - Screen readers provide shortcuts to navigate to `<nav>` landmarks
/// - Use `aria-label` or `aria-labelledby` when multiple navs exist
/// - Not every group of links needs to be in `<nav>`
///
/// # WHATWG Specification
///
/// - [4.3.4 The nav element](https://html.spec.whatwg.org/multipage/sections.html#the-nav-element)
pub struct Nav;
impl HtmlElement for Nav {
    const TAG: &'static str = "nav";
}
impl FlowContent for Nav {}
impl SectioningContent for Nav {}
impl PalpableContent for Nav {}

/// The `<aside>` element - tangentially related content.
///
/// # Purpose
///
/// The `<aside>` element represents content that is tangentially related to the
/// content around it, which could be considered separate from that content.
/// Such sections are often represented as sidebars or call-out boxes.
///
/// # Content Categories
///
/// - Flow Content
/// - Sectioning Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Sidebars with related links or content
/// - Pull quotes or callouts
/// - Advertising
/// - Groups of nav elements (e.g., blogrolls, secondary navigation)
/// - Related articles or "You might also like" sections
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <article>
///   <h1>Climate Change Impact</h1>
///   <p>Recent studies show...</p>
///   <aside>
///     <h2>Related Articles</h2>
///     <ul>
///       <li><a href="/renewable-energy">Renewable Energy</a></li>
///       <li><a href="/carbon-footprint">Carbon Footprint</a></li>
///     </ul>
///   </aside>
/// </article>
/// ```
///
/// # Accessibility
///
/// - Screen readers identify `<aside>` as a complementary landmark
/// - Content should be understandable even if aside is removed
///
/// # WHATWG Specification
///
/// - [4.3.5 The aside element](https://html.spec.whatwg.org/multipage/sections.html#the-aside-element)
pub struct Aside;
impl HtmlElement for Aside {
    const TAG: &'static str = "aside";
}
impl FlowContent for Aside {}
impl SectioningContent for Aside {}
impl PalpableContent for Aside {}

/// The `<h1>` element - level 1 heading.
///
/// # Purpose
///
/// The `<h1>` element represents the highest level (most important) heading in a document.
/// It typically represents the main title or subject of the page or section. There should
/// generally be only one `<h1>` per page for SEO and accessibility best practices.
///
/// # Content Categories
///
/// - Flow Content
/// - Heading Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Main page title
/// - Primary heading for the document
/// - Top-level section heading
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <article>
///   <h1>Introduction to Web Accessibility</h1>
///   <p>Web accessibility ensures that websites...</p>
/// </article>
/// ```
///
/// # Accessibility
///
/// - Screen readers use headings for navigation
/// - Should have only one `<h1>` per page
/// - Creates the document outline structure
///
/// # WHATWG Specification
///
/// - [4.3.6 The h1-h6 elements](https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
pub struct H1;
impl HtmlElement for H1 {
    const TAG: &'static str = "h1";
}
impl FlowContent for H1 {}
impl HeadingContent for H1 {}
impl PalpableContent for H1 {}

/// The `<h2>` element - level 2 heading.
///
/// # Purpose
///
/// The `<h2>` element represents a second-level heading, typically used for major
/// sections within a document. It's subordinate to `<h1>` and superior to `<h3>`.
///
/// # Content Categories
///
/// - Flow Content
/// - Heading Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Major section headings
/// - Chapter titles in longer documents
/// - Main subsections under the `<h1>` heading
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <h1>Complete Guide to HTML</h1>
/// <h2>Basic Elements</h2>
/// <p>HTML provides various basic elements...</p>
/// <h2>Semantic Elements</h2>
/// <p>Semantic elements give meaning...</p>
/// ```
///
/// # Accessibility
///
/// - Creates hierarchical document structure
/// - Should follow heading order (don't skip levels)
/// - Screen readers use for navigation
///
/// # WHATWG Specification
///
/// - [4.3.6 The h1-h6 elements](https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
pub struct H2;
impl HtmlElement for H2 {
    const TAG: &'static str = "h2";
}
impl FlowContent for H2 {}
impl HeadingContent for H2 {}
impl PalpableContent for H2 {}

/// The `<h3>` element - level 3 heading.
///
/// # Purpose
///
/// The `<h3>` element represents a third-level heading, used for subsections
/// within `<h2>` sections. Part of the hierarchical heading structure.
///
/// # Content Categories
///
/// - Flow Content
/// - Heading Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Subsection headings within `<h2>` sections
/// - Detailed topic divisions
/// - Third-level document structure
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <h2>Frontend Development</h2>
/// <h3>HTML</h3>
/// <p>HTML provides structure...</p>
/// <h3>CSS</h3>
/// <p>CSS handles styling...</p>
/// <h3>JavaScript</h3>
/// <p>JavaScript adds interactivity...</p>
/// ```
///
/// # Accessibility
///
/// - Maintains document outline hierarchy
/// - Should not skip heading levels
/// - Helps screen reader users navigate
///
/// # WHATWG Specification
///
/// - [4.3.6 The h1-h6 elements](https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
pub struct H3;
impl HtmlElement for H3 {
    const TAG: &'static str = "h3";
}
impl FlowContent for H3 {}
impl HeadingContent for H3 {}
impl PalpableContent for H3 {}

/// The `<h4>` element - level 4 heading.
///
/// # Purpose
///
/// The `<h4>` element represents a fourth-level heading, used for subsections
/// within `<h3>` sections. Part of the hierarchical heading structure.
///
/// # Content Categories
///
/// - Flow Content
/// - Heading Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Sub-subsection headings
/// - Detailed breakdowns within `<h3>` sections
/// - Fine-grained document structure
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <h3>JavaScript Basics</h3>
/// <h4>Variables</h4>
/// <p>Variables store data...</p>
/// <h4>Functions</h4>
/// <p>Functions are reusable blocks...</p>
/// ```
///
/// # Accessibility
///
/// - Part of document outline
/// - Helps organize complex content
/// - Screen readers use for navigation
///
/// # WHATWG Specification
///
/// - [4.3.6 The h1-h6 elements](https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
pub struct H4;
impl HtmlElement for H4 {
    const TAG: &'static str = "h4";
}
impl FlowContent for H4 {}
impl HeadingContent for H4 {}
impl PalpableContent for H4 {}

/// The `<h5>` element - level 5 heading.
///
/// # Purpose
///
/// The `<h5>` element represents a fifth-level heading, used for subsections
/// within `<h4>` sections. Part of the hierarchical heading structure.
///
/// # Content Categories
///
/// - Flow Content
/// - Heading Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Deep subsection headings
/// - Detailed documentation sections
/// - Complex hierarchical content
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <h4>Array Methods</h4>
/// <h5>Mutating Methods</h5>
/// <p>Methods that modify the original array...</p>
/// <h5>Non-Mutating Methods</h5>
/// <p>Methods that return a new array...</p>
/// ```
///
/// # Accessibility
///
/// - Rarely needed in typical documents
/// - Maintains document outline
/// - Use only when deep hierarchy is needed
///
/// # WHATWG Specification
///
/// - [4.3.6 The h1-h6 elements](https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
pub struct H5;
impl HtmlElement for H5 {
    const TAG: &'static str = "h5";
}
impl FlowContent for H5 {}
impl HeadingContent for H5 {}
impl PalpableContent for H5 {}

/// The `<h6>` element - level 6 heading.
///
/// # Purpose
///
/// The `<h6>` element represents the lowest level (sixth-level) heading,
/// used for the finest grain of subsections. Rarely used in practice.
///
/// # Content Categories
///
/// - Flow Content
/// - Heading Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Very deep subsection headings
/// - Highly detailed technical documentation
/// - Complex nested content structures
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <h5>Specific Use Cases</h5>
/// <h6>Edge Case 1</h6>
/// <p>When dealing with legacy browsers...</p>
/// <h6>Edge Case 2</h6>
/// <p>In certain mobile scenarios...</p>
/// ```
///
/// # Accessibility
///
/// - Least commonly used heading level
/// - Consider if such deep hierarchy is necessary
/// - Screen readers support all heading levels
///
/// # WHATWG Specification
///
/// - [4.3.6 The h1-h6 elements](https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
pub struct H6;
impl HtmlElement for H6 {
    const TAG: &'static str = "h6";
}
impl FlowContent for H6 {}
impl HeadingContent for H6 {}
impl PalpableContent for H6 {}

/// The `<hgroup>` element - heading group.
///
/// # Purpose
///
/// The `<hgroup>` element represents a heading and related content (typically subheadings,
/// alternative titles, or taglines). It groups a heading with secondary content like
/// subtitles or taglines, treating them as a single heading in the document outline.
///
/// # Content Categories
///
/// - Flow Content
/// - Heading Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - One or more `<h1>`-`<h6>` elements, optionally intermixed with `<p>` elements
///
/// # Common Use Cases
///
/// - Grouping a title with a subtitle
/// - Combining a heading with a tagline
/// - Multi-line headings with different semantic levels
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <hgroup>
///   <h1>HTML5 Elements Reference</h1>
///   <p>A comprehensive guide to semantic HTML</p>
/// </hgroup>
/// ```
///
/// # Accessibility
///
/// - Screen readers treat the group as a single heading
/// - Only the highest-ranked heading affects document outline
///
/// # WHATWG Specification
///
/// - [4.3.7 The hgroup element](https://html.spec.whatwg.org/multipage/sections.html#the-hgroup-element)
pub struct Hgroup;
impl HtmlElement for Hgroup {
    const TAG: &'static str = "hgroup";
}
impl FlowContent for Hgroup {}
impl HeadingContent for Hgroup {}
impl PalpableContent for Hgroup {}

/// The `<header>` element - introductory content.
///
/// # Purpose
///
/// The `<header>` element represents introductory content or navigational aids.
/// It typically contains heading elements, logos, search forms, author information,
/// or navigation. A page can have multiple `<header>` elements (e.g., page header
/// and article headers).
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content (but not `<header>`, `<footer>`, or `<main>` descendants)
///
/// # Common Use Cases
///
/// - Site-wide page header with logo and navigation
/// - Article or section headers with title and metadata
/// - Introduction to a piece of content
/// - Masthead for a page or section
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <header>
///   <img src="logo.png" alt="Company Logo">
///   <h1>My Website</h1>
///   <nav>
///     <a href="/">Home</a>
///     <a href="/about">About</a>
///   </nav>
/// </header>
///
/// <article>
///   <header>
///     <h2>Article Title</h2>
///     <p>By John Doe, <time datetime="2024-01-15">Jan 15, 2024</time></p>
///   </header>
///   <p>Article content...</p>
/// </article>
/// ```
///
/// # Accessibility
///
/// - Screen readers may identify headers as landmarks
/// - Can contain navigation for the section it introduces
///
/// # WHATWG Specification
///
/// - [4.3.8 The header element](https://html.spec.whatwg.org/multipage/sections.html#the-header-element)
pub struct Header;
impl HtmlElement for Header {
    const TAG: &'static str = "header";
}
impl FlowContent for Header {}
impl PalpableContent for Header {}

/// The `<footer>` element - footer content.
///
/// # Purpose
///
/// The `<footer>` element represents footer content for its nearest ancestor sectioning
/// content or sectioning root. Typically contains information about the author, copyright,
/// related links, or other metadata. A page can have multiple `<footer>` elements.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content (but not `<header>`, `<footer>`, or `<main>` descendants)
///
/// # Common Use Cases
///
/// - Site-wide footer with copyright and links
/// - Article or section footers with metadata
/// - Author information and related content
/// - Contact information and social links
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <footer>
///   <p>&copy; 2024 My Company. All rights reserved.</p>
///   <nav>
///     <a href="/privacy">Privacy Policy</a>
///     <a href="/terms">Terms of Service</a>
///   </nav>
/// </footer>
///
/// <article>
///   <h1>Blog Post</h1>
///   <p>Content...</p>
///   <footer>
///     <p>Posted by <a href="/author">Jane Doe</a></p>
///     <p>Tags: <a href="/tag/html">HTML</a>, <a href="/tag/web">Web</a></p>
///   </footer>
/// </article>
/// ```
///
/// # Accessibility
///
/// - Screen readers may identify footers as landmarks
/// - Often contains important navigation and legal information
///
/// # WHATWG Specification
///
/// - [4.3.9 The footer element](https://html.spec.whatwg.org/multipage/sections.html#the-footer-element)
pub struct Footer;
impl HtmlElement for Footer {
    const TAG: &'static str = "footer";
}
impl FlowContent for Footer {}
impl PalpableContent for Footer {}

/// The `<address>` element - contact information.
///
/// # Purpose
///
/// The `<address>` element represents contact information for its nearest `<article>` or
/// `<body>` ancestor. This could be physical address, email, phone, social media, or any
/// contact method. It should not be used for arbitrary addresses.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content (but no heading content, sectioning content, `<header>`, `<footer>`, or `<address>` descendants)
///
/// # Common Use Cases
///
/// - Author contact information
/// - Business contact details
/// - Article author information
/// - Organization contact info in page footer
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <footer>
///   <address>
///     <p>Contact us:</p>
///     <p>Email: <a href="mailto:info@example.com">info@example.com</a></p>
///     <p>Phone: <a href="tel:+15551234567">+1 (555) 123-4567</a></p>
///     <p>123 Main St, City, State 12345</p>
///   </address>
/// </footer>
///
/// <article>
///   <h1>Article Title</h1>
///   <p>Content...</p>
///   <footer>
///     <address>
///       Written by <a href="mailto:author@example.com">Jane Doe</a>
///     </address>
///   </footer>
/// </article>
/// ```
///
/// # Accessibility
///
/// - Screen readers may announce address regions
/// - Makes contact information easily discoverable
///
/// # WHATWG Specification
///
/// - [4.3.10 The address element](https://html.spec.whatwg.org/multipage/sections.html#the-address-element)
pub struct Address;
impl HtmlElement for Address {
    const TAG: &'static str = "address";
}
impl FlowContent for Address {}
impl PalpableContent for Address {}

/// The `<main>` element - main content.
///
/// # Purpose
///
/// The `<main>` element represents the dominant content of the `<body>` of a document.
/// The main content area consists of content directly related to or expanding upon the
/// central topic. There should be only one `<main>` element per page (not hidden).
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Main content area of a webpage
/// - Primary content excluding headers, footers, and navigation
/// - Content unique to the page (not repeated across pages)
/// - Central topic or functionality of a page
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <body>
///   <header>
///     <h1>My Website</h1>
///     <nav>...</nav>
///   </header>
///   
///   <main>
///     <h2>Welcome to Our Site</h2>
///     <article>
///       <h3>Latest News</h3>
///       <p>Content...</p>
///     </article>
///   </main>
///   
///   <footer>...</footer>
/// </body>
/// ```
///
/// # Accessibility
///
/// - Screen readers provide shortcuts to jump to `<main>` content
/// - Critical for keyboard navigation and skip links
/// - Only one visible `<main>` per page
///
/// # WHATWG Specification
///
/// - [4.3.11 The main element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-main-element)
pub struct Main;
impl HtmlElement for Main {
    const TAG: &'static str = "main";
}
impl FlowContent for Main {}
impl PalpableContent for Main {}

// =============================================================================
// Text Content Elements
// =============================================================================

/// The `<div>` element - generic container.
///
/// # Purpose
///
/// The `<div>` element is a generic container for flow content. It has no semantic meaning
/// and should be used only when no other semantic element is appropriate. It's primarily
/// used for styling purposes or as a container for scripting.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content (if it has at least one child)
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Layout containers for CSS Grid or Flexbox
/// - Grouping elements for styling with CSS classes
/// - JavaScript manipulation targets
/// - Generic wrappers when no semantic element fits
///
/// # Key Attributes
///
/// - Global attributes only (commonly `class`, `id`, `style`)
///
/// # Example
///
/// ```html
/// <div class="container">
///   <div class="header">
///     <h1>Page Title</h1>
///   </div>
///   <div class="content">
///     <p>Main content...</p>
///   </div>
/// </div>
/// ```
///
/// # WHATWG Specification
///
/// - [4.4.15 The div element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-div-element)
pub struct Div;
impl HtmlElement for Div {
    const TAG: &'static str = "div";
}
impl FlowContent for Div {}
impl PalpableContent for Div {}

/// The `<p>` element - paragraph.
///
/// # Purpose
///
/// The `<p>` element represents a paragraph of text. It's one of the most commonly
/// used HTML elements for organizing textual content into distinct blocks.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content (no block-level elements like `<div>`, `<p>`, or headings)
///
/// # Common Use Cases
///
/// - Body text in articles and documents
/// - Descriptive text blocks
/// - Text content in any context
/// - Structuring prose content
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <article>
///   <h1>Introduction to HTML</h1>
///   <p>HTML (HyperText Markup Language) is the standard markup language
///   for creating web pages.</p>
///   <p>It describes the structure of web pages using markup elements
///   called tags.</p>
/// </article>
/// ```
///
/// # Accessibility
///
/// - Screen readers pause between paragraphs
/// - Natural text structure for reading flow
///
/// # WHATWG Specification
///
/// - [4.4.1 The p element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-p-element)
pub struct P;
impl HtmlElement for P {
    const TAG: &'static str = "p";
}
impl FlowContent for P {}
impl PalpableContent for P {}

/// The `<hr>` element - thematic break.
///
/// # Purpose
///
/// The `<hr>` element represents a thematic break between paragraph-level elements,
/// such as a scene change in a story or a shift in topic. It's displayed as a
/// horizontal rule but carries semantic meaning beyond visual presentation.
///
/// # Content Categories
///
/// - Flow Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Separating sections in a document
/// - Indicating topic shifts within content
/// - Visual and semantic breaks in text flow
/// - Scene changes in narrative content
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <section>
///   <h2>Chapter 1</h2>
///   <p>The story begins...</p>
/// </section>
/// <hr>
/// <section>
///   <h2>Chapter 2</h2>
///   <p>Meanwhile, in another place...</p>
/// </section>
/// ```
///
/// # WHATWG Specification
///
/// - [4.4.2 The hr element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-hr-element)
pub struct Hr;
impl HtmlElement for Hr {
    const TAG: &'static str = "hr";
    const VOID: bool = true;
}
impl FlowContent for Hr {}

/// The `<pre>` element - preformatted text.
///
/// # Purpose
///
/// The `<pre>` element represents preformatted text where whitespace (spaces, tabs,
/// line breaks) is preserved exactly as written in the HTML. Text is typically
/// displayed in a monospace font.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Displaying code snippets (often with `<code>`)
/// - ASCII art or text diagrams
/// - Preserving formatting of plain text
/// - Command-line output or logs
/// - Poetry or text where line breaks matter
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <pre><code>function hello() {
///   console.log("Hello, World!");
/// }</code></pre>
///
/// <pre>
///    /\_/\
///   ( o.o )
///    > ^ <
/// </pre>
/// ```
///
/// # WHATWG Specification
///
/// - [4.4.3 The pre element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-pre-element)
pub struct Pre;
impl HtmlElement for Pre {
    const TAG: &'static str = "pre";
}
impl FlowContent for Pre {}
impl PalpableContent for Pre {}

/// The `<blockquote>` element - block quotation.
///
/// # Purpose
///
/// The `<blockquote>` element represents a section that is quoted from another source.
/// It's used for longer quotations that span multiple lines or paragraphs, as opposed
/// to inline quotes which use `<q>`.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Long quotations from other sources
/// - Testimonials and reviews
/// - Excerpts from books or articles
/// - Citations in academic or professional writing
///
/// # Key Attributes
///
/// - `cite`: URL of the source document or message
/// - Global attributes
///
/// # Example
///
/// ```html
/// <blockquote cite="https://www.example.com/article">
///   <p>The future belongs to those who believe in the beauty
///   of their dreams.</p>
///   <footer>— Eleanor Roosevelt</footer>
/// </blockquote>
///
/// <p>As the specification states:</p>
/// <blockquote>
///   <p>The blockquote element represents content that is quoted
///   from another source.</p>
/// </blockquote>
/// ```
///
/// # WHATWG Specification
///
/// - [4.4.4 The blockquote element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-blockquote-element)
pub struct Blockquote;
impl HtmlElement for Blockquote {
    const TAG: &'static str = "blockquote";
}
impl FlowContent for Blockquote {}
impl PalpableContent for Blockquote {}

/// The `<ol>` element - ordered list.
///
/// # Purpose
///
/// The `<ol>` element represents an ordered list of items, where the order is
/// meaningful. Items are typically numbered but can use other markers like
/// letters or Roman numerals.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content (if it has at least one `<li>` child)
///
/// # Permitted Content Model
///
/// - Zero or more `<li>` and script-supporting elements
///
/// # Common Use Cases
///
/// - Step-by-step instructions or procedures
/// - Ranked lists (top 10, etc.)
/// - Sequential content where order matters
/// - Table of contents with numbered sections
/// - Legal or technical document numbering
///
/// # Key Attributes
///
/// - `reversed`: Reverses the numbering order
/// - `start`: Starting number for the list (e.g., `start="5"`)
/// - `type`: Numbering type (`"1"`, `"A"`, `"a"`, `"I"`, `"i"`)
/// - Global attributes
///
/// # Example
///
/// ```html
/// <h2>Recipe Instructions</h2>
/// <ol>
///   <li>Preheat oven to 350°F</li>
///   <li>Mix dry ingredients</li>
///   <li>Add wet ingredients</li>
///   <li>Bake for 25 minutes</li>
/// </ol>
///
/// <ol type="A" start="3">
///   <li>Option C</li>
///   <li>Option D</li>
/// </ol>
/// ```
///
/// # Accessibility
///
/// - Screen readers announce list with item count
/// - Sequential numbering aids comprehension
///
/// # WHATWG Specification
///
/// - [4.4.5 The ol element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-ol-element)
pub struct Ol;
impl HtmlElement for Ol {
    const TAG: &'static str = "ol";
}
impl FlowContent for Ol {}
impl PalpableContent for Ol {}

/// The `<ul>` element - unordered list.
///
/// # Purpose
///
/// The `<ul>` element represents an unordered list of items, where the order is not
/// meaningful. Items are typically marked with bullets or other symbols.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content (if it has at least one `<li>` child)
///
/// # Permitted Content Model
///
/// - Zero or more `<li>` and script-supporting elements
///
/// # Common Use Cases
///
/// - Feature lists
/// - Navigation menus
/// - Collections of related items
/// - Tag or category lists
/// - Any list where order doesn't matter
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <h2>Features</h2>
/// <ul>
///   <li>Fast performance</li>
///   <li>Easy to use</li>
///   <li>Highly customizable</li>
///   <li>Mobile-friendly</li>
/// </ul>
///
/// <nav>
///   <ul>
///     <li><a href="/">Home</a></li>
///     <li><a href="/about">About</a></li>
///     <li><a href="/contact">Contact</a></li>
///   </ul>
/// </nav>
/// ```
///
/// # Accessibility
///
/// - Screen readers announce list type and item count
/// - Provides semantic grouping of related items
///
/// # WHATWG Specification
///
/// - [4.4.6 The ul element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-ul-element)
pub struct Ul;
impl HtmlElement for Ul {
    const TAG: &'static str = "ul";
}
impl FlowContent for Ul {}
impl PalpableContent for Ul {}

/// The `<menu>` element - menu of commands.
///
/// # Purpose
///
/// The `<menu>` element represents a group of commands or a list of options that a
/// user can perform or activate. It's semantically similar to `<ul>` but specifically
/// for interactive commands or options.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content (if it has at least one `<li>` child)
///
/// # Permitted Content Model
///
/// - Zero or more `<li>`, `<script>`, and `<template>` elements
///
/// # Common Use Cases
///
/// - Toolbar buttons
/// - Context menus
/// - List of commands or actions
/// - Interactive option lists
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <menu>
///   <li><button type="button">New File</button></li>
///   <li><button type="button">Open</button></li>
///   <li><button type="button">Save</button></li>
///   <li><button type="button">Exit</button></li>
/// </menu>
/// ```
///
/// # WHATWG Specification
///
/// - [4.4.7 The menu element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-menu-element)
pub struct Menu;
impl HtmlElement for Menu {
    const TAG: &'static str = "menu";
}
impl FlowContent for Menu {}
impl PalpableContent for Menu {}

/// The `<li>` element - list item.
///
/// # Purpose
///
/// The `<li>` element represents a list item within ordered (`<ol>`), unordered (`<ul>`),
/// or menu (`<menu>`) lists. It's the container for individual items in a list structure.
///
/// # Content Categories
///
/// - None (only valid as child of `<ol>`, `<ul>`, or `<menu>`)
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Items in ordered or unordered lists
/// - Navigation menu items
/// - Steps in procedures
/// - Feature or specification lists
///
/// # Key Attributes
///
/// - `value`: Ordinal value for the item (only in `<ol>`)
/// - Global attributes
///
/// # Example
///
/// ```html
/// <ul>
///   <li>First item</li>
///   <li>Second item with <strong>emphasis</strong></li>
///   <li>
///     Third item with nested content
///     <ul>
///       <li>Nested item</li>
///     </ul>
///   </li>
/// </ul>
///
/// <ol>
///   <li value="10">Start at 10</li>
///   <li>This will be 11</li>
/// </ol>
/// ```
///
/// # Accessibility
///
/// - Screen readers announce list position (e.g., "item 1 of 3")
///
/// # WHATWG Specification
///
/// - [4.4.8 The li element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-li-element)
pub struct Li;
impl HtmlElement for Li {
    const TAG: &'static str = "li";
}

/// The `<dl>` element - description list.
///
/// # Purpose
///
/// The `<dl>` element represents an association list consisting of zero or more
/// name-value groups (term-description pairs). Common uses include glossaries,
/// metadata, or key-value pairs.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content (if it has at least one name-value pair)
///
/// # Permitted Content Model
///
/// - Zero or more groups of one or more `<dt>` elements followed by one or more `<dd>` elements
/// - Optionally intermixed with `<script>` and `<template>` elements
/// - Can also contain `<div>` elements wrapping `<dt>` and `<dd>` groups
///
/// # Common Use Cases
///
/// - Glossaries and definitions
/// - Metadata or property lists
/// - FAQ sections (question-answer pairs)
/// - Product specifications
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <dl>
///   <dt>HTML</dt>
///   <dd>HyperText Markup Language</dd>
///   
///   <dt>CSS</dt>
///   <dd>Cascading Style Sheets</dd>
///   
///   <dt>JavaScript</dt>
///   <dt>JS</dt>
///   <dd>A programming language for web browsers</dd>
/// </dl>
/// ```
///
/// # Accessibility
///
/// - Screen readers may announce term-description relationships
///
/// # WHATWG Specification
///
/// - [4.4.9 The dl element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-dl-element)
pub struct Dl;
impl HtmlElement for Dl {
    const TAG: &'static str = "dl";
}
impl FlowContent for Dl {}
impl PalpableContent for Dl {}

/// The `<dt>` element - description term.
///
/// # Purpose
///
/// The `<dt>` element represents the term or name part of a term-description group
/// in a description list (`<dl>`). It specifies the term being defined or described.
///
/// # Content Categories
///
/// - None (only valid as child of `<dl>` or `<div>` within `<dl>`)
///
/// # Permitted Content Model
///
/// - Flow content (but no `<header>`, `<footer>`, sectioning, or heading content descendants)
///
/// # Common Use Cases
///
/// - Term in a glossary
/// - Property name in metadata
/// - Question in FAQ
/// - Label in key-value pairs
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <dl>
///   <dt>Name</dt>
///   <dd>John Doe</dd>
///   
///   <dt>Email</dt>
///   <dd>john@example.com</dd>
///   
///   <dt>What is HTML?</dt>
///   <dd>HTML is the standard markup language for web pages.</dd>
/// </dl>
/// ```
///
/// # WHATWG Specification
///
/// - [4.4.10 The dt element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-dt-element)
pub struct Dt;
impl HtmlElement for Dt {
    const TAG: &'static str = "dt";
}

/// The `<dd>` element - description details.
///
/// # Purpose
///
/// The `<dd>` element represents the description, definition, or value part of a
/// term-description group in a description list (`<dl>`). It provides details for
/// the term specified by the preceding `<dt>` element(s).
///
/// # Content Categories
///
/// - None (only valid as child of `<dl>` or `<div>` within `<dl>`)
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Definition in a glossary
/// - Property value in metadata
/// - Answer in FAQ
/// - Value in key-value pairs
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <dl>
///   <dt>HTTP</dt>
///   <dd>HyperText Transfer Protocol - the foundation of data
///   communication for the World Wide Web.</dd>
///   
///   <dt>Status</dt>
///   <dd>Active</dd>
///   
///   <dt>Author</dt>
///   <dt>Contributor</dt>
///   <dd>Jane Smith</dd>
///   <dd>John Doe</dd>
/// </dl>
/// ```
///
/// # WHATWG Specification
///
/// - [4.4.11 The dd element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-dd-element)
pub struct Dd;
impl HtmlElement for Dd {
    const TAG: &'static str = "dd";
}

/// The `<figure>` element - self-contained content.
///
/// # Purpose
///
/// The `<figure>` element represents self-contained content, typically with a caption,
/// that is referenced as a single unit from the main content. Often used for images,
/// diagrams, code listings, or other content that can be moved away from the main flow.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Either: one `<figcaption>` followed by flow content
/// - Or: flow content followed by one `<figcaption>`
/// - Or: flow content only
///
/// # Common Use Cases
///
/// - Images with captions
/// - Code examples with descriptions
/// - Diagrams or illustrations
/// - Quotations with attributions
/// - Videos or multimedia with captions
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <figure>
///   <img src="chart.png" alt="Sales data chart">
///   <figcaption>Figure 1: Q4 Sales Performance</figcaption>
/// </figure>
///
/// <figure>
///   <pre><code>function greet(name) {
///   return `Hello, ${name}!`;
/// }</code></pre>
///   <figcaption>Example: Template literal usage</figcaption>
/// </figure>
/// ```
///
/// # Accessibility
///
/// - Screen readers associate caption with content
/// - Use `<figcaption>` for accessible descriptions
///
/// # WHATWG Specification
///
/// - [4.4.12 The figure element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-figure-element)
pub struct Figure;
impl HtmlElement for Figure {
    const TAG: &'static str = "figure";
}
impl FlowContent for Figure {}
impl PalpableContent for Figure {}

/// The `<figcaption>` element - figure caption.
///
/// # Purpose
///
/// The `<figcaption>` element represents a caption or legend for the content of its
/// parent `<figure>` element. It provides a description or title for the figure.
///
/// # Content Categories
///
/// - None (only valid as child of `<figure>`)
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Image captions
/// - Code example descriptions
/// - Chart or diagram titles
/// - Table or figure numbering and titles
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <figure>
///   <img src="mountain.jpg" alt="Mountain landscape">
///   <figcaption>
///     <strong>Figure 2.1:</strong> The Rocky Mountains at sunset.
///     Photo by Jane Photographer.
///   </figcaption>
/// </figure>
///
/// <figure>
///   <table>
///     <tr><th>Year</th><th>Sales</th></tr>
///     <tr><td>2023</td><td>$1.2M</td></tr>
///   </table>
///   <figcaption>Table 1: Annual sales figures</figcaption>
/// </figure>
/// ```
///
/// # WHATWG Specification
///
/// - [4.4.13 The figcaption element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-figcaption-element)
pub struct Figcaption;
impl HtmlElement for Figcaption {
    const TAG: &'static str = "figcaption";
}

/// The `<search>` element - search functionality.
///
/// # Purpose
///
/// The `<search>` element represents a part of a document or application that contains
/// form controls or other content related to performing a search or filtering operation.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Site-wide search forms
/// - Filtering interfaces
/// - Search functionality within specific sections
/// - Product or content search widgets
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <search>
///   <form action="/search" method="get">
///     <label for="query">Search:</label>
///     <input type="search" id="query" name="q">
///     <button type="submit">Search</button>
///   </form>
/// </search>
///
/// <search>
///   <h2>Filter Products</h2>
///   <label>Price: <input type="range" min="0" max="1000"></label>
///   <label>Category: <select>...</select></label>
/// </search>
/// ```
///
/// # Accessibility
///
/// - Provides semantic meaning for search regions
/// - Screen readers can identify search landmarks
///
/// # WHATWG Specification
///
/// - [4.4.14 The search element](https://html.spec.whatwg.org/multipage/grouping-content.html#the-search-element)
pub struct Search;
impl HtmlElement for Search {
    const TAG: &'static str = "search";
}
impl FlowContent for Search {}
impl PalpableContent for Search {}

// =============================================================================
// Inline Text Semantics
// =============================================================================

/// The `<a>` element - hyperlink (anchor).
///
/// # Purpose
///
/// The `<a>` element creates a hyperlink to other web pages, files, locations within
/// the same page, email addresses, or any other URL. It's one of the most fundamental
/// elements of the web, enabling navigation between resources.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Interactive Content (if it has an `href` attribute)
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Transparent (inherits from parent), but must not contain interactive content
///
/// # Common Use Cases
///
/// - Linking to other pages or websites
/// - Creating in-page navigation (anchor links)
/// - Downloadable file links
/// - Email links (mailto:)
/// - Telephone links (tel:)
///
/// # Key Attributes
///
/// - `href`: URL or fragment identifier
/// - `target`: Browsing context (`"_blank"`, `"_self"`, `"_parent"`, `"_top"`)
/// - `rel`: Relationship to linked resource (`"noopener"`, `"noreferrer"`, `"nofollow"`)
/// - `download`: Suggests download instead of navigation
///
/// # Example
///
/// ```html
/// <p>Visit our <a href="https://example.com">website</a> for more information.</p>
/// <p><a href="#section2">Jump to Section 2</a></p>
/// <p><a href="mailto:info@example.com">Email us</a></p>
/// <p><a href="document.pdf" download>Download PDF</a></p>
/// <p><a href="https://external.com" target="_blank" rel="noopener">External link</a></p>
/// ```
///
/// # Accessibility
///
/// - Link text should be descriptive (avoid "click here")
/// - Screen readers announce links separately
/// - Keyboard accessible by default
///
/// # WHATWG Specification
///
/// - [4.5.1 The a element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element)
pub struct A;
impl HtmlElement for A {
    const TAG: &'static str = "a";
}
impl FlowContent for A {}
impl PhrasingContent for A {}
impl InteractiveContent for A {}
impl PalpableContent for A {}

/// The `<em>` element - emphasis.
///
/// # Purpose
///
/// The `<em>` element represents stress emphasis of its contents. The level of emphasis
/// can be increased by nesting `<em>` elements. Typically rendered in italic, but the
/// emphasis is semantic, not just visual.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Emphasizing important words or phrases
/// - Changing the meaning based on stress
/// - Highlighting key terms in context
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>I <em>really</em> need to finish this today.</p>
/// <p>Cats are <em>cute</em> animals.</p>
/// <p>Make sure you <em>do not</em> forget!</p>
/// ```
///
/// # Accessibility
///
/// - Screen readers may use different voice inflection
/// - Conveys semantic emphasis, not just styling
///
/// # WHATWG Specification
///
/// - [4.5.2 The em element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-em-element)
pub struct Em;
impl HtmlElement for Em {
    const TAG: &'static str = "em";
}
impl FlowContent for Em {}
impl PhrasingContent for Em {}
impl PalpableContent for Em {}

/// The `<strong>` element - strong importance.
///
/// # Purpose
///
/// The `<strong>` element represents strong importance, seriousness, or urgency for its
/// contents. Typically rendered in bold, but the semantic meaning is importance, not
/// just visual weight.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Marking critical information or warnings
/// - Highlighting important concepts
/// - Indicating urgency or seriousness
/// - Key terms or takeaways
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p><strong>Warning:</strong> This action cannot be undone.</p>
/// <p>The deadline is <strong>tomorrow</strong>.</p>
/// <p><strong>Important:</strong> Save your work frequently.</p>
/// ```
///
/// # Accessibility
///
/// - Screen readers may emphasize strong content
/// - Conveys semantic importance
///
/// # WHATWG Specification
///
/// - [4.5.3 The strong element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-strong-element)
pub struct Strong;
impl HtmlElement for Strong {
    const TAG: &'static str = "strong";
}
impl FlowContent for Strong {}
impl PhrasingContent for Strong {}
impl PalpableContent for Strong {}

/// The `<small>` element - side comments and small print.
///
/// # Purpose
///
/// The `<small>` element represents side comments such as small print, including
/// copyright, legal text, disclaimers, caveats, or other fine print. It doesn't
/// "de-emphasize" content semantically.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Copyright notices
/// - Legal disclaimers
/// - Fine print and caveats
/// - License information
/// - Attribution text
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <footer>
///   <p><small>&copy; 2024 My Company. All rights reserved.</small></p>
/// </footer>
///
/// <p>Price: $99.99 <small>(taxes not included)</small></p>
/// <p><small>This offer expires on December 31, 2024.</small></p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.4 The small element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-small-element)
pub struct Small;
impl HtmlElement for Small {
    const TAG: &'static str = "small";
}
impl FlowContent for Small {}
impl PhrasingContent for Small {}
impl PalpableContent for Small {}

/// The `<s>` element - strikethrough (no longer accurate).
///
/// # Purpose
///
/// The `<s>` element represents contents that are no longer accurate or no longer relevant.
/// It's typically rendered with a strikethrough line. Use `<del>` for indicating deletions.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Showing outdated prices or information
/// - Indicating obsolete content
/// - Sale prices (showing old price crossed out)
/// - Deprecated features or information
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>Price: <s>$99.99</s> $79.99 (on sale!)</p>
/// <p><s>Meeting at 3pm</s> Meeting rescheduled to 4pm</p>
/// <p><s>This feature is experimental</s> Now stable and recommended</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.5 The s element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-s-element)
pub struct S;
impl HtmlElement for S {
    const TAG: &'static str = "s";
}
impl FlowContent for S {}
impl PhrasingContent for S {}
impl PalpableContent for S {}

/// The `<cite>` element - citation or reference to a creative work.
///
/// # Purpose
///
/// The `<cite>` element represents a reference to a creative work such as a book, article,
/// movie, song, or other cited work. It should contain the title of the work.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Citing book titles
/// - Referencing articles or papers
/// - Mentioning movies, songs, or artworks
/// - Legal case citations
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>As described in <cite>The Great Gatsby</cite>, the American Dream...</p>
/// <p>According to the research in <cite>Nature</cite> journal...</p>
/// <blockquote>
///   <p>To be or not to be...</p>
///   <footer>— <cite>Hamlet</cite> by William Shakespeare</footer>
/// </blockquote>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.6 The cite element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-cite-element)
pub struct Cite;
impl HtmlElement for Cite {
    const TAG: &'static str = "cite";
}
impl FlowContent for Cite {}
impl PhrasingContent for Cite {}
impl PalpableContent for Cite {}

/// The `<q>` element - inline quotation.
///
/// # Purpose
///
/// The `<q>` element represents a short inline quotation. Browsers typically add
/// quotation marks automatically. For longer block quotes, use `<blockquote>`.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Short inline quotations
/// - Quoted phrases within sentences
/// - Referenced statements
///
/// # Key Attributes
///
/// - `cite`: URL of the source of the quotation
/// - Global attributes
///
/// # Example
///
/// ```html
/// <p>She said, <q>I'll be there at 5pm</q>, and left.</p>
/// <p>The motto is <q>Quality over quantity</q>.</p>
/// <p>As Einstein said, <q cite="https://example.com">Imagination is more
/// important than knowledge</q>.</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.7 The q element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-q-element)
pub struct Q;
impl HtmlElement for Q {
    const TAG: &'static str = "q";
}
impl FlowContent for Q {}
impl PhrasingContent for Q {}
impl PalpableContent for Q {}

/// The `<dfn>` element - defining instance of a term.
///
/// # Purpose
///
/// The `<dfn>` element represents the defining instance of a term. The paragraph,
/// description list group, or section that contains the `<dfn>` element should also
/// contain the definition of the term.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content (but must not contain another `<dfn>` element)
///
/// # Common Use Cases
///
/// - Defining technical terms in glossaries
/// - First occurrence of a term being defined
/// - Introducing new terminology
/// - Academic or technical documentation
///
/// # Key Attributes
///
/// - `title`: Full term being defined (if abbreviation is in content)
/// - Global attributes
///
/// # Example
///
/// ```html
/// <p><dfn>HTML</dfn> (HyperText Markup Language) is the standard
/// markup language for web pages.</p>
///
/// <p>A <dfn id="def-widget">widget</dfn> is a small application
/// that provides specific functionality.</p>
///
/// <p><dfn><abbr title="Cascading Style Sheets">CSS</abbr></dfn>
/// is used for styling web pages.</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.8 The dfn element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-dfn-element)
pub struct Dfn;
impl HtmlElement for Dfn {
    const TAG: &'static str = "dfn";
}
impl FlowContent for Dfn {}
impl PhrasingContent for Dfn {}
impl PalpableContent for Dfn {}

/// The `<abbr>` element - abbreviation or acronym.
///
/// # Purpose
///
/// The `<abbr>` element represents an abbreviation or acronym. The `title` attribute
/// can provide the full expansion of the abbreviation, which is often shown as a
/// tooltip on hover.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Abbreviations (Dr., Inc., etc.)
/// - Acronyms (HTML, CSS, API)
/// - Shortened terms with full expansions
/// - Technical terminology
///
/// # Key Attributes
///
/// - `title`: Full expansion or description of the abbreviation
/// - Global attributes
///
/// # Example
///
/// ```html
/// <p>The <abbr title="World Health Organization">WHO</abbr> was founded in 1948.</p>
/// <p>Use <abbr title="HyperText Markup Language">HTML</abbr> for structure.</p>
/// <p>Contact <abbr title="Doctor">Dr.</abbr> Smith for more information.</p>
/// <p>The <abbr title="Application Programming Interface">API</abbr> is RESTful.</p>
/// ```
///
/// # Accessibility
///
/// - Screen readers may announce the expansion from `title`
/// - Helps users understand unfamiliar abbreviations
///
/// # WHATWG Specification
///
/// - [4.5.9 The abbr element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-abbr-element)
pub struct Abbr;
impl HtmlElement for Abbr {
    const TAG: &'static str = "abbr";
}
impl FlowContent for Abbr {}
impl PhrasingContent for Abbr {}
impl PalpableContent for Abbr {}

/// The `<ruby>` element - ruby annotation.
///
/// # Purpose
///
/// The `<ruby>` element represents a ruby annotation, which is used to show pronunciation
/// of East Asian characters. A ruby annotation consists of the base text and ruby text
/// (typically pronunciation), often displayed above or beside the base text.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content and `<rt>`, `<rp>` elements
///
/// # Common Use Cases
///
/// - Japanese furigana (hiragana pronunciation guides)
/// - Chinese pinyin
/// - Korean pronunciation guides
/// - Pronunciation annotations for any language
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <ruby>
///   漢<rt>kan</rt>
///   字<rt>ji</rt>
/// </ruby>
///
/// <ruby>
///   東京<rp>(</rp><rt>とうきょう</rt><rp>)</rp>
/// </ruby>
/// ```
///
/// # Accessibility
///
/// - Screen readers may announce both base and ruby text
/// - `<rp>` provides fallback for browsers without ruby support
///
/// # WHATWG Specification
///
/// - [4.5.11 The ruby element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-ruby-element)
pub struct Ruby;
impl HtmlElement for Ruby {
    const TAG: &'static str = "ruby";
}
impl FlowContent for Ruby {}
impl PhrasingContent for Ruby {}
impl PalpableContent for Ruby {}

/// The `<rt>` element - ruby text component.
///
/// # Purpose
///
/// The `<rt>` element contains the ruby text component of a ruby annotation,
/// providing pronunciation or translation information for the base text in a
/// `<ruby>` element.
///
/// # Content Categories
///
/// - None (only valid as child of `<ruby>`)
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Pronunciation guides (furigana, pinyin)
/// - Translation or meaning annotations
/// - Phonetic transcriptions
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <ruby>日本<rt>にほん</rt></ruby>
/// <ruby>北京<rt>Běijīng</rt></ruby>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.12 The rt element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-rt-element)
pub struct Rt;
impl HtmlElement for Rt {
    const TAG: &'static str = "rt";
}

/// The `<rp>` element - ruby fallback parenthesis.
///
/// # Purpose
///
/// The `<rp>` element provides parentheses or other characters to be displayed
/// by browsers that don't support ruby annotations. It's used to wrap ruby text
/// in fallback scenarios.
///
/// # Content Categories
///
/// - None (only valid as child of `<ruby>`)
///
/// # Permitted Content Model
///
/// - Text (typically parentheses)
///
/// # Common Use Cases
///
/// - Fallback parentheses for ruby annotations
/// - Ensuring ruby text is distinguishable in non-supporting browsers
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <ruby>
///   漢字<rp>(</rp><rt>かんじ</rt><rp>)</rp>
/// </ruby>
/// <!-- Browsers without ruby support show: 漢字(かんじ) -->
/// <!-- Browsers with ruby support show annotation without parentheses -->
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.13 The rp element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-rp-element)
pub struct Rp;
impl HtmlElement for Rp {
    const TAG: &'static str = "rp";
}

/// The `<data>` element - machine-readable data.
///
/// # Purpose
///
/// The `<data>` element links content to a machine-readable translation via the `value`
/// attribute. It associates human-readable content with a machine-readable equivalent,
/// useful for data processing, sorting, or semantic web applications.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Product identifiers (SKUs, UPCs)
/// - Numeric data with formatted display
/// - Sortable values with custom display
/// - Machine-readable metadata
///
/// # Key Attributes
///
/// - `value`: Machine-readable value (required)
/// - Global attributes
///
/// # Example
///
/// ```html
/// <p>Product: <data value="SKU-12345">Widget Pro</data></p>
/// <p>Price: <data value="39.99">$39.99 USD</data></p>
/// <ul>
///   <li><data value="8">Eight</data> items</li>
///   <li><data value="21">Twenty-one</data> items</li>
/// </ul>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.14 The data element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-data-element)
pub struct Data;
impl HtmlElement for Data {
    const TAG: &'static str = "data";
}
impl FlowContent for Data {}
impl PhrasingContent for Data {}
impl PalpableContent for Data {}

/// The `<time>` element - date and/or time.
///
/// # Purpose
///
/// The `<time>` element represents a specific time or date, optionally with a
/// machine-readable timestamp in the `datetime` attribute. It helps search engines
/// and other software understand temporal information.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content (but no `<time>` descendants)
///
/// # Common Use Cases
///
/// - Publication dates
/// - Event dates and times
/// - Deadlines and schedules
/// - Historical dates
/// - Time-based content
///
/// # Key Attributes
///
/// - `datetime`: Machine-readable date/time (ISO 8601 format)
/// - Global attributes
///
/// # Example
///
/// ```html
/// <p>Posted on <time datetime="2024-01-15">January 15, 2024</time></p>
/// <p>Event starts at <time datetime="2024-06-20T19:00">7:00 PM on June 20</time></p>
/// <p>Updated <time datetime="2024-01-15T14:30:00Z">today at 2:30 PM</time></p>
/// <time datetime="2024">2024</time>
/// ```
///
/// # Accessibility
///
/// - Provides semantic meaning for dates and times
/// - `datetime` attribute aids machine parsing
///
/// # WHATWG Specification
///
/// - [4.5.15 The time element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-time-element)
pub struct Time;
impl HtmlElement for Time {
    const TAG: &'static str = "time";
}
impl FlowContent for Time {}
impl PhrasingContent for Time {}
impl PalpableContent for Time {}

/// The `<code>` element - code fragment.
///
/// # Purpose
///
/// The `<code>` element represents a fragment of computer code. It can be an inline code
/// snippet within text or used within `<pre>` for code blocks. Typically displayed in
/// a monospace font.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Inline code references in documentation
/// - Code blocks (with `<pre>`)
/// - Function or variable names in text
/// - HTML/CSS/JavaScript snippets
/// - Programming examples
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>Use the <code>console.log()</code> function for debugging.</p>
/// <p>The <code>&lt;div&gt;</code> element is a container.</p>
/// <pre><code>function greet(name) {
///   return `Hello, ${name}!`;
/// }</code></pre>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.16 The code element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-code-element)
pub struct Code;
impl HtmlElement for Code {
    const TAG: &'static str = "code";
}
impl FlowContent for Code {}
impl PhrasingContent for Code {}
impl PalpableContent for Code {}

/// The `<var>` element - variable or placeholder.
///
/// # Purpose
///
/// The `<var>` element represents a variable in a mathematical expression, programming
/// context, or a placeholder where the user should substitute their own value.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Variables in mathematical equations
/// - Programming variable names
/// - Placeholder values in documentation
/// - Parameters in formulas or functions
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>The area of a rectangle is <var>width</var> × <var>height</var>.</p>
/// <p>Set <var>username</var> to your account name.</p>
/// <p>The function signature is <code>greet(<var>name</var>)</code>.</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.17 The var element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-var-element)
pub struct Var;
impl HtmlElement for Var {
    const TAG: &'static str = "var";
}
impl FlowContent for Var {}
impl PhrasingContent for Var {}
impl PalpableContent for Var {}

/// The `<samp>` element - sample output.
///
/// # Purpose
///
/// The `<samp>` element represents sample or quoted output from a computer program,
/// script, or system. It's used to show what a program or system would display.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Command-line output
/// - Error messages
/// - System responses
/// - Program execution results
/// - Log file excerpts
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>The program outputs: <samp>Hello, World!</samp></p>
/// <p>Error message: <samp>File not found</samp></p>
/// <pre><samp>$ node app.js
/// Server listening on port 3000</samp></pre>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.18 The samp element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-samp-element)
pub struct Samp;
impl HtmlElement for Samp {
    const TAG: &'static str = "samp";
}
impl FlowContent for Samp {}
impl PhrasingContent for Samp {}
impl PalpableContent for Samp {}

/// The `<kbd>` element - keyboard input.
///
/// # Purpose
///
/// The `<kbd>` element represents user input from a keyboard, voice input, or any other
/// text entry device. It's used to indicate keys, key combinations, or commands that
/// users should enter.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Keyboard shortcuts
/// - Keys to press
/// - Command-line commands user should type
/// - Menu selections or button presses
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>Press <kbd>Ctrl</kbd>+<kbd>C</kbd> to copy.</p>
/// <p>Save the file with <kbd>Ctrl</kbd>+<kbd>S</kbd>.</p>
/// <p>To quit, type <kbd>exit</kbd> and press <kbd>Enter</kbd>.</p>
/// <p>Use <kbd><kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>Delete</kbd></kbd> to restart.</p>
/// ```
///
/// # Accessibility
///
/// - Screen readers may announce keyboard input distinctly
/// - Helps users identify actionable keyboard commands
///
/// # WHATWG Specification
///
/// - [4.5.19 The kbd element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-kbd-element)
pub struct Kbd;
impl HtmlElement for Kbd {
    const TAG: &'static str = "kbd";
}
impl FlowContent for Kbd {}
impl PhrasingContent for Kbd {}
impl PalpableContent for Kbd {}

/// The `<sub>` element - subscript.
///
/// # Purpose
///
/// The `<sub>` element represents subscript text, which appears half a character below
/// the normal line and is often rendered in a smaller font. Used for mathematical,
/// chemical, or typographical notations.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Chemical formulas (H₂O)
/// - Mathematical subscripts (xᵢ, aₙ)
/// - Footnote references
/// - Typographic conventions
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>The chemical formula for water is H<sub>2</sub>O.</p>
/// <p>The variable x<sub>i</sub> represents the i-th element.</p>
/// <p>CO<sub>2</sub> emissions have increased.</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.20 The sub element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-sub-and-sup-elements)
pub struct Sub;
impl HtmlElement for Sub {
    const TAG: &'static str = "sub";
}
impl FlowContent for Sub {}
impl PhrasingContent for Sub {}
impl PalpableContent for Sub {}

/// The `<sup>` element - superscript.
///
/// # Purpose
///
/// The `<sup>` element represents superscript text, which appears half a character above
/// the normal line and is often rendered in a smaller font. Used for exponents, ordinal
/// indicators, and footnote markers.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Mathematical exponents (x², 10³)
/// - Ordinal indicators (1st, 2nd, 3rd)
/// - Footnote or reference markers
/// - Trademark symbols (™, ®)
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>Einstein's equation: E = mc<sup>2</sup></p>
/// <p>Area of a square: side<sup>2</sup></p>
/// <p>On the 21<sup>st</sup> of March...</p>
/// <p>Copyright<sup>&copy;</sup> 2024</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.20 The sup element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-sub-and-sup-elements)
pub struct Sup;
impl HtmlElement for Sup {
    const TAG: &'static str = "sup";
}
impl FlowContent for Sup {}
impl PhrasingContent for Sup {}
impl PalpableContent for Sup {}

/// The `<i>` element - idiomatic text.
///
/// # Purpose
///
/// The `<i>` element represents text in an alternate voice or mood, or otherwise offset
/// from normal prose. Examples include taxonomic designations, technical terms, foreign
/// phrases, thoughts, or ship names. Not for emphasis—use `<em>` for that.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Foreign language phrases
/// - Technical or taxonomic terms
/// - Thoughts or internal dialogue
/// - Ship or vessel names
/// - Idiomatic expressions
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>The term <i>café</i> comes from French.</p>
/// <p>The species <i>Homo sapiens</i> evolved in Africa.</p>
/// <p><i>I wonder what she meant,</i> he thought.</p>
/// <p>The <i>HMS Victory</i> was Nelson's flagship.</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.21 The i element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-i-element)
pub struct I;
impl HtmlElement for I {
    const TAG: &'static str = "i";
}
impl FlowContent for I {}
impl PhrasingContent for I {}
impl PalpableContent for I {}

/// The `<b>` element - bring attention to.
///
/// # Purpose
///
/// The `<b>` element draws attention to content without conveying extra importance,
/// seriousness, or emphasis. Used for keywords, product names, or other spans of text
/// whose typical presentation is bold. Not for emphasis—use `<strong>` for that.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Keywords in a document
/// - Product names in reviews
/// - Lead-in words or phrases
/// - Drawing attention without semantic importance
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p><b>Note:</b> This feature is experimental.</p>
/// <p>The <b>iPhone 15</b> was released in September 2023.</p>
/// <p><b>Ingredients:</b> flour, sugar, eggs, butter.</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.22 The b element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-b-element)
pub struct B;
impl HtmlElement for B {
    const TAG: &'static str = "b";
}
impl FlowContent for B {}
impl PhrasingContent for B {}
impl PalpableContent for B {}

/// The `<u>` element - unarticulated annotation.
///
/// # Purpose
///
/// The `<u>` element represents a span of text with an unarticulated, though explicitly
/// rendered, non-textual annotation. Examples include labeling text as misspelled (like
/// spell-checker red underlines) or marking proper names in Chinese text.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Spelling errors in user input
/// - Proper names in Chinese text
/// - Unarticulated annotations
/// - Text requiring non-textual annotation
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>The word <u>recieve</u> is misspelled.</p>
/// <p>User wrote: <u>definately</u> (marked as misspelled)</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.23 The u element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-u-element)
pub struct U;
impl HtmlElement for U {
    const TAG: &'static str = "u";
}
impl FlowContent for U {}
impl PhrasingContent for U {}
impl PalpableContent for U {}

/// The `<mark>` element - highlighted or marked text.
///
/// # Purpose
///
/// The `<mark>` element represents text that is marked or highlighted for reference or
/// notation purposes, due to its relevance in the surrounding context. Think of it as
/// a highlighter pen marking in a document.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Search result highlighting
/// - Important passages or quotes
/// - Code or text being referenced
/// - Changes or updates in reviewed documents
/// - Key parts of quoted text
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>Search results for "HTML": The <mark>HTML</mark> specification defines...</p>
/// <p>Most important: <mark>Always save your work!</mark></p>
/// <blockquote>
///   Four score and seven years ago, <mark>our fathers brought forth</mark>...
/// </blockquote>
/// ```
///
/// # Accessibility
///
/// - Screen readers may announce marked content
/// - Typically rendered with yellow background
///
/// # WHATWG Specification
///
/// - [4.5.24 The mark element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-mark-element)
pub struct Mark;
impl HtmlElement for Mark {
    const TAG: &'static str = "mark";
}
impl FlowContent for Mark {}
impl PhrasingContent for Mark {}
impl PalpableContent for Mark {}

/// The `<bdi>` element - bidirectional isolate.
///
/// # Purpose
///
/// The `<bdi>` element isolates a span of text that might be formatted in a different
/// direction from other text around it. It's used when embedding user-generated content
/// or database values where the text direction is unknown.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - User-generated content (usernames, comments)
/// - Database values with unknown text direction
/// - Mixing left-to-right and right-to-left text
/// - Product names or IDs that may contain RTL characters
///
/// # Key Attributes
///
/// - Global attributes only (particularly `dir`)
///
/// # Example
///
/// ```html
/// <p>User <bdi>إيان</bdi> posted: "Hello world"</p>
/// <ul>
///   <li>User <bdi>jdoe</bdi>: 60 points</li>
///   <li>User <bdi>مستخدم123</bdi>: 50 points</li>
/// </ul>
/// ```
///
/// # Accessibility
///
/// - Prevents text direction issues in screen readers
/// - Maintains correct reading order
///
/// # WHATWG Specification
///
/// - [4.5.25 The bdi element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-bdi-element)
pub struct Bdi;
impl HtmlElement for Bdi {
    const TAG: &'static str = "bdi";
}
impl FlowContent for Bdi {}
impl PhrasingContent for Bdi {}
impl PalpableContent for Bdi {}

/// The `<bdo>` element - bidirectional text override.
///
/// # Purpose
///
/// The `<bdo>` element overrides the current directionality of text, forcing the text
/// within it to be rendered in a specific direction regardless of the Unicode bidirectional
/// algorithm. Requires the `dir` attribute.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Forcing specific text direction for demonstration
/// - Displaying text in reverse for special effects
/// - Overriding automatic text direction detection
///
/// # Key Attributes
///
/// - `dir`: Text direction—`"ltr"` (left-to-right) or `"rtl"` (right-to-left) (required)
/// - Global attributes
///
/// # Example
///
/// ```html
/// <p>This text contains <bdo dir="rtl">reversed text</bdo> in the middle.</p>
/// <p><bdo dir="ltr">This is forced left-to-right</bdo></p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.26 The bdo element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-bdo-element)
pub struct Bdo;
impl HtmlElement for Bdo {
    const TAG: &'static str = "bdo";
}
impl FlowContent for Bdo {}
impl PhrasingContent for Bdo {}
impl PalpableContent for Bdo {}

/// The `<span>` element - generic inline container.
///
/// # Purpose
///
/// The `<span>` element is a generic inline container for phrasing content. It has no
/// semantic meaning and should be used only when no other semantic element is appropriate.
/// Primarily used for styling with CSS or as a target for JavaScript.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content (if it has content)
///
/// # Permitted Content Model
///
/// - Phrasing content
///
/// # Common Use Cases
///
/// - Styling parts of text with CSS
/// - JavaScript manipulation targets
/// - Applying classes or IDs to inline content
/// - Wrapping inline content when no semantic element fits
///
/// # Key Attributes
///
/// - Global attributes only (commonly `class`, `id`, `style`)
///
/// # Example
///
/// ```html
/// <p>The price is <span class="price">$29.99</span></p>
/// <p>Status: <span class="status-active">Active</span></p>
/// <p>Call us at <span id="phone-number">555-1234</span></p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.27 The span element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-span-element)
pub struct Span;
impl HtmlElement for Span {
    const TAG: &'static str = "span";
}
impl FlowContent for Span {}
impl PhrasingContent for Span {}
impl PalpableContent for Span {}

/// The `<br>` element - line break.
///
/// # Purpose
///
/// The `<br>` element produces a line break in text. It's useful for situations where
/// line breaks are part of the content (poems, addresses) but should not be used for
/// spacing or layout—use CSS for that.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Line breaks in poems or verses
/// - Address formatting
/// - Breaking lines in signatures
/// - Line breaks that are part of content semantics
///
/// # Key Attributes
///
/// - Global attributes only (rarely used)
///
/// # Example
///
/// ```html
/// <p>
///   Roses are red,<br>
///   Violets are blue,<br>
///   HTML is great,<br>
///   And CSS too!
/// </p>
///
/// <address>
///   123 Main Street<br>
///   Anytown, CA 12345<br>
///   USA
/// </address>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.28 The br element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-br-element)
pub struct Br;
impl HtmlElement for Br {
    const TAG: &'static str = "br";
    const VOID: bool = true;
}
impl FlowContent for Br {}
impl PhrasingContent for Br {}

/// The `<wbr>` element - word break opportunity.
///
/// # Purpose
///
/// The `<wbr>` element represents a position within text where the browser may optionally
/// break a line, though its line-breaking rules would not otherwise create a break at that
/// position. Useful for long words or URLs.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Long URLs or file paths
/// - Compound words or technical terms
/// - Long strings without natural break points
/// - Email addresses
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <p>Visit http://this<wbr>.is<wbr>.a<wbr>.really<wbr>.long<wbr>.example<wbr>.com/</p>
/// <p>Pneumono<wbr>ultra<wbr>microscopic<wbr>silico<wbr>volcano<wbr>coniosis</p>
/// <p>Email: super<wbr>long<wbr>username<wbr>@<wbr>example.com</p>
/// ```
///
/// # WHATWG Specification
///
/// - [4.5.29 The wbr element](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-wbr-element)
pub struct Wbr;
impl HtmlElement for Wbr {
    const TAG: &'static str = "wbr";
    const VOID: bool = true;
}
impl FlowContent for Wbr {}
impl PhrasingContent for Wbr {}

// =============================================================================
// Image and Multimedia
// =============================================================================

/// The `<img>` element - image.
pub struct Img;
impl HtmlElement for Img {
    const TAG: &'static str = "img";
    const VOID: bool = true;
}
impl FlowContent for Img {}
impl PhrasingContent for Img {}
impl EmbeddedContent for Img {}
impl PalpableContent for Img {}

/// The `<picture>` element - responsive images.
pub struct Picture;
impl HtmlElement for Picture {
    const TAG: &'static str = "picture";
}
impl FlowContent for Picture {}
impl PhrasingContent for Picture {}
impl EmbeddedContent for Picture {}
impl PalpableContent for Picture {}

/// The `<source>` element - media source.
pub struct Source;
impl HtmlElement for Source {
    const TAG: &'static str = "source";
    const VOID: bool = true;
}

/// The `<audio>` element - audio content.
pub struct Audio;
impl HtmlElement for Audio {
    const TAG: &'static str = "audio";
}
impl FlowContent for Audio {}
impl PhrasingContent for Audio {}
impl EmbeddedContent for Audio {}
impl PalpableContent for Audio {}

/// The `<video>` element - video content.
pub struct Video;
impl HtmlElement for Video {
    const TAG: &'static str = "video";
}
impl FlowContent for Video {}
impl PhrasingContent for Video {}
impl EmbeddedContent for Video {}
impl InteractiveContent for Video {}
impl PalpableContent for Video {}

/// The `<track>` element - timed text track.
pub struct Track;
impl HtmlElement for Track {
    const TAG: &'static str = "track";
    const VOID: bool = true;
}

/// The `<map>` element - image map.
pub struct Map;
impl HtmlElement for Map {
    const TAG: &'static str = "map";
}
impl FlowContent for Map {}
impl PhrasingContent for Map {}
impl PalpableContent for Map {}

/// The `<area>` element - image map area.
pub struct Area;
impl HtmlElement for Area {
    const TAG: &'static str = "area";
    const VOID: bool = true;
}
impl FlowContent for Area {}
impl PhrasingContent for Area {}

// =============================================================================
// Embedded Content
// =============================================================================

/// The `<iframe>` element - nested browsing context.
pub struct Iframe;
impl HtmlElement for Iframe {
    const TAG: &'static str = "iframe";
}
impl FlowContent for Iframe {}
impl PhrasingContent for Iframe {}
impl EmbeddedContent for Iframe {}
impl InteractiveContent for Iframe {}
impl PalpableContent for Iframe {}

/// The `<embed>` element - external content.
pub struct Embed;
impl HtmlElement for Embed {
    const TAG: &'static str = "embed";
    const VOID: bool = true;
}
impl FlowContent for Embed {}
impl PhrasingContent for Embed {}
impl EmbeddedContent for Embed {}
impl InteractiveContent for Embed {}
impl PalpableContent for Embed {}

/// The `<object>` element - external resource.
pub struct Object;
impl HtmlElement for Object {
    const TAG: &'static str = "object";
}
impl FlowContent for Object {}
impl PhrasingContent for Object {}
impl EmbeddedContent for Object {}
impl PalpableContent for Object {}

/// The `<param>` element - object parameter.
pub struct Param;
impl HtmlElement for Param {
    const TAG: &'static str = "param";
    const VOID: bool = true;
}

// =============================================================================
// SVG and MathML
// =============================================================================

/// The `<svg>` element - SVG graphics.
pub struct Svg;
impl HtmlElement for Svg {
    const TAG: &'static str = "svg";
}
impl FlowContent for Svg {}
impl PhrasingContent for Svg {}
impl EmbeddedContent for Svg {}
impl PalpableContent for Svg {}

/// The `<math>` element - MathML.
pub struct Math;
impl HtmlElement for Math {
    const TAG: &'static str = "math";
}
impl FlowContent for Math {}
impl PhrasingContent for Math {}
impl EmbeddedContent for Math {}
impl PalpableContent for Math {}

// =============================================================================
// Scripting
// =============================================================================

/// The `<script>` element - executable script.
pub struct Script;
impl HtmlElement for Script {
    const TAG: &'static str = "script";
}
impl MetadataContent for Script {}
impl FlowContent for Script {}
impl PhrasingContent for Script {}
impl ScriptSupporting for Script {}

/// The `<noscript>` element - fallback content.
pub struct Noscript;
impl HtmlElement for Noscript {
    const TAG: &'static str = "noscript";
}
impl MetadataContent for Noscript {}
impl FlowContent for Noscript {}
impl PhrasingContent for Noscript {}

/// The `<template>` element - template.
pub struct Template;
impl HtmlElement for Template {
    const TAG: &'static str = "template";
}
impl MetadataContent for Template {}
impl FlowContent for Template {}
impl PhrasingContent for Template {}
impl ScriptSupporting for Template {}

/// The `<slot>` element - shadow DOM slot.
pub struct Slot;
impl HtmlElement for Slot {
    const TAG: &'static str = "slot";
}
impl FlowContent for Slot {}
impl PhrasingContent for Slot {}

/// The `<canvas>` element - graphics canvas.
pub struct Canvas;
impl HtmlElement for Canvas {
    const TAG: &'static str = "canvas";
}
impl FlowContent for Canvas {}
impl PhrasingContent for Canvas {}
impl EmbeddedContent for Canvas {}
impl PalpableContent for Canvas {}

// =============================================================================
// Table Content
// =============================================================================

/// The `<table>` element - table.
pub struct Table;
impl HtmlElement for Table {
    const TAG: &'static str = "table";
}
impl FlowContent for Table {}
impl PalpableContent for Table {}

/// The `<caption>` element - table caption.
pub struct Caption;
impl HtmlElement for Caption {
    const TAG: &'static str = "caption";
}

/// The `<colgroup>` element - column group.
pub struct Colgroup;
impl HtmlElement for Colgroup {
    const TAG: &'static str = "colgroup";
}

/// The `<col>` element - column.
pub struct Col;
impl HtmlElement for Col {
    const TAG: &'static str = "col";
    const VOID: bool = true;
}

/// The `<thead>` element - table header.
pub struct Thead;
impl HtmlElement for Thead {
    const TAG: &'static str = "thead";
}

/// The `<tbody>` element - table body.
pub struct Tbody;
impl HtmlElement for Tbody {
    const TAG: &'static str = "tbody";
}

/// The `<tfoot>` element - table footer.
pub struct Tfoot;
impl HtmlElement for Tfoot {
    const TAG: &'static str = "tfoot";
}

/// The `<tr>` element - table row.
pub struct Tr;
impl HtmlElement for Tr {
    const TAG: &'static str = "tr";
}

/// The `<th>` element - table header cell.
pub struct Th;
impl HtmlElement for Th {
    const TAG: &'static str = "th";
}

/// The `<td>` element - table data cell.
pub struct Td;
impl HtmlElement for Td {
    const TAG: &'static str = "td";
}

// =============================================================================
// Forms
// =============================================================================

/// The `<form>` element - form.
pub struct Form;
impl HtmlElement for Form {
    const TAG: &'static str = "form";
}
impl FlowContent for Form {}
impl PalpableContent for Form {}

/// The `<label>` element - form label.
pub struct Label;
impl HtmlElement for Label {
    const TAG: &'static str = "label";
}
impl FlowContent for Label {}
impl PhrasingContent for Label {}
impl InteractiveContent for Label {}
impl PalpableContent for Label {}

/// The `<input>` element - form input.
pub struct Input;
impl HtmlElement for Input {
    const TAG: &'static str = "input";
    const VOID: bool = true;
}
impl FlowContent for Input {}
impl PhrasingContent for Input {}
impl InteractiveContent for Input {}
impl PalpableContent for Input {}

/// The `<button>` element - button.
pub struct Button;
impl HtmlElement for Button {
    const TAG: &'static str = "button";
}
impl FlowContent for Button {}
impl PhrasingContent for Button {}
impl InteractiveContent for Button {}
impl PalpableContent for Button {}

/// The `<select>` element - selection control.
pub struct Select;
impl HtmlElement for Select {
    const TAG: &'static str = "select";
}
impl FlowContent for Select {}
impl PhrasingContent for Select {}
impl InteractiveContent for Select {}
impl PalpableContent for Select {}

/// The `<datalist>` element - predefined options.
pub struct Datalist;
impl HtmlElement for Datalist {
    const TAG: &'static str = "datalist";
}
impl FlowContent for Datalist {}
impl PhrasingContent for Datalist {}

/// The `<optgroup>` element - option group.
pub struct Optgroup;
impl HtmlElement for Optgroup {
    const TAG: &'static str = "optgroup";
}

/// The `<option>` element - option.
pub struct Option_;
impl HtmlElement for Option_ {
    const TAG: &'static str = "option";
}

/// The `<textarea>` element - multiline text input.
pub struct Textarea;
impl HtmlElement for Textarea {
    const TAG: &'static str = "textarea";
}
impl FlowContent for Textarea {}
impl PhrasingContent for Textarea {}
impl InteractiveContent for Textarea {}
impl PalpableContent for Textarea {}

/// The `<output>` element - calculation result.
pub struct Output;
impl HtmlElement for Output {
    const TAG: &'static str = "output";
}
impl FlowContent for Output {}
impl PhrasingContent for Output {}
impl PalpableContent for Output {}

/// The `<progress>` element - progress indicator.
pub struct Progress;
impl HtmlElement for Progress {
    const TAG: &'static str = "progress";
}
impl FlowContent for Progress {}
impl PhrasingContent for Progress {}
impl PalpableContent for Progress {}

/// The `<meter>` element - scalar measurement.
pub struct Meter;
impl HtmlElement for Meter {
    const TAG: &'static str = "meter";
}
impl FlowContent for Meter {}
impl PhrasingContent for Meter {}
impl PalpableContent for Meter {}

/// The `<fieldset>` element - form field group.
pub struct Fieldset;
impl HtmlElement for Fieldset {
    const TAG: &'static str = "fieldset";
}
impl FlowContent for Fieldset {}
impl PalpableContent for Fieldset {}

/// The `<legend>` element - fieldset caption.
pub struct Legend;
impl HtmlElement for Legend {
    const TAG: &'static str = "legend";
}

// =============================================================================
// Interactive Elements
// =============================================================================

/// The `<details>` element - disclosure widget.
pub struct Details;
impl HtmlElement for Details {
    const TAG: &'static str = "details";
}
impl FlowContent for Details {}
impl InteractiveContent for Details {}
impl PalpableContent for Details {}

/// The `<summary>` element - details summary.
pub struct Summary;
impl HtmlElement for Summary {
    const TAG: &'static str = "summary";
}

/// The `<dialog>` element - dialog box.
pub struct Dialog;
impl HtmlElement for Dialog {
    const TAG: &'static str = "dialog";
}
impl FlowContent for Dialog {}

// =============================================================================
// Web Components
// =============================================================================

// Note: Custom elements are handled separately

// =============================================================================
// Deprecated/Obsolete Elements (included for completeness)
// =============================================================================

/// The `<del>` element - deleted text.
pub struct Del;
impl HtmlElement for Del {
    const TAG: &'static str = "del";
}
impl FlowContent for Del {}
impl PhrasingContent for Del {}

/// The `<ins>` element - inserted text.
pub struct Ins;
impl HtmlElement for Ins {
    const TAG: &'static str = "ins";
}
impl FlowContent for Ins {}
impl PhrasingContent for Ins {}

// =============================================================================
// Content Model Implementations
// https://html.spec.whatwg.org/multipage/dom.html#content-models
// =============================================================================

// -----------------------------------------------------------------------------
// Text can be contained by phrasing content elements
// -----------------------------------------------------------------------------

impl CanContain<Text> for Span {}
impl CanContain<Text> for A {}
impl CanContain<Text> for Em {}
impl CanContain<Text> for Strong {}
impl CanContain<Text> for Small {}
impl CanContain<Text> for S {}
impl CanContain<Text> for Cite {}
impl CanContain<Text> for Q {}
impl CanContain<Text> for Dfn {}
impl CanContain<Text> for Abbr {}
impl CanContain<Text> for Code {}
impl CanContain<Text> for Var {}
impl CanContain<Text> for Samp {}
impl CanContain<Text> for Kbd {}
impl CanContain<Text> for Sub {}
impl CanContain<Text> for Sup {}
impl CanContain<Text> for I {}
impl CanContain<Text> for B {}
impl CanContain<Text> for U {}
impl CanContain<Text> for Mark {}
impl CanContain<Text> for Bdi {}
impl CanContain<Text> for Bdo {}
impl CanContain<Text> for Data {}
impl CanContain<Text> for Time {}

// Text in block-level elements
impl CanContain<Text> for P {}
impl CanContain<Text> for H1 {}
impl CanContain<Text> for H2 {}
impl CanContain<Text> for H3 {}
impl CanContain<Text> for H4 {}
impl CanContain<Text> for H5 {}
impl CanContain<Text> for H6 {}
impl CanContain<Text> for Pre {}
impl CanContain<Text> for Blockquote {}
impl CanContain<Text> for Li {}
impl CanContain<Text> for Dt {}
impl CanContain<Text> for Dd {}
impl CanContain<Text> for Figcaption {}
impl CanContain<Text> for Div {}
impl CanContain<Text> for Td {}
impl CanContain<Text> for Th {}
impl CanContain<Text> for Caption {}
impl CanContain<Text> for Label {}
impl CanContain<Text> for Legend {}
impl CanContain<Text> for Summary {}
impl CanContain<Text> for Title {}
impl CanContain<Text> for Option_ {}
impl CanContain<Text> for Textarea {}
impl CanContain<Text> for Button {}
impl CanContain<Text> for Script {}
impl CanContain<Text> for Style {}

// -----------------------------------------------------------------------------
// Document structure
// https://html.spec.whatwg.org/multipage/semantics.html
// -----------------------------------------------------------------------------

impl CanContain<Head> for Html {}
impl CanContain<Body> for Html {}

impl CanContain<Title> for Head {}
impl CanContain<Base> for Head {}
impl CanContain<Link> for Head {}
impl CanContain<Meta> for Head {}
impl CanContain<Style> for Head {}
impl CanContain<Script> for Head {}
impl CanContain<Noscript> for Head {}
impl CanContain<Template> for Head {}

// -----------------------------------------------------------------------------
// Flow content containers (can contain any flow content)
// -----------------------------------------------------------------------------

// Body can contain flow content
impl<T: FlowContent> CanContain<T> for Body {}

// Div can contain flow content
impl<T: FlowContent> CanContain<T> for Div {}

// Article can contain flow content
impl<T: FlowContent> CanContain<T> for Article {}

// Section can contain flow content
impl<T: FlowContent> CanContain<T> for Section {}

// Nav can contain flow content
impl<T: FlowContent> CanContain<T> for Nav {}

// Aside can contain flow content
impl<T: FlowContent> CanContain<T> for Aside {}

// Header can contain flow content
impl<T: FlowContent> CanContain<T> for Header {}

// Footer can contain flow content
impl<T: FlowContent> CanContain<T> for Footer {}

// Main can contain flow content
impl<T: FlowContent> CanContain<T> for Main {}

// Address can contain flow content
impl<T: FlowContent> CanContain<T> for Address {}

// Blockquote can contain flow content
impl<T: FlowContent> CanContain<T> for Blockquote {}

// Figure can contain flow content
impl<T: FlowContent> CanContain<T> for Figure {}

// Figcaption can contain flow content
impl<T: FlowContent> CanContain<T> for Figcaption {}

// Details can contain flow content
impl<T: FlowContent> CanContain<T> for Details {}
impl CanContain<Summary> for Details {}

// Dialog can contain flow content
impl<T: FlowContent> CanContain<T> for Dialog {}

// Search can contain flow content
impl<T: FlowContent> CanContain<T> for Search {}

// Form can contain flow content
impl<T: FlowContent> CanContain<T> for Form {}

// Fieldset can contain flow content
impl<T: FlowContent> CanContain<T> for Fieldset {}
impl CanContain<Legend> for Fieldset {}

// -----------------------------------------------------------------------------
// Phrasing content containers (can only contain phrasing content)
// https://html.spec.whatwg.org/multipage/dom.html#phrasing-content
// -----------------------------------------------------------------------------

// P can only contain phrasing content
impl<T: PhrasingContent> CanContain<T> for P {}

// Headings can only contain phrasing content
impl<T: PhrasingContent> CanContain<T> for H1 {}
impl<T: PhrasingContent> CanContain<T> for H2 {}
impl<T: PhrasingContent> CanContain<T> for H3 {}
impl<T: PhrasingContent> CanContain<T> for H4 {}
impl<T: PhrasingContent> CanContain<T> for H5 {}
impl<T: PhrasingContent> CanContain<T> for H6 {}

// Pre can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Pre {}

// Span can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Span {}

// A can contain phrasing content (transparent, but simplified here)
impl<T: PhrasingContent> CanContain<T> for A {}

// Em, Strong, etc. can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Em {}
impl<T: PhrasingContent> CanContain<T> for Strong {}
impl<T: PhrasingContent> CanContain<T> for Small {}
impl<T: PhrasingContent> CanContain<T> for S {}
impl<T: PhrasingContent> CanContain<T> for Cite {}
impl<T: PhrasingContent> CanContain<T> for Q {}
impl<T: PhrasingContent> CanContain<T> for Dfn {}
impl<T: PhrasingContent> CanContain<T> for Abbr {}
impl<T: PhrasingContent> CanContain<T> for Code {}
impl<T: PhrasingContent> CanContain<T> for Var {}
impl<T: PhrasingContent> CanContain<T> for Samp {}
impl<T: PhrasingContent> CanContain<T> for Kbd {}
impl<T: PhrasingContent> CanContain<T> for Sub {}
impl<T: PhrasingContent> CanContain<T> for Sup {}
impl<T: PhrasingContent> CanContain<T> for I {}
impl<T: PhrasingContent> CanContain<T> for B {}
impl<T: PhrasingContent> CanContain<T> for U {}
impl<T: PhrasingContent> CanContain<T> for Mark {}
impl<T: PhrasingContent> CanContain<T> for Bdi {}
impl<T: PhrasingContent> CanContain<T> for Bdo {}
impl<T: PhrasingContent> CanContain<T> for Data {}
impl<T: PhrasingContent> CanContain<T> for Time {}
impl<T: PhrasingContent> CanContain<T> for Del {}
impl<T: PhrasingContent> CanContain<T> for Ins {}

// Ruby annotation
impl<T: PhrasingContent> CanContain<T> for Ruby {}
impl CanContain<Rt> for Ruby {}
impl CanContain<Rp> for Ruby {}

// Label can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Label {}

// Output can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Output {}

// Legend can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Legend {}

// Summary can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Summary {}

// Button can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Button {}

// Progress and Meter can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Progress {}
impl<T: PhrasingContent> CanContain<T> for Meter {}

// -----------------------------------------------------------------------------
// List content models
// https://html.spec.whatwg.org/multipage/grouping-content.html
// -----------------------------------------------------------------------------

// Ul can only contain Li (and script-supporting elements)
impl CanContain<Li> for Ul {}
impl CanContain<Script> for Ul {}
impl CanContain<Template> for Ul {}

// Ol can only contain Li (and script-supporting elements)
impl CanContain<Li> for Ol {}
impl CanContain<Script> for Ol {}
impl CanContain<Template> for Ol {}

// Menu can only contain Li (and script-supporting elements)
impl CanContain<Li> for Menu {}
impl CanContain<Script> for Menu {}
impl CanContain<Template> for Menu {}

// Li can contain flow content
impl<T: FlowContent> CanContain<T> for Li {}

// Dl can contain Dt, Dd, and div (for grouping)
impl CanContain<Dt> for Dl {}
impl CanContain<Dd> for Dl {}
impl CanContain<Div> for Dl {}
impl CanContain<Script> for Dl {}
impl CanContain<Template> for Dl {}

// Dt can contain flow content (but no header, footer, sectioning, heading)
impl<T: PhrasingContent> CanContain<T> for Dt {}

// Dd can contain flow content
impl<T: FlowContent> CanContain<T> for Dd {}

// -----------------------------------------------------------------------------
// Table content model
// https://html.spec.whatwg.org/multipage/tables.html
// -----------------------------------------------------------------------------

// Table can contain caption, colgroup, thead, tbody, tfoot, tr
impl CanContain<Caption> for Table {}
impl CanContain<Colgroup> for Table {}
impl CanContain<Thead> for Table {}
impl CanContain<Tbody> for Table {}
impl CanContain<Tfoot> for Table {}
impl CanContain<Tr> for Table {}
impl CanContain<Script> for Table {}
impl CanContain<Template> for Table {}

// Caption can contain flow content (but no tables)
impl<T: FlowContent> CanContain<T> for Caption {}

// Colgroup can contain Col and Template
impl CanContain<Col> for Colgroup {}
impl CanContain<Template> for Colgroup {}

// Thead, Tbody, Tfoot can contain Tr
impl CanContain<Tr> for Thead {}
impl CanContain<Script> for Thead {}
impl CanContain<Template> for Thead {}

impl CanContain<Tr> for Tbody {}
impl CanContain<Script> for Tbody {}
impl CanContain<Template> for Tbody {}

impl CanContain<Tr> for Tfoot {}
impl CanContain<Script> for Tfoot {}
impl CanContain<Template> for Tfoot {}

// Tr can contain Th and Td
impl CanContain<Th> for Tr {}
impl CanContain<Td> for Tr {}
impl CanContain<Script> for Tr {}
impl CanContain<Template> for Tr {}

// Th can contain flow content
impl<T: FlowContent> CanContain<T> for Th {}

// Td can contain flow content
impl<T: FlowContent> CanContain<T> for Td {}

// Hgroup can contain headings and p
impl CanContain<H1> for Hgroup {}
impl CanContain<H2> for Hgroup {}
impl CanContain<H3> for Hgroup {}
impl CanContain<H4> for Hgroup {}
impl CanContain<H5> for Hgroup {}
impl CanContain<H6> for Hgroup {}
impl CanContain<P> for Hgroup {}
impl CanContain<Script> for Hgroup {}
impl CanContain<Template> for Hgroup {}

// -----------------------------------------------------------------------------
// Select and Datalist content model
// https://html.spec.whatwg.org/multipage/form-elements.html
// -----------------------------------------------------------------------------

// Select can contain Option and Optgroup
impl CanContain<Option_> for Select {}
impl CanContain<Optgroup> for Select {}
impl CanContain<Script> for Select {}
impl CanContain<Template> for Select {}

// Optgroup can contain Option
impl CanContain<Option_> for Optgroup {}
impl CanContain<Script> for Optgroup {}
impl CanContain<Template> for Optgroup {}

// Datalist can contain Option and phrasing content
impl CanContain<Option_> for Datalist {}
impl<T: PhrasingContent> CanContain<T> for Datalist {}

// -----------------------------------------------------------------------------
// Media content model
// https://html.spec.whatwg.org/multipage/media.html
// -----------------------------------------------------------------------------

// Audio can contain Source, Track, and flow content (fallback)
impl CanContain<Source> for Audio {}
impl CanContain<Track> for Audio {}

// Video can contain Source, Track, and flow content (fallback)
impl CanContain<Source> for Video {}
impl CanContain<Track> for Video {}

// Picture can contain Source and Img
impl CanContain<Source> for Picture {}
impl CanContain<Img> for Picture {}
impl CanContain<Script> for Picture {}
impl CanContain<Template> for Picture {}

// Map can contain phrasing content (including Area which implements PhrasingContent)
impl<T: PhrasingContent> CanContain<T> for Map {}

// Object can contain Param and flow content (fallback)
impl CanContain<Param> for Object {}
impl<T: FlowContent> CanContain<T> for Object {}

// Canvas can contain flow content (fallback)
impl<T: FlowContent> CanContain<T> for Canvas {}

// Noscript can contain various content depending on context
impl<T: FlowContent> CanContain<T> for Noscript {}

// Template can contain anything (it's transparent content model)
// Using FlowContent covers most use cases; metadata elements typically go in Head
impl<T: FlowContent> CanContain<T> for Template {}

// Slot can contain phrasing content
impl<T: PhrasingContent> CanContain<T> for Slot {}

// Iframe content is loaded externally, but can have fallback
impl<T: FlowContent> CanContain<T> for Iframe {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_tags() {
        assert_eq!(Div::TAG, "div");
        assert_eq!(Span::TAG, "span");
        assert_eq!(A::TAG, "a");
        assert_eq!(Img::TAG, "img");
        assert_eq!(Table::TAG, "table");
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_void_elements() {
        // Non-void elements
        assert!(!Div::VOID);
        assert!(!Span::VOID);
        // Void elements
        assert!(Img::VOID);
        assert!(Br::VOID);
        assert!(Hr::VOID);
        assert!(Input::VOID);
        assert!(Meta::VOID);
        assert!(Link::VOID);
    }

    #[test]
    fn test_content_categories() {
        // Test that elements implement correct traits
        fn is_flow<T: FlowContent>() {}
        fn is_phrasing<T: PhrasingContent>() {}
        fn is_sectioning<T: SectioningContent>() {}
        fn is_heading<T: HeadingContent>() {}
        fn is_embedded<T: EmbeddedContent>() {}
        fn is_interactive<T: InteractiveContent>() {}

        is_flow::<Div>();
        is_flow::<P>();
        is_flow::<Span>();
        is_phrasing::<Span>();
        is_phrasing::<A>();
        is_phrasing::<Em>();
        is_sectioning::<Article>();
        is_sectioning::<Section>();
        is_sectioning::<Nav>();
        is_heading::<H1>();
        is_heading::<H2>();
        is_embedded::<Img>();
        is_embedded::<Video>();
        is_embedded::<Iframe>();
        is_interactive::<A>();
        is_interactive::<Button>();
        is_interactive::<Input>();
    }

    #[test]
    fn test_can_contain() {
        // Test valid parent-child relationships
        fn valid<P: CanContain<C>, C>() {}

        // Document structure
        valid::<Html, Head>();
        valid::<Html, Body>();
        valid::<Head, Title>();
        valid::<Head, Meta>();
        valid::<Head, Link>();
        valid::<Head, Script>();

        // Flow content containers
        valid::<Body, Div>();
        valid::<Body, P>();
        valid::<Div, Div>();
        valid::<Div, Span>();
        valid::<Div, P>();
        valid::<Article, Section>();
        valid::<Section, Article>();

        // Phrasing content
        valid::<P, Span>();
        valid::<P, A>();
        valid::<P, Em>();
        valid::<Span, Strong>();
        valid::<A, Code>();

        // Lists
        valid::<Ul, Li>();
        valid::<Ol, Li>();
        valid::<Li, Div>();
        valid::<Li, P>();
        valid::<Dl, Dt>();
        valid::<Dl, Dd>();

        // Tables
        valid::<Table, Thead>();
        valid::<Table, Tbody>();
        valid::<Table, Tfoot>();
        valid::<Table, Tr>();
        valid::<Thead, Tr>();
        valid::<Tbody, Tr>();
        valid::<Tr, Th>();
        valid::<Tr, Td>();
        valid::<Td, Div>();
        valid::<Th, Span>();

        // Forms
        valid::<Form, Input>();
        valid::<Form, Button>();
        valid::<Form, Label>();
        valid::<Select, Option_>();
        valid::<Select, Optgroup>();
        valid::<Optgroup, Option_>();
        valid::<Fieldset, Legend>();

        // Media
        valid::<Picture, Source>();
        valid::<Picture, Img>();
        valid::<Audio, Source>();
        valid::<Video, Source>();

        // Text content
        valid::<Div, Text>();
        valid::<P, Text>();
        valid::<Span, Text>();
        valid::<H1, Text>();
        valid::<Button, Text>();
    }
}
