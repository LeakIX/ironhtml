//! # ironhtml-elements
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
//! use ironhtml_elements::{Div, Span, A, Img, HtmlElement};
//! use ironhtml_elements::{FlowContent, PhrasingContent, EmbeddedContent};
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
//! use ironhtml_elements::{Div, Span, A};
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
//! use ironhtml_elements::{Div, Span, Article, H1};
//! use ironhtml_elements::{FlowContent, PhrasingContent, SectioningContent, HeadingContent};
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
//! ### The `CanContain` Pattern
//!
//! Parent-child relationships use a binary trait pattern:
//!
//! ```rust
//! use ironhtml_elements::{Div, Span, Ul, Li, Table, Tr, Td, P, CanContain};
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
/// use ironhtml_elements::{CanContain, Div, Span, P, Ul, Li, Table, Tr, Td, Text};
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

/// The `<img>` element - embeds an image into the document.
///
/// # Purpose
///
/// The `<img>` element represents an image and its fallback text. It is used to embed
/// graphics, photographs, illustrations, diagrams, and other visual content into web pages.
/// Images are loaded asynchronously and become part of the document flow.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - Palpable Content
/// - If with `usemap` attribute: Interactive Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Displaying photographs and illustrations
/// - Showing logos and branding elements
/// - Embedding diagrams and infographics
/// - Creating image-based navigation elements
/// - Displaying user avatars and profile pictures
///
/// # Key Attributes
///
/// - `src`: URL of the image (required)
/// - `alt`: Alternative text description (required for accessibility)
/// - `width`: Intrinsic width in pixels
/// - `height`: Intrinsic height in pixels
/// - `loading`: Lazy loading behavior ("lazy" or "eager")
/// - `decoding`: Image decoding hint ("sync", "async", or "auto")
/// - `srcset`: Responsive image sources for different resolutions
/// - `sizes`: Media conditions for responsive images
/// - `crossorigin`: CORS settings for image fetching
/// - `usemap`: Associates image with a `<map>` element
/// - `ismap`: Indicates server-side image map
/// - `referrerpolicy`: Referrer policy for image requests
/// - `fetchpriority`: Hint for relative fetch priority
///
/// # Example
///
/// ```html
/// <!-- Basic image -->
/// <img src="/images/logo.png" alt="Company Logo" width="200" height="100">
///
/// <!-- Responsive image with srcset -->
/// <img src="/images/photo.jpg"
///      srcset="/images/photo-320w.jpg 320w,
///              /images/photo-640w.jpg 640w,
///              /images/photo-1024w.jpg 1024w"
///      sizes="(max-width: 320px) 280px,
///             (max-width: 640px) 600px,
///             1000px"
///      alt="Scenic landscape photograph">
///
/// <!-- Lazy-loaded image -->
/// <img src="/images/hero.jpg" alt="Hero banner" loading="lazy">
///
/// <!-- Image with decorative purpose -->
/// <img src="/images/decorative-divider.png" alt="" role="presentation">
/// ```
///
/// # Accessibility
///
/// - Always provide meaningful `alt` text describing the image content
/// - Use empty `alt=""` for decorative images
/// - Ensure alt text is concise yet descriptive (typically under 150 characters)
/// - Don't use phrases like "image of" or "picture of" in alt text
/// - For complex images (charts, diagrams), consider using `<figure>` with `<figcaption>`
/// - Ensure sufficient color contrast for images containing text
/// - Provide text alternatives for informational images
///
/// # WHATWG Specification
///
/// - [4.8.3 The img element](https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element)
pub struct Img;
impl HtmlElement for Img {
    const TAG: &'static str = "img";
    const VOID: bool = true;
}
impl FlowContent for Img {}
impl PhrasingContent for Img {}
impl EmbeddedContent for Img {}
impl PalpableContent for Img {}

/// The `<picture>` element - contains multiple image sources for responsive images.
///
/// # Purpose
///
/// The `<picture>` element provides a container for zero or more `<source>` elements and one
/// `<img>` element to offer alternative versions of an image for different display/device
/// scenarios. It enables art direction and format-based image selection, going beyond simple
/// resolution switching.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Zero or more `<source>` elements
/// - Followed by one `<img>` element
/// - Optionally intermixed with script-supporting elements
///
/// # Common Use Cases
///
/// - Art direction (different crops or compositions for different viewport sizes)
/// - Serving modern image formats with fallbacks (WebP, AVIF with JPEG fallback)
/// - Resolution switching based on device pixel ratio
/// - Bandwidth optimization by serving different image sizes
/// - Responsive design with different aspect ratios
///
/// # Key Attributes
///
/// - Global attributes only (attributes are primarily on child `<source>` and `<img>` elements)
///
/// # Example
///
/// ```html
/// <!-- Art direction: different images for different viewport sizes -->
/// <picture>
///   <source media="(min-width: 1024px)" srcset="/images/hero-wide.jpg">
///   <source media="(min-width: 768px)" srcset="/images/hero-medium.jpg">
///   <img src="/images/hero-narrow.jpg" alt="Hero image">
/// </picture>
///
/// <!-- Format selection: modern formats with fallback -->
/// <picture>
///   <source srcset="/images/photo.avif" type="image/avif">
///   <source srcset="/images/photo.webp" type="image/webp">
///   <img src="/images/photo.jpg" alt="Photograph">
/// </picture>
///
/// <!-- Combining media queries and formats -->
/// <picture>
///   <source media="(min-width: 768px)" srcset="/images/large.webp" type="image/webp">
///   <source media="(min-width: 768px)" srcset="/images/large.jpg">
///   <source srcset="/images/small.webp" type="image/webp">
///   <img src="/images/small.jpg" alt="Responsive image">
/// </picture>
/// ```
///
/// # Accessibility
///
/// - The `alt` attribute should be on the `<img>` element, not `<picture>`
/// - Ensure all image variations convey the same essential information
/// - Test that fallback images are appropriate when sources don't match
///
/// # WHATWG Specification
///
/// - [4.8.1 The picture element](https://html.spec.whatwg.org/multipage/embedded-content.html#the-picture-element)
pub struct Picture;
impl HtmlElement for Picture {
    const TAG: &'static str = "picture";
}
impl FlowContent for Picture {}
impl PhrasingContent for Picture {}
impl EmbeddedContent for Picture {}
impl PalpableContent for Picture {}

/// The `<source>` element - specifies media resources for `<picture>`, `<audio>`, and `<video>`.
///
/// # Purpose
///
/// The `<source>` element specifies multiple media resources for `<picture>`, `<audio>`, or
/// `<video>` elements. The browser selects the most appropriate source based on media queries,
/// format support, and other conditions. It enables responsive media delivery and format fallbacks.
///
/// # Content Categories
///
/// - None (used only within specific parent elements)
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Providing multiple video formats for cross-browser compatibility
/// - Offering different audio quality levels or formats
/// - Specifying image sources for different screen sizes in `<picture>`
/// - Delivering modern media formats with legacy fallbacks
/// - Bandwidth optimization with multiple quality levels
///
/// # Key Attributes
///
/// - `src`: URL of the media resource (for `<audio>` and `<video>`)
/// - `srcset`: Image URLs for responsive images (for `<picture>`)
/// - `type`: MIME type of the resource
/// - `media`: Media query for when this source applies
/// - `sizes`: Image sizes for different viewport widths (for `<picture>`)
/// - `width`: Intrinsic width for image sources
/// - `height`: Intrinsic height for image sources
///
/// # Example
///
/// ```html
/// <!-- Video with multiple formats -->
/// <video controls>
///   <source src="/video/movie.webm" type="video/webm">
///   <source src="/video/movie.mp4" type="video/mp4">
///   Your browser doesn't support the video element.
/// </video>
///
/// <!-- Audio with quality options -->
/// <audio controls>
///   <source src="/audio/music.opus" type="audio/opus">
///   <source src="/audio/music.ogg" type="audio/ogg">
///   <source src="/audio/music.mp3" type="audio/mpeg">
/// </audio>
///
/// <!-- Responsive images in picture element -->
/// <picture>
///   <source media="(min-width: 1200px)" srcset="/images/xl.jpg">
///   <source media="(min-width: 768px)" srcset="/images/lg.jpg">
///   <source srcset="/images/sm.jpg">
///   <img src="/images/fallback.jpg" alt="Description">
/// </picture>
///
/// <!-- Modern image formats with type hints -->
/// <picture>
///   <source srcset="/images/photo.avif" type="image/avif">
///   <source srcset="/images/photo.webp" type="image/webp">
///   <img src="/images/photo.jpg" alt="Photo">
/// </picture>
/// ```
///
/// # WHATWG Specification
///
/// - [4.8.2 The source element](https://html.spec.whatwg.org/multipage/embedded-content.html#the-source-element)
pub struct Source;
impl HtmlElement for Source {
    const TAG: &'static str = "source";
    const VOID: bool = true;
}

/// The `<audio>` element - embeds sound content into documents.
///
/// # Purpose
///
/// The `<audio>` element is used to embed audio content in documents. It may contain one or
/// more audio sources using nested `<source>` elements or the `src` attribute. The browser
/// will choose the most suitable source. It provides built-in playback controls and APIs for
/// programmatic control.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - If with `controls` attribute: Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Zero or more `<source>` elements
/// - Zero or more `<track>` elements
/// - Transparent content (fallback for browsers without audio support)
/// - No media elements among descendants
///
/// # Common Use Cases
///
/// - Music players and playlists
/// - Podcast players and audio articles
/// - Sound effects for interactive elements
/// - Audio feedback for user actions
/// - Background music or ambient sound
///
/// # Key Attributes
///
/// - `src`: URL of the audio file
/// - `controls`: Show playback controls
/// - `autoplay`: Automatically start playback (use with caution)
/// - `loop`: Loop the audio
/// - `muted`: Mute audio by default
/// - `preload`: Hint for loading strategy ("none", "metadata", "auto")
/// - `crossorigin`: CORS settings for audio fetching
/// - `volume`: Initial volume (0.0 to 1.0, set via JavaScript)
///
/// # Example
///
/// ```html
/// <!-- Basic audio player -->
/// <audio src="/audio/music.mp3" controls>
///   Your browser doesn't support the audio element.
/// </audio>
///
/// <!-- Multiple format sources -->
/// <audio controls>
///   <source src="/audio/podcast.opus" type="audio/opus">
///   <source src="/audio/podcast.ogg" type="audio/ogg; codecs=vorbis">
///   <source src="/audio/podcast.mp3" type="audio/mpeg">
///   <p>Your browser doesn't support HTML5 audio. <a href="/audio/podcast.mp3">Download</a></p>
/// </audio>
///
/// <!-- Looping background audio -->
/// <audio src="/audio/ambient.mp3" loop muted autoplay>
/// </audio>
///
/// <!-- Audio with preloading control -->
/// <audio src="/audio/effect.mp3" preload="none" id="sound-effect">
/// </audio>
/// ```
///
/// # Accessibility
///
/// - Provide transcripts for audio-only content
/// - Use captions/transcripts for important information conveyed via audio
/// - Don't use `autoplay` with sound, as it can be disruptive
/// - Ensure playback controls are keyboard accessible
/// - Consider users with hearing impairments
///
/// # WHATWG Specification
///
/// - [4.8.9 The audio element](https://html.spec.whatwg.org/multipage/media.html#the-audio-element)
pub struct Audio;
impl HtmlElement for Audio {
    const TAG: &'static str = "audio";
}
impl FlowContent for Audio {}
impl PhrasingContent for Audio {}
impl EmbeddedContent for Audio {}
impl PalpableContent for Audio {}

/// The `<video>` element - embeds video content into documents.
///
/// # Purpose
///
/// The `<video>` element embeds video content in documents. It may contain multiple video
/// sources via `<source>` elements or use the `src` attribute. Provides native playback
/// controls, poster images, and comprehensive JavaScript APIs for media control and monitoring.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Zero or more `<source>` elements
/// - Zero or more `<track>` elements
/// - Transparent content (fallback for browsers without video support)
/// - No media elements among descendants
///
/// # Common Use Cases
///
/// - Embedding tutorial and educational videos
/// - Product demonstrations and marketing videos
/// - Video backgrounds for hero sections
/// - Live streaming and recorded broadcasts
/// - Video conferencing and communication
///
/// # Key Attributes
///
/// - `src`: URL of the video file
/// - `controls`: Display video controls
/// - `autoplay`: Automatically start playback
/// - `loop`: Loop the video
/// - `muted`: Mute audio by default
/// - `poster`: URL of image to show before playback
/// - `preload`: Loading strategy ("none", "metadata", "auto")
/// - `width`: Display width in CSS pixels
/// - `height`: Display height in CSS pixels
/// - `playsinline`: Play inline on mobile devices
/// - `crossorigin`: CORS settings for video fetching
///
/// # Example
///
/// ```html
/// <!-- Basic video with controls -->
/// <video src="/videos/tutorial.mp4" controls width="640" height="360">
///   Your browser doesn't support the video element.
/// </video>
///
/// <!-- Multiple sources with poster -->
/// <video controls width="800" height="450" poster="/images/poster.jpg">
///   <source src="/videos/movie.webm" type="video/webm">
///   <source src="/videos/movie.mp4" type="video/mp4">
///   <track src="/captions/en.vtt" kind="subtitles" srclang="en" label="English">
///   <p>Your browser doesn't support HTML5 video. <a href="/videos/movie.mp4">Download</a></p>
/// </video>
///
/// <!-- Autoplay muted background video -->
/// <video autoplay loop muted playsinline class="bg-video">
///   <source src="/videos/background.webm" type="video/webm">
///   <source src="/videos/background.mp4" type="video/mp4">
/// </video>
///
/// <!-- Video with lazy loading -->
/// <video controls preload="none" poster="/images/video-thumb.jpg">
///   <source src="/videos/demo.mp4" type="video/mp4">
/// </video>
/// ```
///
/// # Accessibility
///
/// - Provide captions for deaf and hard-of-hearing users
/// - Include audio descriptions for blind and low-vision users
/// - Ensure video controls are keyboard accessible
/// - Provide transcripts for video content
/// - Use descriptive poster images
/// - Avoid autoplay with sound (can be disorienting)
/// - Ensure sufficient contrast for controls
///
/// # WHATWG Specification
///
/// - [4.8.9 The video element](https://html.spec.whatwg.org/multipage/media.html#the-video-element)
pub struct Video;
impl HtmlElement for Video {
    const TAG: &'static str = "video";
}
impl FlowContent for Video {}
impl PhrasingContent for Video {}
impl EmbeddedContent for Video {}
impl InteractiveContent for Video {}
impl PalpableContent for Video {}

/// The `<track>` element - specifies timed text tracks for media elements.
///
/// # Purpose
///
/// The `<track>` element provides text tracks for `<audio>` and `<video>` elements. These
/// tracks include subtitles, captions, descriptions, chapters, and metadata. Tracks are in
/// `WebVTT` format and can be displayed or processed programmatically to enhance media accessibility
/// and user experience.
///
/// # Content Categories
///
/// - None (used only within `<audio>` and `<video>` elements)
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Subtitles for foreign language translation
/// - Closed captions for deaf and hard-of-hearing users
/// - Audio descriptions for blind and low-vision users
/// - Chapter markers for video navigation
/// - Metadata tracks for programmatic access
///
/// # Key Attributes
///
/// - `kind`: Type of track ("subtitles", "captions", "descriptions", "chapters", "metadata")
/// - `src`: URL of the track file (`WebVTT` format, required)
/// - `srclang`: Language of the track text (required for subtitles)
/// - `label`: User-readable title for the track
/// - `default`: Enable this track by default
///
/// # Example
///
/// ```html
/// <!-- Video with multiple subtitle tracks -->
/// <video controls>
///   <source src="/videos/movie.mp4" type="video/mp4">
///   <track kind="subtitles" src="/subs/en.vtt" srclang="en" label="English" default>
///   <track kind="subtitles" src="/subs/es.vtt" srclang="es" label="Español">
///   <track kind="subtitles" src="/subs/fr.vtt" srclang="fr" label="Français">
/// </video>
///
/// <!-- Video with captions and descriptions -->
/// <video controls>
///   <source src="/videos/tutorial.mp4" type="video/mp4">
///   <track kind="captions" src="/captions/en.vtt" srclang="en" label="English Captions" default>
///   <track kind="descriptions" src="/descriptions/en.vtt" srclang="en" label="Audio Descriptions">
///   <track kind="chapters" src="/chapters/en.vtt" srclang="en" label="Chapters">
/// </video>
///
/// <!-- Audio with chapter markers -->
/// <audio controls>
///   <source src="/audio/podcast.mp3" type="audio/mpeg">
///   <track kind="chapters" src="/chapters/podcast.vtt" srclang="en" label="Episode Chapters">
///   <track kind="metadata" src="/metadata/podcast.vtt">
/// </audio>
/// ```
///
/// # Accessibility
///
/// - Use `kind="captions"` for accessibility (includes sound effects and speaker identification)
/// - Use `kind="subtitles"` for translation only
/// - Provide descriptions for visual content that isn't conveyed through audio
/// - Ensure track files are in proper `WebVTT` format
/// - Set appropriate default tracks based on user preferences
///
/// # WHATWG Specification
///
/// - [4.8.11 The track element](https://html.spec.whatwg.org/multipage/media.html#the-track-element)
pub struct Track;
impl HtmlElement for Track {
    const TAG: &'static str = "track";
    const VOID: bool = true;
}

/// The `<map>` element - defines an image map with clickable areas.
///
/// # Purpose
///
/// The `<map>` element defines an image map - a collection of clickable areas on an image.
/// Used with the `<area>` element to create hotspots on images that link to different
/// destinations. Images reference maps using the `usemap` attribute.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Transparent content (typically contains `<area>` elements)
///
/// # Common Use Cases
///
/// - Interactive geographic maps with region links
/// - Architectural floor plans with clickable rooms
/// - Organizational charts with clickable positions
/// - Product images with clickable component areas
/// - Educational diagrams with interactive sections
///
/// # Key Attributes
///
/// - `name`: Name referenced by `<img>` element's `usemap` attribute (required)
///
/// # Example
///
/// ```html
/// <!-- Image map for a world map -->
/// <img src="/images/world-map.jpg" alt="World Map" usemap="#world">
/// <map name="world">
///   <area shape="rect" coords="0,0,100,100" href="/regions/north-america" alt="North America">
///   <area shape="rect" coords="100,0,200,100" href="/regions/europe" alt="Europe">
///   <area shape="circle" coords="150,150,50" href="/regions/asia" alt="Asia">
///   <area shape="poly" coords="50,200,100,250,50,300" href="/regions/africa" alt="Africa">
/// </map>
///
/// <!-- Interactive floor plan -->
/// <img src="/images/floor-plan.png" alt="Office Floor Plan" usemap="#office-map">
/// <map name="office-map">
///   <area shape="rect" coords="10,10,110,60" href="/rooms/conference-a" alt="Conference Room A">
///   <area shape="rect" coords="120,10,220,60" href="/rooms/conference-b" alt="Conference Room B">
///   <area shape="rect" coords="10,70,110,120" href="/rooms/kitchen" alt="Kitchen">
/// </map>
///
/// <!-- Product feature map -->
/// <img src="/images/product.jpg" alt="Product Features" usemap="#features">
/// <map name="features">
///   <area shape="circle" coords="100,100,30" href="#feature-1" alt="Feature 1: Display">
///   <area shape="circle" coords="200,100,30" href="#feature-2" alt="Feature 2: Controls">
/// </map>
/// ```
///
/// # Accessibility
///
/// - Provide meaningful `alt` text for each `<area>` element
/// - Ensure keyboard navigation is available for all areas
/// - Consider providing a text-based alternative navigation
/// - Test with screen readers to ensure areas are announced properly
/// - Ensure sufficient clickable area size for touch interfaces
///
/// # WHATWG Specification
///
/// - [4.8.13 The map element](https://html.spec.whatwg.org/multipage/image-maps.html#the-map-element)
pub struct Map;
impl HtmlElement for Map {
    const TAG: &'static str = "map";
}
impl FlowContent for Map {}
impl PhrasingContent for Map {}
impl PalpableContent for Map {}

/// The `<area>` element - defines a clickable area within an image map.
///
/// # Purpose
///
/// The `<area>` element defines a hot-spot region on an image map and specifies the
/// hyperlink target for that region. Must be used as a descendant of a `<map>` element.
/// Supports rectangular, circular, and polygonal shapes.
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
/// - Creating clickable regions on geographic maps
/// - Interactive product diagrams with feature links
/// - Organizational charts with clickable positions
/// - Architectural plans with room navigation
/// - Educational diagrams with section links
///
/// # Key Attributes
///
/// - `shape`: Shape of the area ("rect", "circle", "poly", "default")
/// - `coords`: Coordinates defining the shape
/// - `href`: URL the area links to
/// - `alt`: Alternative text for the area (required)
/// - `target`: Browsing context for navigation
/// - `download`: Download the resource instead of navigating
/// - `ping`: URLs to ping when the link is followed
/// - `rel`: Relationship between current document and target
/// - `referrerpolicy`: Referrer policy for navigation
///
/// # Example
///
/// ```html
/// <!-- Rectangular areas -->
/// <map name="nav-map">
///   <area shape="rect" coords="0,0,100,50" href="/home" alt="Home">
///   <area shape="rect" coords="100,0,200,50" href="/about" alt="About">
///   <area shape="rect" coords="200,0,300,50" href="/contact" alt="Contact">
/// </map>
///
/// <!-- Circular area -->
/// <map name="button-map">
///   <area shape="circle" coords="150,150,75" href="/action" alt="Click to activate">
/// </map>
///
/// <!-- Polygon area (triangle) -->
/// <map name="complex-map">
///   <area shape="poly" coords="100,50,150,150,50,150" href="/info" alt="Information">
/// </map>
///
/// <!-- Default area (fallback for unmapped regions) -->
/// <map name="diagram">
///   <area shape="rect" coords="10,10,90,90" href="/section-1" alt="Section 1">
///   <area shape="default" href="/overview" alt="General overview">
/// </map>
/// ```
///
/// # Accessibility
///
/// - Always provide meaningful `alt` text describing the area's purpose
/// - Ensure `alt` text is concise and descriptive
/// - Make sure areas are large enough for touch interaction (minimum 44x44 pixels)
/// - Provide keyboard-accessible alternatives to image maps when possible
/// - Consider using semantic HTML links instead of image maps for better accessibility
///
/// # WHATWG Specification
///
/// - [4.8.14 The area element](https://html.spec.whatwg.org/multipage/image-maps.html#the-area-element)
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

/// The `<iframe>` element - embeds another HTML page within the current page.
///
/// # Purpose
///
/// The `<iframe>` element represents a nested browsing context, embedding another HTML page
/// into the current document. Creates an isolated environment for displaying external content,
/// third-party widgets, or sandboxed applications with controlled permissions and communication.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Nothing (fallback text for browsers that don't support iframes)
///
/// # Common Use Cases
///
/// - Embedding third-party content (maps, videos, social media)
/// - Displaying external widgets and plugins
/// - Creating sandboxed environments for untrusted content
/// - Loading advertisements in isolated contexts
/// - Embedding interactive tools and applications
///
/// # Key Attributes
///
/// - `src`: URL of the page to embed
/// - `srcdoc`: Inline HTML content to display
/// - `name`: Name for targeting the iframe
/// - `sandbox`: Security restrictions (empty or space-separated tokens)
/// - `allow`: Permissions policy (features the iframe can use)
/// - `width`: Width in CSS pixels
/// - `height`: Height in CSS pixels
/// - `loading`: Lazy loading ("lazy" or "eager")
/// - `referrerpolicy`: Referrer policy for requests
/// - `allowfullscreen`: Allow fullscreen mode
/// - `allowpaymentrequest`: Allow Payment Request API
///
/// # Example
///
/// ```html
/// <!-- Basic iframe embedding -->
/// <iframe src="https://example.com/widget" width="600" height="400">
///   Your browser doesn't support iframes.
/// </iframe>
///
/// <!-- Sandboxed iframe with restrictions -->
/// <iframe src="/untrusted.html"
///         sandbox="allow-scripts allow-same-origin"
///         width="100%" height="300">
/// </iframe>
///
/// <!-- Lazy-loaded iframe -->
/// <iframe src="https://www.youtube.com/embed/VIDEO_ID"
///         width="560" height="315"
///         loading="lazy"
///         allowfullscreen>
/// </iframe>
///
/// <!-- Iframe with inline content -->
/// <iframe srcdoc="<h1>Hello World</h1><p>This is inline HTML content.</p>"
///         width="400" height="200">
/// </iframe>
///
/// <!-- Iframe with permissions policy -->
/// <iframe src="/map.html"
///         allow="geolocation 'self'; camera 'none'"
///         width="800" height="600">
/// </iframe>
/// ```
///
/// # Accessibility
///
/// - Provide a descriptive `title` attribute for screen readers
/// - Ensure iframe content is keyboard accessible
/// - Consider whether iframe content should be directly in the page instead
/// - Test that embedded content meets accessibility standards
/// - Ensure iframe has meaningful fallback text
///
/// # WHATWG Specification
///
/// - [4.8.5 The iframe element](https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-iframe-element)
pub struct Iframe;
impl HtmlElement for Iframe {
    const TAG: &'static str = "iframe";
}
impl FlowContent for Iframe {}
impl PhrasingContent for Iframe {}
impl EmbeddedContent for Iframe {}
impl InteractiveContent for Iframe {}
impl PalpableContent for Iframe {}

/// The `<embed>` element - embeds external content at the specified point.
///
/// # Purpose
///
/// The `<embed>` element represents an integration point for external application or
/// interactive content, typically handled by a browser plugin. While historically used
/// for Flash and other plugins, it's now primarily used for embedding PDFs and other
/// plugin-based content. Modern alternatives like `<video>`, `<audio>`, and `<iframe>`
/// are preferred when applicable.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Embedding PDF documents inline
/// - Displaying plugin-based content (legacy)
/// - Embedding specialized media types
/// - Integration with native applications
/// - Displaying Flash content (legacy, deprecated)
///
/// # Key Attributes
///
/// - `src`: URL of the resource to embed (required)
/// - `type`: MIME type of the embedded content
/// - `width`: Width in CSS pixels
/// - `height`: Height in CSS pixels
/// - Any custom attributes for the plugin
///
/// # Example
///
/// ```html
/// <!-- Embedding a PDF document -->
/// <embed src="/documents/manual.pdf"
///        type="application/pdf"
///        width="800"
///        height="600">
///
/// <!-- Embedding with explicit dimensions -->
/// <embed src="/media/content.swf"
///        type="application/x-shockwave-flash"
///        width="640"
///        height="480">
///
/// <!-- Simple embed without type -->
/// <embed src="/files/document.pdf" width="100%" height="500">
///
/// <!-- Embed with custom parameters -->
/// <embed src="/plugin/app.plugin"
///        type="application/x-custom-plugin"
///        width="400"
///        height="300"
///        quality="high">
/// ```
///
/// # Accessibility
///
/// - Provide alternative content mechanisms when possible
/// - Ensure embedded content is keyboard accessible
/// - Consider using modern alternatives (`<iframe>`, `<video>`, `<audio>`)
/// - Test with assistive technologies
/// - Provide download links as fallback
///
/// # WHATWG Specification
///
/// - [4.8.6 The embed element](https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-embed-element)
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

/// The `<object>` element - embeds external resource as an object.
///
/// # Purpose
///
/// The `<object>` element represents an external resource, which can be treated as an image,
/// nested browsing context, or resource to be handled by a plugin. It provides fallback
/// content for when the object cannot be displayed. More flexible than `<embed>` with better
/// fallback support, but modern elements like `<video>`, `<audio>`, and `<iframe>` are often
/// more appropriate.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - If with `usemap` attribute: Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Zero or more `<param>` elements
/// - Followed by transparent content (fallback)
///
/// # Common Use Cases
///
/// - Embedding PDF documents with fallback
/// - Displaying SVG images with fallback content
/// - Plugin-based content with degradation path
/// - Embedding Flash content with alternatives (legacy)
/// - Nested browsing contexts with fallback options
///
/// # Key Attributes
///
/// - `data`: URL of the resource
/// - `type`: MIME type of the resource
/// - `name`: Name for form submission or scripting
/// - `width`: Width in CSS pixels
/// - `height`: Height in CSS pixels
/// - `usemap`: Associates with an image map
/// - `form`: Associates with a form element
/// - `typemustmatch`: Type attribute must match resource's type
///
/// # Example
///
/// ```html
/// <!-- PDF with fallback link -->
/// <object data="/documents/report.pdf"
///         type="application/pdf"
///         width="800"
///         height="600">
///   <p>Your browser doesn't support PDF viewing.
///      <a href="/documents/report.pdf">Download the PDF</a></p>
/// </object>
///
/// <!-- SVG with fallback image -->
/// <object data="/images/diagram.svg"
///         type="image/svg+xml"
///         width="400"
///         height="300">
///   <img src="/images/diagram.png" alt="Diagram">
/// </object>
///
/// <!-- Object with parameters -->
/// <object data="/media/animation.swf"
///         type="application/x-shockwave-flash"
///         width="640"
///         height="480">
///   <param name="quality" value="high">
///   <param name="autoplay" value="false">
///   <p>Flash content requires the Adobe Flash Player.</p>
/// </object>
///
/// <!-- Nested fallbacks -->
/// <object data="/video/movie.mp4" type="video/mp4">
///   <object data="/video/movie.ogv" type="video/ogg">
///     <p>Your browser doesn't support the video. <a href="/video/movie.mp4">Download</a></p>
///   </object>
/// </object>
/// ```
///
/// # Accessibility
///
/// - Provide meaningful fallback content
/// - Ensure embedded content is accessible
/// - Use `aria-label` or descriptive fallback text
/// - Test with assistive technologies
/// - Consider using semantic alternatives when available
///
/// # WHATWG Specification
///
/// - [4.8.7 The object element](https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-object-element)
pub struct Object;
impl HtmlElement for Object {
    const TAG: &'static str = "object";
}
impl FlowContent for Object {}
impl PhrasingContent for Object {}
impl EmbeddedContent for Object {}
impl PalpableContent for Object {}

/// The `<param>` element - defines parameters for an `<object>` element.
///
/// # Purpose
///
/// The `<param>` element defines parameters that are passed to the plugin or application
/// instantiated by an `<object>` element. Each parameter is specified as a name-value pair.
/// Must be a child of an `<object>` element and appear before any fallback content.
///
/// # Content Categories
///
/// - None (used only within `<object>` elements)
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Passing configuration to Flash content
/// - Setting plugin initialization parameters
/// - Configuring embedded applications
/// - Controlling playback settings
/// - Specifying display or behavior options
///
/// # Key Attributes
///
/// - `name`: Name of the parameter (required)
/// - `value`: Value of the parameter (required)
///
/// # Example
///
/// ```html
/// <!-- Flash parameters -->
/// <object data="/media/animation.swf" type="application/x-shockwave-flash">
///   <param name="quality" value="high">
///   <param name="wmode" value="transparent">
///   <param name="allowfullscreen" value="true">
///   <param name="flashvars" value="autoplay=false&volume=50">
/// </object>
///
/// <!-- Plugin configuration -->
/// <object data="/plugins/viewer.plugin" type="application/x-custom">
///   <param name="autostart" value="false">
///   <param name="volume" value="75">
///   <param name="controls" value="true">
/// </object>
///
/// <!-- Multiple parameters for video -->
/// <object data="/media/video.mp4" type="video/mp4">
///   <param name="autoplay" value="false">
///   <param name="loop" value="false">
///   <param name="controls" value="true">
///   <p>Your browser doesn't support this video format.</p>
/// </object>
/// ```
///
/// # WHATWG Specification
///
/// - [4.8.8 The param element](https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-param-element)
pub struct Param;
impl HtmlElement for Param {
    const TAG: &'static str = "param";
    const VOID: bool = true;
}

// =============================================================================
// SVG and MathML
// =============================================================================

/// The `<svg>` element - embeds SVG (Scalable Vector Graphics) content.
///
/// # Purpose
///
/// The `<svg>` element is a container for SVG graphics. SVG is an XML-based vector image
/// format for defining two-dimensional graphics with support for interactivity and animation.
/// SVG images scale without loss of quality and can be styled with CSS and manipulated with
/// JavaScript.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - SVG elements (follows SVG specification, not HTML)
///
/// # Common Use Cases
///
/// - Scalable icons and logos
/// - Data visualizations and charts
/// - Interactive graphics and diagrams
/// - Animated illustrations
/// - Responsive vector artwork
///
/// # Key Attributes
///
/// - `width`: Width of the SVG viewport
/// - `height`: Height of the SVG viewport
/// - `viewBox`: Define coordinate system and aspect ratio
/// - `preserveAspectRatio`: How to scale the viewBox
/// - `xmlns`: XML namespace (usually `"http://www.w3.org/2000/svg"`)
/// - Plus all SVG-specific attributes
///
/// # Example
///
/// ```html
/// <!-- Simple SVG circle -->
/// <svg width="100" height="100">
///   <circle cx="50" cy="50" r="40" fill="blue" />
/// </svg>
///
/// <!-- SVG with viewBox for responsive scaling -->
/// <svg viewBox="0 0 200 200" width="100%" height="auto">
///   <rect x="10" y="10" width="180" height="180" fill="lightblue" />
///   <circle cx="100" cy="100" r="50" fill="red" />
/// </svg>
///
/// <!-- Inline SVG icon -->
/// <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
///   <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
///   <polyline points="9 22 9 12 15 12 15 22"></polyline>
/// </svg>
///
/// <!-- SVG with accessibility -->
/// <svg role="img" aria-labelledby="logo-title" width="200" height="100">
///   <title id="logo-title">Company Logo</title>
///   <rect x="0" y="0" width="200" height="100" fill="#333"/>
///   <text x="100" y="55" text-anchor="middle" fill="white">LOGO</text>
/// </svg>
/// ```
///
/// # Accessibility
///
/// - Use `<title>` element inside SVG for accessible names
/// - Add `role="img"` for decorative or meaningful graphics
/// - Use `aria-labelledby` to reference title elements
/// - Provide `<desc>` for longer descriptions
/// - Use `aria-hidden="true"` for purely decorative SVGs
/// - Ensure sufficient color contrast
///
/// # WHATWG Specification
///
/// - [4.8.16 SVG](https://html.spec.whatwg.org/multipage/embedded-content.html#svg-0)
pub struct Svg;
impl HtmlElement for Svg {
    const TAG: &'static str = "svg";
}
impl FlowContent for Svg {}
impl PhrasingContent for Svg {}
impl EmbeddedContent for Svg {}
impl PalpableContent for Svg {}

/// The `<math>` element - embeds `MathML` (Mathematical Markup Language) content.
///
/// # Purpose
///
/// The `<math>` element is the top-level element for `MathML` content, used to embed
/// mathematical expressions and equations in HTML documents. `MathML` provides semantic
/// markup for mathematical notation, enabling proper rendering, accessibility, and
/// computational manipulation of mathematical expressions.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - `MathML` elements (follows `MathML` specification, not HTML)
///
/// # Common Use Cases
///
/// - Displaying mathematical equations and formulas
/// - Scientific and technical documentation
/// - Educational materials with math content
/// - Research papers and academic publications
/// - Interactive math applications
///
/// # Key Attributes
///
/// - `display`: "block" or "inline" (controls display mode)
/// - `xmlns`: XML namespace (usually `"http://www.w3.org/1998/Math/MathML"`)
/// - Plus all MathML-specific attributes
///
/// # Example
///
/// ```html
/// <!-- Inline math: Pythagorean theorem -->
/// <p>The Pythagorean theorem states that
/// <math>
///   <msup><mi>a</mi><mn>2</mn></msup>
///   <mo>+</mo>
///   <msup><mi>b</mi><mn>2</mn></msup>
///   <mo>=</mo>
///   <msup><mi>c</mi><mn>2</mn></msup>
/// </math>
/// </p>
///
/// <!-- Block math: Quadratic formula -->
/// <math display="block">
///   <mi>x</mi>
///   <mo>=</mo>
///   <mfrac>
///     <mrow>
///       <mo>−</mo><mi>b</mi>
///       <mo>±</mo>
///       <msqrt>
///         <msup><mi>b</mi><mn>2</mn></msup>
///         <mo>−</mo>
///         <mn>4</mn><mi>a</mi><mi>c</mi>
///       </msqrt>
///     </mrow>
///     <mrow>
///       <mn>2</mn><mi>a</mi>
///     </mrow>
///   </mfrac>
/// </math>
///
/// <!-- Fraction notation -->
/// <math>
///   <mfrac>
///     <mn>1</mn>
///     <mn>2</mn>
///   </mfrac>
/// </math>
///
/// <!-- Complex expression with matrix -->
/// <math display="block">
///   <mrow>
///     <mo>[</mo>
///     <mtable>
///       <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
///       <mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr>
///     </mtable>
///     <mo>]</mo>
///   </mrow>
/// </math>
/// ```
///
/// # Accessibility
///
/// - `MathML` provides built-in semantic accessibility
/// - Screen readers can navigate and speak mathematical expressions
/// - Consider providing text alternatives for complex equations
/// - Ensure proper use of `MathML` semantic elements
/// - Test with assistive technologies that support `MathML`
///
/// # WHATWG Specification
///
/// - [4.8.16 MathML](https://html.spec.whatwg.org/multipage/embedded-content.html#mathml)
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

/// The `<script>` element - embeds or references executable code.
///
/// # Purpose
///
/// The `<script>` element embeds executable code or data, or references an external script
/// file. Primarily used for JavaScript, it enables dynamic behavior, interactivity, and
/// client-side functionality in web pages. Scripts can be inline or loaded from external files.
///
/// # Content Categories
///
/// - Metadata Content
/// - Flow Content
/// - Phrasing Content
/// - Script-supporting Element
///
/// # Permitted Content Model
///
/// - If no `src` attribute: inline script content matching the `type`
/// - If `src` attribute: no content or only comments and whitespace
///
/// # Common Use Cases
///
/// - Adding interactivity to web pages
/// - Manipulating the DOM dynamically
/// - Handling user events and input
/// - Making AJAX requests and fetching data
/// - Implementing client-side application logic
///
/// # Key Attributes
///
/// - `src`: URL of external script file
/// - `type`: MIME type of the script (default: "text/javascript")
/// - `async`: Execute asynchronously (for external scripts)
/// - `defer`: Defer execution until document parsing is complete
/// - `crossorigin`: CORS settings for script fetching
/// - `integrity`: Subresource integrity hash for security
/// - `referrerpolicy`: Referrer policy for script requests
/// - `nomodule`: Execute only in browsers that don't support ES modules
/// - `nonce`: Cryptographic nonce for Content Security Policy
///
/// # Example
///
/// ```html
/// <!-- External script -->
/// <script src="/js/app.js"></script>
///
/// <!-- External script with async loading -->
/// <script src="/js/analytics.js" async></script>
///
/// <!-- External script with deferred execution -->
/// <script src="/js/init.js" defer></script>
///
/// <!-- Inline script -->
/// <script>
///   console.log('Hello, World!');
///   document.addEventListener('DOMContentLoaded', function() {
///     // Initialize app
///   });
/// </script>
///
/// <!-- ES6 module -->
/// <script type="module">
///   import { init } from './modules/app.js';
///   init();
/// </script>
///
/// <!-- Script with integrity check -->
/// <script src="https://cdn.example.com/library.js"
///         integrity="sha384-ABC123..."
///         crossorigin="anonymous"></script>
///
/// <!-- JSON-LD structured data -->
/// <script type="application/ld+json">
/// {
///   "@context": "https://schema.org",
///   "@type": "Organization",
///   "name": "Example Company"
/// }
/// </script>
/// ```
///
/// # WHATWG Specification
///
/// - [4.12.1 The script element](https://html.spec.whatwg.org/multipage/scripting.html#the-script-element)
pub struct Script;
impl HtmlElement for Script {
    const TAG: &'static str = "script";
}
impl MetadataContent for Script {}
impl FlowContent for Script {}
impl PhrasingContent for Script {}
impl ScriptSupporting for Script {}

/// The `<noscript>` element - defines fallback content for when scripts are disabled.
///
/// # Purpose
///
/// The `<noscript>` element provides fallback content for users who have disabled scripts
/// or use browsers that don't support scripting. The content inside is only displayed when
/// scripting is unavailable. Useful for providing alternative content, instructions, or
/// degraded experiences.
///
/// # Content Categories
///
/// - Metadata Content (when used in `<head>`)
/// - Flow Content (when used in `<body>`)
/// - Phrasing Content (when used in `<body>`)
///
/// # Permitted Content Model
///
/// - When in `<head>`: `<link>`, `<style>`, and `<meta>` elements
/// - When in `<body>`: Transparent content (with restrictions)
///
/// # Common Use Cases
///
/// - Providing messages about enabling JavaScript
/// - Offering alternative navigation when scripts fail
/// - Displaying static content as fallback
/// - Showing contact information when forms require JavaScript
/// - Providing download links for content requiring scripts
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Message in head for styles -->
/// <head>
///   <noscript>
///     <style>
///       .js-only { display: none; }
///     </style>
///   </noscript>
/// </head>
///
/// <!-- Alternative content in body -->
/// <noscript>
///   <div class="alert">
///     <p>This website requires JavaScript to function properly.</p>
///     <p>Please enable JavaScript in your browser settings.</p>
///   </div>
/// </noscript>
///
/// <!-- Fallback for dynamic content -->
/// <div id="app">
///   <noscript>
///     <p>Our interactive application requires JavaScript.</p>
///     <p>You can still <a href="/static-version">view the static version</a>.</p>
///   </noscript>
/// </div>
///
/// <!-- Alternative form submission -->
/// <form id="ajax-form" action="/submit" method="post">
///   <noscript>
///     <p>JavaScript is disabled. Please use the traditional form submission.</p>
///     <input type="submit" value="Submit Form">
///   </noscript>
/// </form>
/// ```
///
/// # Accessibility
///
/// - Ensure noscript content is meaningful and helpful
/// - Provide clear instructions for enabling JavaScript if required
/// - Consider whether your site should work without JavaScript
/// - Test the experience with scripting disabled
///
/// # WHATWG Specification
///
/// - [4.12.2 The noscript element](https://html.spec.whatwg.org/multipage/scripting.html#the-noscript-element)
pub struct Noscript;
impl HtmlElement for Noscript {
    const TAG: &'static str = "noscript";
}
impl MetadataContent for Noscript {}
impl FlowContent for Noscript {}
impl PhrasingContent for Noscript {}

/// The `<template>` element - holds HTML content that is not rendered immediately.
///
/// # Purpose
///
/// The `<template>` element is used to declare fragments of HTML that can be cloned and
/// inserted into the document via JavaScript. Its content is not rendered when the page loads,
/// making it ideal for client-side templating. The content is parsed but inert until activated.
///
/// # Content Categories
///
/// - Metadata Content
/// - Flow Content
/// - Phrasing Content
/// - Script-supporting Element
///
/// # Permitted Content Model
///
/// - Anything (content is inert and stored in a `DocumentFragment`)
///
/// # Common Use Cases
///
/// - Client-side HTML templates
/// - Repeating UI patterns (list items, cards, etc.)
/// - Dynamic content generation
/// - Web components and custom elements
/// - Avoiding script-based string concatenation
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Template for list items -->
/// <template id="item-template">
///   <li class="item">
///     <h3 class="item-title"></h3>
///     <p class="item-description"></p>
///   </li>
/// </template>
///
/// <ul id="item-list"></ul>
///
/// <script>
///   const template = document.getElementById('item-template');
///   const list = document.getElementById('item-list');
///   
///   const clone = template.content.cloneNode(true);
///   clone.querySelector('.item-title').textContent = 'Title';
///   clone.querySelector('.item-description').textContent = 'Description';
///   list.appendChild(clone);
/// </script>
///
/// <!-- Card template -->
/// <template id="card-template">
///   <div class="card">
///     <img class="card-image" src="" alt="">
///     <div class="card-body">
///       <h4 class="card-title"></h4>
///       <p class="card-text"></p>
///       <a class="card-link" href="#">Learn more</a>
///     </div>
///   </div>
/// </template>
///
/// <!-- Table row template -->
/// <template id="row-template">
///   <tr>
///     <td class="col-name"></td>
///     <td class="col-email"></td>
///     <td class="col-role"></td>
///   </tr>
/// </template>
/// ```
///
/// # WHATWG Specification
///
/// - [4.12.3 The template element](https://html.spec.whatwg.org/multipage/scripting.html#the-template-element)
pub struct Template;
impl HtmlElement for Template {
    const TAG: &'static str = "template";
}
impl MetadataContent for Template {}
impl FlowContent for Template {}
impl PhrasingContent for Template {}
impl ScriptSupporting for Template {}

/// The `<slot>` element - defines a placeholder in a web component's shadow DOM.
///
/// # Purpose
///
/// The `<slot>` element is part of the Web Components technology suite. It creates a
/// placeholder inside a web component that users can fill with their own markup. Slots
/// enable flexible, reusable components where content can be projected from the light DOM
/// into the shadow DOM.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
///
/// # Permitted Content Model
///
/// - Transparent content (fallback content when slot is not filled)
///
/// # Common Use Cases
///
/// - Creating reusable web components
/// - Defining customizable areas in shadow DOM templates
/// - Building flexible UI component libraries
/// - Implementing compound components with multiple insertion points
/// - Providing default fallback content for empty slots
///
/// # Key Attributes
///
/// - `name`: Named slot identifier (unnamed slots are default slots)
///
/// # Example
///
/// ```html
/// <!-- Custom element template (in shadow DOM) -->
/// <template id="card-template">
///   <style>
///     .card { border: 1px solid #ccc; padding: 1rem; }
///     .card-header { font-weight: bold; }
///   </style>
///   <div class="card">
///     <div class="card-header">
///       <slot name="header">Default Header</slot>
///     </div>
///     <div class="card-body">
///       <slot>Default content</slot>
///     </div>
///     <div class="card-footer">
///       <slot name="footer"></slot>
///     </div>
///   </div>
/// </template>
///
/// <!-- Usage of the custom element -->
/// <my-card>
///   <span slot="header">Custom Header</span>
///   <p>This is the main content that goes into the default slot.</p>
///   <small slot="footer">Footer text</small>
/// </my-card>
///
/// <!-- Named slots with fallback -->
/// <custom-dialog>
///   <h2 slot="title">Confirmation</h2>
///   <p>Are you sure you want to proceed?</p>
///   <div slot="actions">
///     <button>Cancel</button>
///     <button>OK</button>
///   </div>
/// </custom-dialog>
/// ```
///
/// # WHATWG Specification
///
/// - [4.12.4 The slot element](https://html.spec.whatwg.org/multipage/scripting.html#the-slot-element)
pub struct Slot;
impl HtmlElement for Slot {
    const TAG: &'static str = "slot";
}
impl FlowContent for Slot {}
impl PhrasingContent for Slot {}

/// The `<canvas>` element - provides a bitmap drawing surface for graphics via JavaScript.
///
/// # Purpose
///
/// The `<canvas>` element provides a resolution-dependent bitmap canvas for drawing graphics
/// via JavaScript and the Canvas API. It can be used for rendering graphs, game graphics,
/// animations, photo composition, real-time video processing, and other visual images on the fly.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Embedded Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Transparent content (fallback for browsers without canvas support)
///
/// # Common Use Cases
///
/// - 2D games and interactive animations
/// - Data visualizations and charts
/// - Image editing and manipulation
/// - Real-time video effects and filters
/// - Drawing tools and diagramming applications
///
/// # Key Attributes
///
/// - `width`: Width of the canvas in CSS pixels (default: 300)
/// - `height`: Height of the canvas in CSS pixels (default: 150)
///
/// # Example
///
/// ```html
/// <!-- Basic canvas -->
/// <canvas id="myCanvas" width="800" height="600">
///   Your browser doesn't support the canvas element.
/// </canvas>
/// <script>
///   const ctx = document.getElementById('myCanvas').getContext('2d');
///   ctx.fillStyle = 'blue';
///   ctx.fillRect(10, 10, 100, 100);
/// </script>
///
/// <!-- Canvas for charts -->
/// <canvas id="chart" width="600" height="400" aria-label="Sales data chart">
///   <p>Sales data: Q1: $100k, Q2: $150k, Q3: $175k, Q4: $200k</p>
/// </canvas>
///
/// <!-- Game canvas -->
/// <canvas id="gameCanvas" width="1024" height="768">
///   <p>This game requires a browser with canvas support.</p>
/// </canvas>
///
/// <!-- High DPI canvas -->
/// <canvas id="hdCanvas" width="1600" height="1200" style="width: 800px; height: 600px;">
///   Fallback content for accessibility.
/// </canvas>
/// ```
///
/// # Accessibility
///
/// - Provide meaningful fallback content describing what the canvas shows
/// - Use `aria-label` or `aria-labelledby` to describe the canvas purpose
/// - For interactive canvases, ensure keyboard accessibility
/// - Consider providing alternative text-based representations of visual data
/// - Use ARIA live regions to announce dynamic changes
/// - Ensure canvas content has sufficient color contrast
///
/// # WHATWG Specification
///
/// - [4.12.5 The canvas element](https://html.spec.whatwg.org/multipage/canvas.html#the-canvas-element)
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

/// The `<table>` element - represents tabular data in rows and columns.
///
/// # Purpose
///
/// The `<table>` element represents data with more than one dimension in the form of a table.
/// Tables should be used for tabular data, not for layout purposes (use CSS for layout).
/// Provides semantic structure for organizing related information in rows and columns.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Optional `<caption>` element
/// - Zero or more `<colgroup>` elements
/// - Optional `<thead>` element
/// - Either: zero or more `<tbody>` elements, or one or more `<tr>` elements
/// - Optional `<tfoot>` element
///
/// # Common Use Cases
///
/// - Displaying datasets and spreadsheet-like data
/// - Pricing tables and comparison charts
/// - Financial reports and statistics
/// - Schedules and calendars
/// - Product specifications and feature comparisons
///
/// # Key Attributes
///
/// - Global attributes only (older attributes like `border` are obsolete)
///
/// # Example
///
/// ```html
/// <!-- Basic table -->
/// <table>
///   <thead>
///     <tr>
///       <th>Name</th>
///       <th>Age</th>
///       <th>City</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Alice</td>
///       <td>30</td>
///       <td>New York</td>
///     </tr>
///     <tr>
///       <td>Bob</td>
///       <td>25</td>
///       <td>Los Angeles</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Table with caption and footer -->
/// <table>
///   <caption>Quarterly Sales Report</caption>
///   <thead>
///     <tr>
///       <th>Quarter</th>
///       <th>Revenue</th>
///       <th>Growth</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Q1</td>
///       <td>$100,000</td>
///       <td>5%</td>
///     </tr>
///     <tr>
///       <td>Q2</td>
///       <td>$120,000</td>
///       <td>20%</td>
///     </tr>
///   </tbody>
///   <tfoot>
///     <tr>
///       <td>Total</td>
///       <td>$220,000</td>
///       <td>12.5%</td>
///     </tr>
///   </tfoot>
/// </table>
///
/// <!-- Complex table with column groups -->
/// <table>
///   <colgroup>
///     <col>
///     <col span="2" class="financial">
///   </colgroup>
///   <thead>
///     <tr>
///       <th>Product</th>
///       <th>Price</th>
///       <th>Stock</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Widget</td>
///       <td>$10</td>
///       <td>50</td>
///     </tr>
///   </tbody>
/// </table>
/// ```
///
/// # Accessibility
///
/// - Use `<th>` elements for headers with `scope` attribute
/// - Provide a `<caption>` for table context
/// - Use `headers` attribute for complex tables
/// - Ensure proper header association for screen readers
/// - Consider using `aria-describedby` for additional context
/// - Make tables responsive for mobile devices
///
/// # WHATWG Specification
///
/// - [4.9.1 The table element](https://html.spec.whatwg.org/multipage/tables.html#the-table-element)
pub struct Table;
impl HtmlElement for Table {
    const TAG: &'static str = "table";
}
impl FlowContent for Table {}
impl PalpableContent for Table {}

/// The `<caption>` element - represents the title of a table.
///
/// # Purpose
///
/// The `<caption>` element provides a title or caption for its parent `<table>`. It gives
/// users context about the table's content before they start reading the data. Must be the
/// first child of the table if present. Screen readers announce captions to help users
/// understand table purpose.
///
/// # Content Categories
///
/// - None (only valid as first child of `<table>`)
///
/// # Permitted Content Model
///
/// - Flow content (excluding table elements)
///
/// # Common Use Cases
///
/// - Providing descriptive titles for data tables
/// - Summarizing table content and purpose
/// - Adding context for screen reader users
/// - Labeling financial reports and statistics
/// - Titling comparison and pricing tables
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Simple caption -->
/// <table>
///   <caption>Employee Directory</caption>
///   <thead>
///     <tr>
///       <th>Name</th>
///       <th>Department</th>
///       <th>Email</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>John Doe</td>
///       <td>Engineering</td>
///       <td>john@example.com</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Caption with additional context -->
/// <table>
///   <caption>
///     <strong>Quarterly Sales Data</strong>
///     <br>
///     <small>Fiscal Year 2024</small>
///   </caption>
///   <thead>
///     <tr>
///       <th>Quarter</th>
///       <th>Sales</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Q1</td>
///       <td>$50,000</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Styled caption -->
/// <table>
///   <caption style="caption-side: bottom;">
///     Table 1: Customer satisfaction ratings by region
///   </caption>
///   <thead>
///     <tr>
///       <th>Region</th>
///       <th>Rating</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>North</td>
///       <td>4.5/5</td>
///     </tr>
///   </tbody>
/// </table>
/// ```
///
/// # Accessibility
///
/// - Always provide a caption for data tables
/// - Keep captions concise but descriptive
/// - Screen readers announce captions before table content
/// - Use CSS `caption-side` property for visual positioning
///
/// # WHATWG Specification
///
/// - [4.9.2 The caption element](https://html.spec.whatwg.org/multipage/tables.html#the-caption-element)
pub struct Caption;
impl HtmlElement for Caption {
    const TAG: &'static str = "caption";
}

/// The `<colgroup>` element - defines a group of columns in a table.
///
/// # Purpose
///
/// The `<colgroup>` element specifies a group of one or more columns in a table for formatting
/// purposes. It allows styling of entire columns without repeating styles on each cell. Can
/// contain `<col>` elements or use the `span` attribute to define column groups.
///
/// # Content Categories
///
/// - None (only valid within `<table>`, after `<caption>` and before table rows)
///
/// # Permitted Content Model
///
/// - If `span` attribute is present: empty
/// - Otherwise: zero or more `<col>` elements
///
/// # Common Use Cases
///
/// - Styling multiple columns with shared characteristics
/// - Grouping related columns semantically
/// - Setting widths for multiple columns at once
/// - Applying background colors to column groups
/// - Defining visibility for column groups
///
/// # Key Attributes
///
/// - `span`: Number of columns the group spans (if no `<col>` children)
///
/// # Example
///
/// ```html
/// <!-- Column group with span -->
/// <table>
///   <colgroup span="2" class="financial-data"></colgroup>
///   <colgroup></colgroup>
///   <thead>
///     <tr>
///       <th>Item</th>
///       <th>Price</th>
///       <th>Stock</th>
///     </tr>
///   </thead>
/// </table>
///
/// <!-- Column group with col elements -->
/// <table>
///   <colgroup>
///     <col class="name-col">
///     <col class="email-col">
///   </colgroup>
///   <colgroup>
///     <col class="role-col">
///   </colgroup>
///   <thead>
///     <tr>
///       <th>Name</th>
///       <th>Email</th>
///       <th>Role</th>
///     </tr>
///   </thead>
/// </table>
///
/// <!-- Styling column groups -->
/// <table>
///   <colgroup>
///     <col>
///   </colgroup>
///   <colgroup class="highlight">
///     <col span="2">
///   </colgroup>
///   <thead>
///     <tr>
///       <th>Product</th>
///       <th>Q1</th>
///       <th>Q2</th>
///     </tr>
///   </thead>
/// </table>
/// ```
///
/// # WHATWG Specification
///
/// - [4.9.3 The colgroup element](https://html.spec.whatwg.org/multipage/tables.html#the-colgroup-element)
pub struct Colgroup;
impl HtmlElement for Colgroup {
    const TAG: &'static str = "colgroup";
}

/// The `<col>` element - defines a column within a table.
///
/// # Purpose
///
/// The `<col>` element defines a column or a group of columns within a table. Used inside
/// `<colgroup>` to apply attributes and styles to entire columns without affecting individual
/// cells. Provides a way to style columns collectively.
///
/// # Content Categories
///
/// - None (only valid within `<colgroup>`)
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Setting column widths
/// - Applying styles to specific columns
/// - Defining visibility for individual columns
/// - Grouping columns with shared formatting
/// - Creating alternating column styles
///
/// # Key Attributes
///
/// - `span`: Number of consecutive columns this element represents (default: 1)
///
/// # Example
///
/// ```html
/// <!-- Individual column styling -->
/// <table>
///   <colgroup>
///     <col class="name-column">
///     <col class="data-column">
///     <col class="data-column">
///   </colgroup>
///   <thead>
///     <tr>
///       <th>Name</th>
///       <th>Value 1</th>
///       <th>Value 2</th>
///     </tr>
///   </thead>
/// </table>
///
/// <!-- Using span attribute -->
/// <table>
///   <colgroup>
///     <col>
///     <col span="2" class="numeric-columns">
///     <col>
///   </colgroup>
///   <thead>
///     <tr>
///       <th>Product</th>
///       <th>Price</th>
///       <th>Quantity</th>
///       <th>Notes</th>
///     </tr>
///   </thead>
/// </table>
///
/// <!-- Column widths -->
/// <table>
///   <colgroup>
///     <col style="width: 40%;">
///     <col style="width: 30%;">
///     <col style="width: 30%;">
///   </colgroup>
///   <thead>
///     <tr>
///       <th>Description</th>
///       <th>Category</th>
///       <th>Status</th>
///     </tr>
///   </thead>
/// </table>
/// ```
///
/// # WHATWG Specification
///
/// - [4.9.4 The col element](https://html.spec.whatwg.org/multipage/tables.html#the-col-element)
pub struct Col;
impl HtmlElement for Col {
    const TAG: &'static str = "col";
    const VOID: bool = true;
}

/// The `<thead>` element - groups header rows in a table.
///
/// # Purpose
///
/// The `<thead>` element groups one or more `<tr>` elements that contain table headers.
/// It defines the header section of a table, typically containing column labels. Browsers
/// can use this to enable scrolling of the table body independently of the header, and to
/// repeat headers when printing multi-page tables.
///
/// # Content Categories
///
/// - None (only valid within `<table>`)
///
/// # Permitted Content Model
///
/// - Zero or more `<tr>` elements
///
/// # Common Use Cases
///
/// - Defining column headers for tables
/// - Creating sticky headers that remain visible while scrolling
/// - Enabling header repetition in printed tables
/// - Semantically separating headers from data
/// - Styling table headers distinctly from data
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Basic table header -->
/// <table>
///   <thead>
///     <tr>
///       <th>Name</th>
///       <th>Email</th>
///       <th>Role</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>John Doe</td>
///       <td>john@example.com</td>
///       <td>Developer</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Multi-row header -->
/// <table>
///   <thead>
///     <tr>
///       <th rowspan="2">Name</th>
///       <th colspan="2">Contact</th>
///     </tr>
///     <tr>
///       <th>Email</th>
///       <th>Phone</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Alice</td>
///       <td>alice@example.com</td>
///       <td>555-0001</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Sticky header -->
/// <table>
///   <thead style="position: sticky; top: 0; background: white;">
///     <tr>
///       <th>Product</th>
///       <th>Price</th>
///       <th>Stock</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Widget</td>
///       <td>$10</td>
///       <td>50</td>
///     </tr>
///     <!-- Many more rows... -->
///   </tbody>
/// </table>
/// ```
///
/// # Accessibility
///
/// - Use `<th>` elements within `<thead>` for proper header semantics
/// - Add `scope` attributes to header cells for complex tables
/// - Ensure header text is descriptive and concise
///
/// # WHATWG Specification
///
/// - [4.9.6 The thead element](https://html.spec.whatwg.org/multipage/tables.html#the-thead-element)
pub struct Thead;
impl HtmlElement for Thead {
    const TAG: &'static str = "thead";
}

/// The `<tbody>` element - groups body content rows in a table.
///
/// # Purpose
///
/// The `<tbody>` element groups one or more `<tr>` elements as the body section of a table.
/// It represents the main data content, as opposed to headers (`<thead>`) and footers (`<tfoot>`).
/// Allows applying styles and behavior to the table body separately from headers and footers.
///
/// # Content Categories
///
/// - None (only valid within `<table>`)
///
/// # Permitted Content Model
///
/// - Zero or more `<tr>` elements
///
/// # Common Use Cases
///
/// - Separating table data from headers and footers
/// - Applying styles to the table body
/// - Enabling independent scrolling of table body
/// - Grouping data rows semantically
/// - Creating multiple body sections in complex tables
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Basic table body -->
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
///     <tr>
///       <td>Bob</td>
///       <td>25</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Multiple tbody sections -->
/// <table>
///   <thead>
///     <tr>
///       <th>Product</th>
///       <th>Price</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td colspan="2"><strong>Electronics</strong></td>
///     </tr>
///     <tr>
///       <td>Laptop</td>
///       <td>$999</td>
///     </tr>
///   </tbody>
///   <tbody>
///     <tr>
///       <td colspan="2"><strong>Clothing</strong></td>
///     </tr>
///     <tr>
///       <td>T-Shirt</td>
///       <td>$20</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Scrollable table body -->
/// <table>
///   <thead>
///     <tr>
///       <th>Date</th>
///       <th>Event</th>
///     </tr>
///   </thead>
///   <tbody style="display: block; max-height: 200px; overflow-y: scroll;">
///     <tr>
///       <td>2024-01-01</td>
///       <td>New Year</td>
///     </tr>
///     <!-- More rows... -->
///   </tbody>
/// </table>
/// ```
///
/// # WHATWG Specification
///
/// - [4.9.5 The tbody element](https://html.spec.whatwg.org/multipage/tables.html#the-tbody-element)
pub struct Tbody;
impl HtmlElement for Tbody {
    const TAG: &'static str = "tbody";
}

/// The `<tfoot>` element - groups footer rows in a table.
///
/// # Purpose
///
/// The `<tfoot>` element groups one or more `<tr>` elements that contain summary or footer
/// information for a table. Typically contains totals, summaries, or additional notes.
/// Like `<thead>`, it can be repeated when printing multi-page tables and can remain visible
/// during scrolling.
///
/// # Content Categories
///
/// - None (only valid within `<table>`)
///
/// # Permitted Content Model
///
/// - Zero or more `<tr>` elements
///
/// # Common Use Cases
///
/// - Displaying totals and summary calculations
/// - Adding footnotes or additional context
/// - Showing aggregate data for table columns
/// - Creating sticky footers for scrollable tables
/// - Providing supplementary information
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Table with totals footer -->
/// <table>
///   <thead>
///     <tr>
///       <th>Item</th>
///       <th>Quantity</th>
///       <th>Price</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Widget</td>
///       <td>5</td>
///       <td>$50</td>
///     </tr>
///     <tr>
///       <td>Gadget</td>
///       <td>3</td>
///       <td>$45</td>
///     </tr>
///   </tbody>
///   <tfoot>
///     <tr>
///       <th>Total</th>
///       <td>8</td>
///       <td>$95</td>
///     </tr>
///   </tfoot>
/// </table>
///
/// <!-- Footer with notes -->
/// <table>
///   <thead>
///     <tr>
///       <th>Product</th>
///       <th>Status</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Alpha</td>
///       <td>Available</td>
///     </tr>
///   </tbody>
///   <tfoot>
///     <tr>
///       <td colspan="2">
///         <small>* Prices subject to change</small>
///       </td>
///     </tr>
///   </tfoot>
/// </table>
///
/// <!-- Multiple footer rows -->
/// <table>
///   <thead>
///     <tr>
///       <th>Category</th>
///       <th>Amount</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Sales</td>
///       <td>$100,000</td>
///     </tr>
///   </tbody>
///   <tfoot>
///     <tr>
///       <th>Subtotal</th>
///       <td>$100,000</td>
///     </tr>
///     <tr>
///       <th>Tax (10%)</th>
///       <td>$10,000</td>
///     </tr>
///     <tr>
///       <th>Total</th>
///       <td>$110,000</td>
///     </tr>
///   </tfoot>
/// </table>
/// ```
///
/// # WHATWG Specification
///
/// - [4.9.7 The tfoot element](https://html.spec.whatwg.org/multipage/tables.html#the-tfoot-element)
pub struct Tfoot;
impl HtmlElement for Tfoot {
    const TAG: &'static str = "tfoot";
}

/// The `<tr>` element - defines a row of cells in a table.
///
/// # Purpose
///
/// The `<tr>` element represents a row of cells in a table. Each row contains one or more
/// `<th>` (header cell) or `<td>` (data cell) elements. Rows can be grouped within `<thead>`,
/// `<tbody>`, and `<tfoot>` elements for semantic structure.
///
/// # Content Categories
///
/// - None (only valid within `<table>`, `<thead>`, `<tbody>`, or `<tfoot>`)
///
/// # Permitted Content Model
///
/// - Zero or more `<td>` or `<th>` elements
/// - Optionally intermixed with script-supporting elements
///
/// # Common Use Cases
///
/// - Creating rows of data in tables
/// - Organizing tabular information horizontally
/// - Building spreadsheet-like structures
/// - Displaying lists of records
/// - Creating pricing and comparison tables
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Basic table rows -->
/// <table>
///   <thead>
///     <tr>
///       <th>Name</th>
///       <th>Age</th>
///       <th>City</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Alice</td>
///       <td>30</td>
///       <td>New York</td>
///     </tr>
///     <tr>
///       <td>Bob</td>
///       <td>25</td>
///       <td>San Francisco</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Row with mixed headers and data -->
/// <table>
///   <tbody>
///     <tr>
///       <th scope="row">Product A</th>
///       <td>$99</td>
///       <td>In Stock</td>
///     </tr>
///     <tr>
///       <th scope="row">Product B</th>
///       <td>$149</td>
///       <td>Out of Stock</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Row with colspan -->
/// <table>
///   <tr>
///     <td colspan="3">Full width cell</td>
///   </tr>
///   <tr>
///     <td>Cell 1</td>
///     <td>Cell 2</td>
///     <td>Cell 3</td>
///   </tr>
/// </table>
///
/// <!-- Alternating row styles -->
/// <table>
///   <tbody>
///     <tr class="odd">
///       <td>Row 1</td>
///       <td>Data</td>
///     </tr>
///     <tr class="even">
///       <td>Row 2</td>
///       <td>Data</td>
///     </tr>
///   </tbody>
/// </table>
/// ```
///
/// # WHATWG Specification
///
/// - [4.9.8 The tr element](https://html.spec.whatwg.org/multipage/tables.html#the-tr-element)
pub struct Tr;
impl HtmlElement for Tr {
    const TAG: &'static str = "tr";
}

/// The `<th>` element - defines a header cell in a table.
///
/// # Purpose
///
/// The `<th>` element represents a header cell in a table. It labels a row or column of data
/// cells, providing context for the information in the table. Header cells are typically
/// rendered with bold, centered text by default and are crucial for table accessibility.
///
/// # Content Categories
///
/// - None (only valid within `<tr>`)
///
/// # Permitted Content Model
///
/// - Flow content (excluding header, footer, sectioning content, and heading content)
///
/// # Common Use Cases
///
/// - Labeling table columns
/// - Labeling table rows
/// - Creating multi-level headers with rowspan/colspan
/// - Providing context for data cells
/// - Improving table accessibility for screen readers
///
/// # Key Attributes
///
/// - `scope`: Specifies cells the header relates to ("row", "col", "rowgroup", "colgroup")
/// - `colspan`: Number of columns the header spans
/// - `rowspan`: Number of rows the header spans
/// - `headers`: Space-separated list of other header cell IDs
/// - `abbr`: Abbreviated description of the header
///
/// # Example
///
/// ```html
/// <!-- Column headers -->
/// <table>
///   <thead>
///     <tr>
///       <th scope="col">Name</th>
///       <th scope="col">Email</th>
///       <th scope="col">Role</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>Alice</td>
///       <td>alice@example.com</td>
///       <td>Developer</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Row headers -->
/// <table>
///   <tbody>
///     <tr>
///       <th scope="row">Product A</th>
///       <td>$99</td>
///       <td>In Stock</td>
///     </tr>
///     <tr>
///       <th scope="row">Product B</th>
///       <td>$149</td>
///       <td>Out of Stock</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Multi-level headers -->
/// <table>
///   <thead>
///     <tr>
///       <th rowspan="2" scope="col">Name</th>
///       <th colspan="2" scope="colgroup">Scores</th>
///     </tr>
///     <tr>
///       <th scope="col">Math</th>
///       <th scope="col">Science</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <th scope="row">Alice</th>
///       <td>95</td>
///       <td>92</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Headers with abbreviations -->
/// <table>
///   <thead>
///     <tr>
///       <th scope="col" abbr="Temp">Temperature (°F)</th>
///       <th scope="col" abbr="Humid">Humidity (%)</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <td>72</td>
///       <td>65</td>
///     </tr>
///   </tbody>
/// </table>
/// ```
///
/// # Accessibility
///
/// - Always use `scope` attribute to clarify what the header labels
/// - Use "col" for column headers, "row" for row headers
/// - Use "colgroup" or "rowgroup" for headers spanning groups
/// - Provide `abbr` for long header text to aid screen readers
/// - Ensure header text is concise and descriptive
///
/// # WHATWG Specification
///
/// - [4.9.10 The th element](https://html.spec.whatwg.org/multipage/tables.html#the-th-element)
pub struct Th;
impl HtmlElement for Th {
    const TAG: &'static str = "th";
}

/// The `<td>` element - defines a data cell in a table.
///
/// # Purpose
///
/// The `<td>` element represents a data cell in a table. It contains the actual data values
/// within table rows. Can span multiple rows or columns using `rowspan` and `colspan` attributes.
/// Distinguished from `<th>` header cells which label data.
///
/// # Content Categories
///
/// - Sectioning Root
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Displaying data values in tables
/// - Creating spreadsheet cells
/// - Showing individual records in datasets
/// - Building data grids and matrices
/// - Presenting structured information
///
/// # Key Attributes
///
/// - `colspan`: Number of columns the cell spans
/// - `rowspan`: Number of rows the cell spans
/// - `headers`: Space-separated list of header cell IDs this cell relates to
///
/// # Example
///
/// ```html
/// <!-- Basic data cells -->
/// <table>
///   <tr>
///     <th>Name</th>
///     <th>Score</th>
///   </tr>
///   <tr>
///     <td>Alice</td>
///     <td>95</td>
///   </tr>
///   <tr>
///     <td>Bob</td>
///     <td>87</td>
///   </tr>
/// </table>
///
/// <!-- Cells with colspan -->
/// <table>
///   <tr>
///     <td colspan="2">This cell spans two columns</td>
///   </tr>
///   <tr>
///     <td>Column 1</td>
///     <td>Column 2</td>
///   </tr>
/// </table>
///
/// <!-- Cells with rowspan -->
/// <table>
///   <tr>
///     <td rowspan="2">Spans 2 rows</td>
///     <td>Row 1, Col 2</td>
///   </tr>
///   <tr>
///     <td>Row 2, Col 2</td>
///   </tr>
/// </table>
///
/// <!-- Complex table with headers attribute -->
/// <table>
///   <thead>
///     <tr>
///       <th id="name">Name</th>
///       <th id="math">Math</th>
///       <th id="science">Science</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <th id="alice">Alice</th>
///       <td headers="alice math">95</td>
///       <td headers="alice science">92</td>
///     </tr>
///   </tbody>
/// </table>
///
/// <!-- Rich content in cells -->
/// <table>
///   <tr>
///     <td>
///       <strong>Product Name</strong><br>
///       <small>SKU: 12345</small>
///     </td>
///     <td>
///       <a href="/details">View Details</a>
///     </td>
///   </tr>
/// </table>
/// ```
///
/// # Accessibility
///
/// - Use `headers` attribute to associate cells with headers in complex tables
/// - Ensure data cells are properly associated with their headers
/// - Keep cell content concise and scannable
/// - Use `scope` on header cells to clarify relationships
///
/// # WHATWG Specification
///
/// - [4.9.9 The td element](https://html.spec.whatwg.org/multipage/tables.html#the-td-element)
pub struct Td;
impl HtmlElement for Td {
    const TAG: &'static str = "td";
}

// =============================================================================
// Forms
// =============================================================================

/// The `<form>` element - represents a document section containing interactive controls for submitting information.
///
/// # Purpose
///
/// The `<form>` element represents a collection of form-associated elements for gathering
/// user input and submitting data to a server. It provides the context for form controls,
/// handles submission, and defines how data should be encoded and transmitted.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Flow content (but no nested `<form>` elements)
///
/// # Common Use Cases
///
/// - User registration and login forms
/// - Search interfaces
/// - Contact and feedback forms
/// - E-commerce checkout processes
/// - Survey and questionnaire forms
///
/// # Key Attributes
///
/// - `action`: URL where form data is sent
/// - `method`: HTTP method for submission ("get" or "post")
/// - `enctype`: Encoding type for form data ("application/x-www-form-urlencoded", "multipart/form-data", "text/plain")
/// - `name`: Name of the form
/// - `target`: Browsing context for response ("_self", "_blank", "_parent", "_top")
/// - `novalidate`: Disable built-in validation
/// - `autocomplete`: Enable/disable autocomplete ("on" or "off")
/// - `accept-charset`: Character encodings for submission
///
/// # Example
///
/// ```html
/// <!-- Basic login form -->
/// <form action="/login" method="post">
///   <label for="username">Username:</label>
///   <input type="text" id="username" name="username" required>
///   
///   <label for="password">Password:</label>
///   <input type="password" id="password" name="password" required>
///   
///   <button type="submit">Log In</button>
/// </form>
///
/// <!-- Search form with GET -->
/// <form action="/search" method="get" role="search">
///   <label for="q">Search:</label>
///   <input type="search" id="q" name="q" placeholder="Enter search terms">
///   <button type="submit">Search</button>
/// </form>
///
/// <!-- File upload form -->
/// <form action="/upload" method="post" enctype="multipart/form-data">
///   <label for="file">Choose file:</label>
///   <input type="file" id="file" name="file" required>
///   <button type="submit">Upload</button>
/// </form>
///
/// <!-- Form with validation disabled -->
/// <form action="/submit" method="post" novalidate>
///   <input type="email" name="email">
///   <button type="submit">Submit</button>
/// </form>
///
/// <!-- Form targeting new window -->
/// <form action="/external" method="post" target="_blank">
///   <input type="text" name="data">
///   <button type="submit">Open in New Tab</button>
/// </form>
/// ```
///
/// # Accessibility
///
/// - Use `<label>` elements for all form controls
/// - Group related fields with `<fieldset>` and `<legend>`
/// - Provide clear error messages near relevant fields
/// - Ensure logical tab order through form fields
/// - Use `autocomplete` attributes appropriately
/// - Add `aria-describedby` for additional instructions
///
/// # WHATWG Specification
///
/// - [4.10.3 The form element](https://html.spec.whatwg.org/multipage/forms.html#the-form-element)
pub struct Form;
impl HtmlElement for Form {
    const TAG: &'static str = "form";
}
impl FlowContent for Form {}
impl PalpableContent for Form {}

/// The `<label>` element - represents a caption for a form control.
///
/// # Purpose
///
/// The `<label>` element provides a text label for a form control, creating an explicit
/// association between the label text and the control. Clicking the label activates the
/// associated control, improving usability and accessibility. Essential for screen readers
/// and assistive technologies.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content (excluding other `<label>` elements and labelable elements other than the labeled control)
///
/// # Common Use Cases
///
/// - Labeling text inputs and textareas
/// - Labeling checkboxes and radio buttons
/// - Labeling select dropdowns
/// - Improving form accessibility
/// - Increasing click target area for form controls
///
/// # Key Attributes
///
/// - `for`: ID of the form control this label is associated with
///
/// # Example
///
/// ```html
/// <!-- Label with for attribute -->
/// <label for="username">Username:</label>
/// <input type="text" id="username" name="username">
///
/// <!-- Label wrapping input -->
/// <label>
///   Email:
///   <input type="email" name="email">
/// </label>
///
/// <!-- Checkbox with label -->
/// <input type="checkbox" id="terms" name="terms">
/// <label for="terms">I agree to the terms and conditions</label>
///
/// <!-- Radio buttons with labels -->
/// <fieldset>
///   <legend>Choose a size:</legend>
///   <input type="radio" id="small" name="size" value="small">
///   <label for="small">Small</label>
///   
///   <input type="radio" id="medium" name="size" value="medium">
///   <label for="medium">Medium</label>
///   
///   <input type="radio" id="large" name="size" value="large">
///   <label for="large">Large</label>
/// </fieldset>
///
/// <!-- Label with required indicator -->
/// <label for="email">
///   Email Address <abbr title="required" aria-label="required">*</abbr>
/// </label>
/// <input type="email" id="email" name="email" required>
///
/// <!-- Wrapping label for checkbox -->
/// <label>
///   <input type="checkbox" name="subscribe" value="yes">
///   Subscribe to newsletter
/// </label>
/// ```
///
/// # Accessibility
///
/// - Always associate labels with form controls using `for` attribute or wrapping
/// - Ensure label text is descriptive and concise
/// - Don't use placeholder text as a substitute for labels
/// - Use `aria-label` or `aria-labelledby` when visual labels aren't possible
/// - Indicate required fields clearly
/// - Avoid nesting interactive elements within labels
///
/// # WHATWG Specification
///
/// - [4.10.4 The label element](https://html.spec.whatwg.org/multipage/forms.html#the-label-element)
pub struct Label;
impl HtmlElement for Label {
    const TAG: &'static str = "label";
}
impl FlowContent for Label {}
impl PhrasingContent for Label {}
impl InteractiveContent for Label {}
impl PalpableContent for Label {}

/// The `<input>` element - represents a typed data field for user input.
///
/// # Purpose
///
/// The `<input>` element is a versatile form control for collecting user input. Its behavior
/// and appearance vary dramatically based on the `type` attribute, ranging from text fields
/// to buttons, checkboxes, date pickers, and more. The most commonly used form element.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - If `type` is not "hidden": Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - None (void element)
///
/// # Common Use Cases
///
/// - Text and password input fields
/// - Checkboxes and radio buttons for selections
/// - File uploads
/// - Date, time, and color pickers
/// - Number and range sliders
///
/// # Key Attributes
///
/// - `type`: Input type (text, password, email, number, checkbox, radio, file, date, etc.)
/// - `name`: Name for form submission
/// - `value`: Current value of the control
/// - `placeholder`: Hint text displayed when empty
/// - `required`: Makes the field mandatory
/// - `disabled`: Disables the control
/// - `readonly`: Makes the field read-only
/// - `min`, `max`: Minimum and maximum values (for numeric/date types)
/// - `step`: Increment step (for numeric types)
/// - `pattern`: Regular expression for validation
/// - `autocomplete`: Autocomplete hint
/// - `multiple`: Allow multiple values (file, email)
/// - `accept`: File types to accept (for file inputs)
/// - `checked`: Pre-checked state (checkbox, radio)
///
/// # Example
///
/// ```html
/// <!-- Text input -->
/// <label for="name">Name:</label>
/// <input type="text" id="name" name="name" placeholder="Enter your name" required>
///
/// <!-- Email with validation -->
/// <label for="email">Email:</label>
/// <input type="email" id="email" name="email" placeholder="user@example.com" required>
///
/// <!-- Password field -->
/// <label for="pwd">Password:</label>
/// <input type="password" id="pwd" name="password" minlength="8" required>
///
/// <!-- Number with range -->
/// <label for="age">Age:</label>
/// <input type="number" id="age" name="age" min="18" max="120" step="1">
///
/// <!-- Checkbox -->
/// <input type="checkbox" id="subscribe" name="subscribe" value="yes" checked>
/// <label for="subscribe">Subscribe to newsletter</label>
///
/// <!-- Radio buttons -->
/// <input type="radio" id="color-red" name="color" value="red">
/// <label for="color-red">Red</label>
/// <input type="radio" id="color-blue" name="color" value="blue">
/// <label for="color-blue">Blue</label>
///
/// <!-- File upload -->
/// <label for="avatar">Profile picture:</label>
/// <input type="file" id="avatar" name="avatar" accept="image/*">
///
/// <!-- Date picker -->
/// <label for="dob">Date of birth:</label>
/// <input type="date" id="dob" name="dob" min="1900-01-01" max="2024-12-31">
///
/// <!-- Range slider -->
/// <label for="volume">Volume:</label>
/// <input type="range" id="volume" name="volume" min="0" max="100" value="50">
///
/// <!-- Search field -->
/// <input type="search" name="q" placeholder="Search..." autocomplete="off">
///
/// <!-- Color picker -->
/// <label for="color">Choose color:</label>
/// <input type="color" id="color" name="color" value="#ff0000">
/// ```
///
/// # Accessibility
///
/// - Always provide associated `<label>` elements
/// - Use appropriate `type` attribute for semantic meaning
/// - Provide helpful placeholder text (but don't rely on it alone)
/// - Use `aria-describedby` for additional instructions
/// - Ensure sufficient color contrast for visible inputs
/// - Make error messages clear and associated with inputs
/// - Use `autocomplete` for common fields
///
/// # WHATWG Specification
///
/// - [4.10.5 The input element](https://html.spec.whatwg.org/multipage/input.html#the-input-element)
pub struct Input;
impl HtmlElement for Input {
    const TAG: &'static str = "input";
    const VOID: bool = true;
}
impl FlowContent for Input {}
impl PhrasingContent for Input {}
impl InteractiveContent for Input {}
impl PalpableContent for Input {}

/// The `<button>` element - represents a clickable button.
///
/// # Purpose
///
/// The `<button>` element represents a clickable button control. Unlike `<input type="button">`,
/// it can contain rich content like text, images, and other elements. Used for form submission,
/// resetting forms, or triggering custom JavaScript actions. More flexible and semantic than
/// input buttons.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content (but no interactive content descendants)
///
/// # Common Use Cases
///
/// - Form submission buttons
/// - Form reset buttons
/// - Custom action buttons with JavaScript
/// - Toggle buttons for UI state changes
/// - Buttons with icons or complex content
///
/// # Key Attributes
///
/// - `type`: Button type ("submit", "reset", "button")
/// - `name`: Name for form submission
/// - `value`: Value sent with form submission
/// - `disabled`: Disables the button
/// - `form`: Associates with a form by ID
/// - `formaction`: Override form's action URL
/// - `formmethod`: Override form's method
/// - `formenctype`: Override form's encoding type
/// - `formnovalidate`: Override form's validation
/// - `formtarget`: Override form's target
///
/// # Example
///
/// ```html
/// <!-- Submit button -->
/// <form action="/submit" method="post">
///   <input type="text" name="username">
///   <button type="submit">Submit</button>
/// </form>
///
/// <!-- Reset button -->
/// <form>
///   <input type="text" name="data">
///   <button type="reset">Reset Form</button>
/// </form>
///
/// <!-- Button with custom action -->
/// <button type="button" onclick="alert('Clicked!')">Click Me</button>
///
/// <!-- Button with icon -->
/// <button type="submit">
///   <svg width="16" height="16">
///     <path d="M8 0l8 8-8 8-8-8z"/>
///   </svg>
///   Submit Form
/// </button>
///
/// <!-- Disabled button -->
/// <button type="submit" disabled>Please wait...</button>
///
/// <!-- Button overriding form attributes -->
/// <form action="/default" method="post">
///   <input type="text" name="data">
///   <button type="submit">Normal Submit</button>
///   <button type="submit" formaction="/alternative" formmethod="get">
///     Alternative Submit
///   </button>
/// </form>
///
/// <!-- Button associated with external form -->
/// <form id="myForm" action="/submit">
///   <input type="text" name="field">
/// </form>
/// <button type="submit" form="myForm">Submit External Form</button>
///
/// <!-- Delete button with confirmation -->
/// <button type="button" onclick="if(confirm('Delete?')) submit()">
///   Delete Item
/// </button>
/// ```
///
/// # Accessibility
///
/// - Use descriptive button text that explains the action
/// - Provide `aria-label` when button contains only an icon
/// - Use `type="button"` for non-submit actions to prevent accidental submission
/// - Ensure sufficient color contrast for button text
/// - Make buttons keyboard accessible (they are by default)
/// - Use `disabled` attribute to prevent interaction, not just CSS
/// - Provide visual feedback for button states (hover, active, disabled)
///
/// # WHATWG Specification
///
/// - [4.10.6 The button element](https://html.spec.whatwg.org/multipage/form-elements.html#the-button-element)
pub struct Button;
impl HtmlElement for Button {
    const TAG: &'static str = "button";
}
impl FlowContent for Button {}
impl PhrasingContent for Button {}
impl InteractiveContent for Button {}
impl PalpableContent for Button {}

/// The `<select>` element - represents a control for selecting among a set of options.
///
/// # Purpose
///
/// The `<select>` element provides a dropdown list of options from which users can choose
/// one or more values. Contains `<option>` elements that define the available choices, and
/// can be grouped using `<optgroup>` elements. More compact than radio buttons for multiple
/// choices.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Zero or more `<option>` or `<optgroup>` elements
///
/// # Common Use Cases
///
/// - Country or state selection dropdowns
/// - Category or type selectors
/// - Quantity or size pickers
/// - Date component selectors (month, year)
/// - Multi-select lists for tags or categories
///
/// # Key Attributes
///
/// - `name`: Name for form submission
/// - `multiple`: Allow selecting multiple options
/// - `size`: Number of visible options
/// - `required`: Make selection mandatory
/// - `disabled`: Disable the entire select
/// - `autocomplete`: Autocomplete hint
/// - `form`: Associates with a form by ID
///
/// # Example
///
/// ```html
/// <!-- Basic dropdown -->
/// <label for="country">Country:</label>
/// <select id="country" name="country">
///   <option value="">Select a country</option>
///   <option value="us">United States</option>
///   <option value="uk">United Kingdom</option>
///   <option value="ca">Canada</option>
/// </select>
///
/// <!-- Select with pre-selected option -->
/// <label for="size">Size:</label>
/// <select id="size" name="size">
///   <option value="s">Small</option>
///   <option value="m" selected>Medium</option>
///   <option value="l">Large</option>
/// </select>
///
/// <!-- Multi-select -->
/// <label for="interests">Interests (select multiple):</label>
/// <select id="interests" name="interests" multiple size="4">
///   <option value="sports">Sports</option>
///   <option value="music">Music</option>
///   <option value="art">Art</option>
///   <option value="tech">Technology</option>
/// </select>
///
/// <!-- Grouped options -->
/// <label for="car">Choose a car:</label>
/// <select id="car" name="car">
///   <optgroup label="Swedish Cars">
///     <option value="volvo">Volvo</option>
///     <option value="saab">Saab</option>
///   </optgroup>
///   <optgroup label="German Cars">
///     <option value="mercedes">Mercedes</option>
///     <option value="audi">Audi</option>
///   </optgroup>
/// </select>
///
/// <!-- Required select with placeholder -->
/// <label for="department">Department:</label>
/// <select id="department" name="department" required>
///   <option value="" disabled selected>-- Choose department --</option>
///   <option value="sales">Sales</option>
///   <option value="engineering">Engineering</option>
///   <option value="support">Support</option>
/// </select>
///
/// <!-- Disabled select -->
/// <label for="status">Status:</label>
/// <select id="status" name="status" disabled>
///   <option>Processing</option>
/// </select>
/// ```
///
/// # Accessibility
///
/// - Always provide an associated `<label>`
/// - Use first option as a prompt/placeholder, not a valid choice
/// - Provide clear option text
/// - Group related options with `<optgroup>`
/// - For long lists, consider searchable alternatives
/// - Ensure keyboard navigation works properly
/// - Use `aria-describedby` for additional instructions
///
/// # WHATWG Specification
///
/// - [4.10.7 The select element](https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element)
pub struct Select;
impl HtmlElement for Select {
    const TAG: &'static str = "select";
}
impl FlowContent for Select {}
impl PhrasingContent for Select {}
impl InteractiveContent for Select {}
impl PalpableContent for Select {}

/// The `<datalist>` element - contains a set of predefined options for other controls.
///
/// # Purpose
///
/// The `<datalist>` element provides a list of predefined options for an `<input>` element.
/// It creates an autocomplete or suggestion feature where users can either select from the
/// list or type their own value. Offers flexibility between free-form input and selection.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
///
/// # Permitted Content Model
///
/// - Either: Phrasing content
/// - Or: Zero or more `<option>` elements
///
/// # Common Use Cases
///
/// - Autocomplete suggestions for text inputs
/// - Common value suggestions with custom input allowed
/// - Search suggestions
/// - Product or category suggestions
/// - Location or address suggestions
///
/// # Key Attributes
///
/// - `id`: ID referenced by input's `list` attribute (required)
///
/// # Example
///
/// ```html
/// <!-- Basic datalist for text input -->
/// <label for="browser">Choose a browser:</label>
/// <input list="browsers" id="browser" name="browser">
/// <datalist id="browsers">
///   <option value="Chrome">
///   <option value="Firefox">
///   <option value="Safari">
///   <option value="Edge">
/// </datalist>
///
/// <!-- Datalist with labels -->
/// <label for="ice-cream">Favorite ice cream:</label>
/// <input list="ice-cream-flavors" id="ice-cream" name="ice-cream">
/// <datalist id="ice-cream-flavors">
///   <option value="Chocolate">Rich chocolate</option>
///   <option value="Vanilla">Classic vanilla</option>
///   <option value="Strawberry">Fresh strawberry</option>
/// </datalist>
///
/// <!-- Datalist for email with common domains -->
/// <label for="email">Email:</label>
/// <input type="email" list="email-domains" id="email" name="email">
/// <datalist id="email-domains">
///   <option value="user@gmail.com">
///   <option value="user@yahoo.com">
///   <option value="user@outlook.com">
/// </datalist>
///
/// <!-- Datalist for search -->
/// <label for="search">Search:</label>
/// <input type="search" list="recent-searches" id="search" name="q">
/// <datalist id="recent-searches">
///   <option value="HTML tutorials">
///   <option value="CSS grid layout">
///   <option value="JavaScript async">
/// </datalist>
///
/// <!-- Datalist for URL input -->
/// <label for="website">Website:</label>
/// <input type="url" list="popular-sites" id="website" name="website">
/// <datalist id="popular-sites">
///   <option value="https://github.com">
///   <option value="https://stackoverflow.com">
///   <option value="https://developer.mozilla.org">
/// </datalist>
/// ```
///
/// # Accessibility
///
/// - Options are announced to screen readers as suggestions
/// - Users can still type custom values
/// - Works with standard input accessibility features
/// - Provide meaningful option values
///
/// # WHATWG Specification
///
/// - [4.10.8 The datalist element](https://html.spec.whatwg.org/multipage/form-elements.html#the-datalist-element)
pub struct Datalist;
impl HtmlElement for Datalist {
    const TAG: &'static str = "datalist";
}
impl FlowContent for Datalist {}
impl PhrasingContent for Datalist {}

/// The `<optgroup>` element - groups related options within a `<select>` element.
///
/// # Purpose
///
/// The `<optgroup>` element groups related `<option>` elements within a `<select>` element.
/// Provides semantic grouping and visual separation of options, making long select lists
/// more organized and easier to navigate. Option groups are typically displayed with
/// indented options and a bold group label.
///
/// # Content Categories
///
/// - None (only valid within `<select>`)
///
/// # Permitted Content Model
///
/// - Zero or more `<option>` elements
///
/// # Common Use Cases
///
/// - Organizing countries by region
/// - Grouping products by category
/// - Categorizing options by type or brand
/// - Structuring hierarchical selections
/// - Improving navigation in long dropdown lists
///
/// # Key Attributes
///
/// - `label`: Name of the group (required)
/// - `disabled`: Disables all options in the group
///
/// # Example
///
/// ```html
/// <!-- Countries grouped by region -->
/// <label for="country">Select country:</label>
/// <select id="country" name="country">
///   <optgroup label="North America">
///     <option value="us">United States</option>
///     <option value="ca">Canada</option>
///     <option value="mx">Mexico</option>
///   </optgroup>
///   <optgroup label="Europe">
///     <option value="uk">United Kingdom</option>
///     <option value="de">Germany</option>
///     <option value="fr">France</option>
///   </optgroup>
///   <optgroup label="Asia">
///     <option value="jp">Japan</option>
///     <option value="cn">China</option>
///     <option value="in">India</option>
///   </optgroup>
/// </select>
///
/// <!-- Products by category -->
/// <label for="product">Choose product:</label>
/// <select id="product" name="product">
///   <optgroup label="Electronics">
///     <option value="laptop">Laptop</option>
///     <option value="phone">Smartphone</option>
///   </optgroup>
///   <optgroup label="Clothing">
///     <option value="shirt">T-Shirt</option>
///     <option value="jeans">Jeans</option>
///   </optgroup>
/// </select>
///
/// <!-- Disabled option group -->
/// <label for="service">Service level:</label>
/// <select id="service" name="service">
///   <optgroup label="Available">
///     <option value="basic">Basic</option>
///     <option value="standard">Standard</option>
///   </optgroup>
///   <optgroup label="Premium Options" disabled>
///     <option value="premium">Premium</option>
///     <option value="enterprise">Enterprise</option>
///   </optgroup>
/// </select>
///
/// <!-- Time zones grouped -->
/// <label for="timezone">Time zone:</label>
/// <select id="timezone" name="timezone">
///   <optgroup label="US Time Zones">
///     <option value="est">Eastern</option>
///     <option value="cst">Central</option>
///     <option value="pst">Pacific</option>
///   </optgroup>
///   <optgroup label="European Time Zones">
///     <option value="gmt">GMT</option>
///     <option value="cet">CET</option>
///   </optgroup>
/// </select>
/// ```
///
/// # Accessibility
///
/// - Group labels are announced by screen readers
/// - Helps users understand option organization
/// - Makes navigation easier in long lists
/// - Ensure group labels are descriptive
///
/// # WHATWG Specification
///
/// - [4.10.9 The optgroup element](https://html.spec.whatwg.org/multipage/form-elements.html#the-optgroup-element)
pub struct Optgroup;
impl HtmlElement for Optgroup {
    const TAG: &'static str = "optgroup";
}

/// The `<option>` element - defines an option in a `<select>`, `<optgroup>`, or `<datalist>`.
///
/// # Purpose
///
/// The `<option>` element defines an individual option within a `<select>` element or
/// suggestions within a `<datalist>` element. Each option represents a value that users
/// can choose. The text content of the element is what users see, while the `value`
/// attribute is what gets submitted with the form.
///
/// # Content Categories
///
/// - None (only valid within `<select>`, `<optgroup>`, or `<datalist>`)
///
/// # Permitted Content Model
///
/// - Text content (if `label` attribute is present, text is ignored)
///
/// # Common Use Cases
///
/// - Dropdown menu choices
/// - Multi-select list items
/// - Autocomplete suggestions
/// - Combobox options
/// - Form selection values
///
/// # Key Attributes
///
/// - `value`: Value submitted with the form (defaults to text content if not specified)
/// - `selected`: Pre-selects this option
/// - `disabled`: Disables this option
/// - `label`: Alternative text for the option (overrides text content)
///
/// # Example
///
/// ```html
/// <!-- Basic options -->
/// <select name="color">
///   <option value="red">Red</option>
///   <option value="green">Green</option>
///   <option value="blue">Blue</option>
/// </select>
///
/// <!-- Option with selected attribute -->
/// <select name="size">
///   <option value="s">Small</option>
///   <option value="m" selected>Medium</option>
///   <option value="l">Large</option>
/// </select>
///
/// <!-- Disabled option -->
/// <select name="status">
///   <option value="">Select status</option>
///   <option value="active">Active</option>
///   <option value="inactive" disabled>Inactive (unavailable)</option>
/// </select>
///
/// <!-- Options with different display and value -->
/// <select name="country">
///   <option value="us">United States</option>
///   <option value="uk">United Kingdom</option>
///   <option value="ca">Canada</option>
/// </select>
///
/// <!-- Option with label attribute -->
/// <select name="product">
///   <option value="prod1" label="Premium Widget">Premium Widget - $99</option>
///   <option value="prod2" label="Basic Widget">Basic Widget - $49</option>
/// </select>
///
/// <!-- Options in datalist -->
/// <input list="browsers" name="browser">
/// <datalist id="browsers">
///   <option value="Chrome">Google Chrome</option>
///   <option value="Firefox">Mozilla Firefox</option>
///   <option value="Safari">Apple Safari</option>
/// </datalist>
///
/// <!-- Placeholder option -->
/// <select name="category" required>
///   <option value="" disabled selected>-- Select category --</option>
///   <option value="tech">Technology</option>
///   <option value="health">Health</option>
///   <option value="finance">Finance</option>
/// </select>
/// ```
///
/// # Accessibility
///
/// - Use clear, concise option text
/// - Ensure value attributes are meaningful
/// - Don't rely solely on color to distinguish options
/// - Use disabled attribute instead of hiding options when appropriate
/// - Provide a default/placeholder option for clarity
///
/// # WHATWG Specification
///
/// - [4.10.10 The option element](https://html.spec.whatwg.org/multipage/form-elements.html#the-option-element)
pub struct Option_;
impl HtmlElement for Option_ {
    const TAG: &'static str = "option";
}

/// The `<textarea>` element - represents a multi-line plain text editing control.
///
/// # Purpose
///
/// The `<textarea>` element provides a multi-line text input control for entering larger
/// amounts of text. Unlike single-line `<input>` elements, textareas can contain multiple
/// lines and typically show scroll bars when content exceeds the visible area. Essential
/// for comments, descriptions, and longer form fields.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Text content (no child elements)
///
/// # Common Use Cases
///
/// - Comment and feedback forms
/// - Message composition areas
/// - Description and bio fields
/// - Code or text snippet input
/// - Notes and memo fields
///
/// # Key Attributes
///
/// - `name`: Name for form submission
/// - `rows`: Visible number of text lines
/// - `cols`: Visible width in average character widths
/// - `maxlength`: Maximum number of characters
/// - `minlength`: Minimum number of characters
/// - `placeholder`: Hint text when empty
/// - `required`: Makes the field mandatory
/// - `disabled`: Disables the control
/// - `readonly`: Makes the field read-only
/// - `autocomplete`: Autocomplete behavior
/// - `wrap`: Text wrapping behavior ("soft" or "hard")
/// - `spellcheck`: Enable spell checking
/// - `form`: Associates with a form by ID
///
/// # Example
///
/// ```html
/// <!-- Basic textarea -->
/// <label for="message">Message:</label>
/// <textarea id="message" name="message" rows="4" cols="50"></textarea>
///
/// <!-- Textarea with placeholder -->
/// <label for="comment">Comment:</label>
/// <textarea id="comment" name="comment" rows="5"
///           placeholder="Enter your comment here..."></textarea>
///
/// <!-- Textarea with default value -->
/// <label for="bio">Biography:</label>
/// <textarea id="bio" name="bio" rows="6" cols="60">
/// This is the default text that appears in the textarea.
/// It can span multiple lines.
/// </textarea>
///
/// <!-- Textarea with character limits -->
/// <label for="tweet">Tweet (280 characters max):</label>
/// <textarea id="tweet" name="tweet" rows="3" maxlength="280" required></textarea>
///
/// <!-- Readonly textarea -->
/// <label for="terms">Terms and Conditions:</label>
/// <textarea id="terms" rows="10" cols="80" readonly>
/// Lorem ipsum dolor sit amet, consectetur adipiscing elit.
/// These terms cannot be edited.
/// </textarea>
///
/// <!-- Textarea with hard wrap -->
/// <label for="email-body">Email body:</label>
/// <textarea id="email-body" name="body" rows="10" cols="72" wrap="hard"></textarea>
///
/// <!-- Disabled textarea -->
/// <label for="status">Status:</label>
/// <textarea id="status" rows="2" disabled>Processing...</textarea>
///
/// <!-- Textarea with spell check disabled -->
/// <label for="code">Code snippet:</label>
/// <textarea id="code" name="code" rows="8" spellcheck="false"
///           style="font-family: monospace;"></textarea>
/// ```
///
/// # Accessibility
///
/// - Always provide an associated `<label>`
/// - Use `aria-describedby` for additional instructions
/// - Provide clear character limits when applicable
/// - Ensure sufficient size for expected content
/// - Consider resize behavior for user control
/// - Use `placeholder` for hints, not instructions
/// - Ensure adequate color contrast
///
/// # WHATWG Specification
///
/// - [4.10.11 The textarea element](https://html.spec.whatwg.org/multipage/form-elements.html#the-textarea-element)
pub struct Textarea;
impl HtmlElement for Textarea {
    const TAG: &'static str = "textarea";
}
impl FlowContent for Textarea {}
impl PhrasingContent for Textarea {}
impl InteractiveContent for Textarea {}
impl PalpableContent for Textarea {}

/// The `<output>` element - represents the result of a calculation or user action.
///
/// # Purpose
///
/// The `<output>` element represents the result of a calculation, user action, or the
/// outcome of a script execution. It's specifically designed to display computed values
/// and results, typically from form calculations or JavaScript operations. Different from
/// regular text in that it semantically indicates generated output.
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
/// - Displaying calculation results in forms
/// - Showing real-time computed values
/// - Range slider value displays
/// - Form validation feedback
/// - Shopping cart totals and summaries
///
/// # Key Attributes
///
/// - `for`: Space-separated list of IDs of elements that contributed to the calculation
/// - `form`: Associates with a form by ID
/// - `name`: Name for form submission
///
/// # Example
///
/// ```html
/// <!-- Calculator result -->
/// <form oninput="result.value = parseInt(a.value) + parseInt(b.value)">
///   <input type="number" id="a" value="0"> +
///   <input type="number" id="b" value="0"> =
///   <output name="result" for="a b">0</output>
/// </form>
///
/// <!-- Range slider value display -->
/// <label for="volume">Volume:</label>
/// <input type="range" id="volume" min="0" max="100" value="50"
///        oninput="volumeOutput.value = this.value">
/// <output id="volumeOutput" for="volume">50</output>
///
/// <!-- Price calculator -->
/// <form oninput="total.value = (quantity.value * 10).toFixed(2)">
///   <label for="quantity">Quantity:</label>
///   <input type="number" id="quantity" name="quantity" value="1" min="1">
///   <p>Price per item: $10.00</p>
///   <p>Total: $<output name="total" for="quantity">10.00</output></p>
/// </form>
///
/// <!-- BMI Calculator -->
/// <form oninput="bmi.value = (weight.value / ((height.value / 100) ** 2)).toFixed(1)">
///   <label for="weight">Weight (kg):</label>
///   <input type="number" id="weight" value="70">
///   
///   <label for="height">Height (cm):</label>
///   <input type="number" id="height" value="175">
///   
///   <p>BMI: <output name="bmi" for="weight height">22.9</output></p>
/// </form>
///
/// <!-- Form validation summary -->
/// <form>
///   <input type="email" id="email" required>
///   <output for="email" id="emailStatus"></output>
/// </form>
/// ```
///
/// # Accessibility
///
/// - Output values are announced by screen readers when changed
/// - Use `aria-live="polite"` for important dynamic updates
/// - Ensure output is clearly associated with input controls via `for` attribute
/// - Provide context for what the output represents
/// - Make output values visually distinct
///
/// # WHATWG Specification
///
/// - [4.10.12 The output element](https://html.spec.whatwg.org/multipage/form-elements.html#the-output-element)
pub struct Output;
impl HtmlElement for Output {
    const TAG: &'static str = "output";
}
impl FlowContent for Output {}
impl PhrasingContent for Output {}
impl PalpableContent for Output {}

/// The `<progress>` element - represents the completion progress of a task.
///
/// # Purpose
///
/// The `<progress>` element displays the progress of a task, such as a file upload, download,
/// or form completion. Shows a progress bar indicating how much of the task has been completed.
/// Can represent both determinate (known total) and indeterminate (unknown total) progress.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content (fallback for browsers without progress support, but no `<progress>` descendants)
///
/// # Common Use Cases
///
/// - File upload progress indicators
/// - Download progress displays
/// - Form completion tracking
/// - Multi-step process progress
/// - Loading indicators with known duration
///
/// # Key Attributes
///
/// - `value`: Current progress value
/// - `max`: Maximum value (total amount of work, default: 1.0)
///
/// # Example
///
/// ```html
/// <!-- Basic progress bar -->
/// <label for="file-progress">Uploading file:</label>
/// <progress id="file-progress" value="70" max="100">70%</progress>
///
/// <!-- Progress with percentage label -->
/// <p>
///   Installation progress:
///   <progress value="0.6" max="1.0">60%</progress>
///   60%
/// </p>
///
/// <!-- Indeterminate progress (no value attribute) -->
/// <p>
///   Loading...
///   <progress max="100">Loading...</progress>
/// </p>
///
/// <!-- Progress updated via JavaScript -->
/// <progress id="dynamic-progress" value="0" max="100"></progress>
/// <script>
///   let progress = 0;
///   setInterval(() => {
///     progress = Math.min(progress + 10, 100);
///     document.getElementById('dynamic-progress').value = progress;
///   }, 500);
/// </script>
///
/// <!-- Download progress -->
/// <p>
///   Downloading: <span id="filename">document.pdf</span>
///   <progress id="download" value="2.5" max="10">2.5 MB of 10 MB</progress>
///   <span id="progress-text">2.5 MB of 10 MB</span>
/// </p>
///
/// <!-- Form completion indicator -->
/// <form>
///   <p>Form completion: <progress value="3" max="5">Step 3 of 5</progress></p>
///   <!-- Form fields here -->
/// </form>
///
/// <!-- Styled progress bar -->
/// <progress value="75" max="100" style="width: 300px; height: 30px;">
///   75% complete
/// </progress>
/// ```
///
/// # Accessibility
///
/// - Provide text content as fallback for older browsers
/// - Use `aria-label` or nearby text to describe what's progressing
/// - Update `aria-valuenow`, `aria-valuemin`, `aria-valuemax` for complex cases
/// - Announce progress updates to screen readers with aria-live regions
/// - Ensure progress bar has sufficient color contrast
///
/// # WHATWG Specification
///
/// - [4.10.13 The progress element](https://html.spec.whatwg.org/multipage/form-elements.html#the-progress-element)
pub struct Progress;
impl HtmlElement for Progress {
    const TAG: &'static str = "progress";
}
impl FlowContent for Progress {}
impl PhrasingContent for Progress {}
impl PalpableContent for Progress {}

/// The `<meter>` element - represents a scalar measurement within a known range.
///
/// # Purpose
///
/// The `<meter>` element represents a scalar measurement within a known range, or a fractional
/// value. Used for displaying measurements like disk usage, voting results, relevance scores,
/// or any gauge-style indicator. Unlike `<progress>`, which shows task completion, `<meter>`
/// shows a measurement on a scale.
///
/// # Content Categories
///
/// - Flow Content
/// - Phrasing Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Phrasing content (fallback for browsers without meter support, but no `<meter>` descendants)
///
/// # Common Use Cases
///
/// - Disk space usage indicators
/// - Battery level displays
/// - Rating or scoring displays
/// - Temperature or volume gauges
/// - Relevance or match percentages
///
/// # Key Attributes
///
/// - `value`: Current value (required)
/// - `min`: Lower bound of range (default: 0)
/// - `max`: Upper bound of range (default: 1)
/// - `low`: Upper bound of "low" range
/// - `high`: Lower bound of "high" range
/// - `optimum`: Optimal value in the range
///
/// # Example
///
/// ```html
/// <!-- Basic meter -->
/// <label for="disk-usage">Disk usage:</label>
/// <meter id="disk-usage" value="0.6">60%</meter>
///
/// <!-- Meter with full range specification -->
/// <label for="battery">Battery level:</label>
/// <meter id="battery"
///        min="0" max="100"
///        low="20" high="80"
///        optimum="100"
///        value="65">65%</meter>
///
/// <!-- High value is bad (e.g., CPU usage) -->
/// <label for="cpu">CPU usage:</label>
/// <meter id="cpu"
///        min="0" max="100"
///        low="25" high="75"
///        optimum="0"
///        value="85">85%</meter>
///
/// <!-- Low value is bad (e.g., fuel) -->
/// <label for="fuel">Fuel level:</label>
/// <meter id="fuel"
///        min="0" max="100"
///        low="20" high="80"
///        optimum="100"
///        value="15">15%</meter>
///
/// <!-- Rating display -->
/// <p>
///   Rating:
///   <meter min="0" max="5" value="4.2">4.2 out of 5</meter>
///   4.2 out of 5 stars
/// </p>
///
/// <!-- Storage usage -->
/// <p>
///   Storage used:
///   <meter min="0" max="1000" low="700" high="900" value="852">
///     852 GB of 1000 GB
///   </meter>
///   852 GB / 1 TB
/// </p>
///
/// <!-- Temperature gauge -->
/// <label for="temp">Room temperature:</label>
/// <meter id="temp"
///        min="0" max="50"
///        low="18" high="28"
///        optimum="22"
///        value="25">25°C</meter>
/// ```
///
/// # Accessibility
///
/// - Provide text content as fallback
/// - Use labels to describe what's being measured
/// - Ensure color isn't the only indicator of status
/// - Browser typically colors the meter based on value ranges
/// - Screen readers announce the value and context
///
/// # WHATWG Specification
///
/// - [4.10.14 The meter element](https://html.spec.whatwg.org/multipage/form-elements.html#the-meter-element)
pub struct Meter;
impl HtmlElement for Meter {
    const TAG: &'static str = "meter";
}
impl FlowContent for Meter {}
impl PhrasingContent for Meter {}
impl PalpableContent for Meter {}

/// The `<fieldset>` element - groups related form controls and labels.
///
/// # Purpose
///
/// The `<fieldset>` element groups related form controls and labels within a form. Provides
/// semantic grouping and visual separation of form sections. Can be disabled as a group,
/// affecting all contained controls. Typically rendered with a border around the grouped
/// elements.
///
/// # Content Categories
///
/// - Flow Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - Optionally a `<legend>` element (must be first child)
/// - Followed by flow content
///
/// # Common Use Cases
///
/// - Grouping personal information fields
/// - Grouping address fields
/// - Grouping related checkboxes or radio buttons
/// - Creating form sections with logical divisions
/// - Disabling multiple controls at once
///
/// # Key Attributes
///
/// - `disabled`: Disables all form controls within the fieldset
/// - `form`: Associates with a form by ID
/// - `name`: Name for the fieldset (for scripting purposes)
///
/// # Example
///
/// ```html
/// <!-- Basic fieldset with legend -->
/// <fieldset>
///   <legend>Personal Information</legend>
///   <label for="fname">First name:</label>
///   <input type="text" id="fname" name="fname">
///   
///   <label for="lname">Last name:</label>
///   <input type="text" id="lname" name="lname">
/// </fieldset>
///
/// <!-- Radio button group -->
/// <fieldset>
///   <legend>Choose your preferred contact method:</legend>
///   <input type="radio" id="email" name="contact" value="email">
///   <label for="email">Email</label>
///   
///   <input type="radio" id="phone" name="contact" value="phone">
///   <label for="phone">Phone</label>
///   
///   <input type="radio" id="mail" name="contact" value="mail">
///   <label for="mail">Mail</label>
/// </fieldset>
///
/// <!-- Address fieldset -->
/// <fieldset>
///   <legend>Billing Address</legend>
///   <label for="street">Street:</label>
///   <input type="text" id="street" name="street">
///   
///   <label for="city">City:</label>
///   <input type="text" id="city" name="city">
///   
///   <label for="zip">ZIP Code:</label>
///   <input type="text" id="zip" name="zip">
/// </fieldset>
///
/// <!-- Disabled fieldset -->
/// <fieldset disabled>
///   <legend>Advanced Settings (Coming Soon)</legend>
///   <label for="option1">Option 1:</label>
///   <input type="checkbox" id="option1" name="option1">
///   
///   <label for="option2">Option 2:</label>
///   <input type="checkbox" id="option2" name="option2">
/// </fieldset>
///
/// <!-- Nested fieldsets -->
/// <form>
///   <fieldset>
///     <legend>Account Information</legend>
///     
///     <fieldset>
///       <legend>Login Credentials</legend>
///       <label for="user">Username:</label>
///       <input type="text" id="user" name="username">
///       
///       <label for="pass">Password:</label>
///       <input type="password" id="pass" name="password">
///     </fieldset>
///     
///     <fieldset>
///       <legend>Contact Details</legend>
///       <label for="email">Email:</label>
///       <input type="email" id="email" name="email">
///     </fieldset>
///   </fieldset>
/// </form>
/// ```
///
/// # Accessibility
///
/// - Use `<legend>` to provide a clear group label
/// - Legends are announced by screen readers when entering the fieldset
/// - Helps users understand form structure and relationships
/// - Disabled fieldsets clearly indicate unavailable sections
/// - Ensure legends are concise and descriptive
///
/// # WHATWG Specification
///
/// - [4.10.15 The fieldset element](https://html.spec.whatwg.org/multipage/form-elements.html#the-fieldset-element)
pub struct Fieldset;
impl HtmlElement for Fieldset {
    const TAG: &'static str = "fieldset";
}
impl FlowContent for Fieldset {}
impl PalpableContent for Fieldset {}

/// The `<legend>` element - represents a caption for a `<fieldset>`.
///
/// # Purpose
///
/// The `<legend>` element provides a caption or title for its parent `<fieldset>` element.
/// It describes the group of form controls contained within the fieldset. Must be the first
/// child of the fieldset if present. Typically displayed as a title positioned on the border
/// of the fieldset.
///
/// # Content Categories
///
/// - None (only valid as first child of `<fieldset>`)
///
/// # Permitted Content Model
///
/// - Phrasing content
/// - Optionally intermixed with heading content
///
/// # Common Use Cases
///
/// - Labeling form sections
/// - Describing groups of radio buttons or checkboxes
/// - Titling address or contact information groups
/// - Naming configuration sections
/// - Providing context for related form fields
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Basic legend -->
/// <fieldset>
///   <legend>Personal Details</legend>
///   <label for="name">Name:</label>
///   <input type="text" id="name" name="name">
/// </fieldset>
///
/// <!-- Legend for radio button group -->
/// <fieldset>
///   <legend>Select your subscription plan:</legend>
///   <input type="radio" id="basic" name="plan" value="basic">
///   <label for="basic">Basic - $9.99/mo</label>
///   
///   <input type="radio" id="premium" name="plan" value="premium">
///   <label for="premium">Premium - $19.99/mo</label>
/// </fieldset>
///
/// <!-- Legend with emphasis -->
/// <fieldset>
///   <legend><strong>Required Information</strong></legend>
///   <label for="email">Email:</label>
///   <input type="email" id="email" name="email" required>
/// </fieldset>
///
/// <!-- Legend with additional context -->
/// <fieldset>
///   <legend>
///     Shipping Address
///     <small>(where should we deliver your order?)</small>
///   </legend>
///   <label for="address">Address:</label>
///   <input type="text" id="address" name="address">
/// </fieldset>
///
/// <!-- Legend for checkbox group -->
/// <fieldset>
///   <legend>Select your interests:</legend>
///   <input type="checkbox" id="sports" name="interests" value="sports">
///   <label for="sports">Sports</label>
///   
///   <input type="checkbox" id="music" name="interests" value="music">
///   <label for="music">Music</label>
///   
///   <input type="checkbox" id="reading" name="interests" value="reading">
///   <label for="reading">Reading</label>
/// </fieldset>
///
/// <!-- Legend for payment information -->
/// <fieldset>
///   <legend>Payment Information</legend>
///   <label for="card">Card Number:</label>
///   <input type="text" id="card" name="card-number">
///   
///   <label for="expiry">Expiry Date:</label>
///   <input type="text" id="expiry" name="expiry">
/// </fieldset>
/// ```
///
/// # Accessibility
///
/// - Legends are announced by screen readers when users enter the fieldset
/// - Provides important context for form field groups
/// - Keep legend text concise and descriptive
/// - Use heading elements within legend sparingly
/// - Helps users understand the purpose of grouped fields
///
/// # WHATWG Specification
///
/// - [4.10.16 The legend element](https://html.spec.whatwg.org/multipage/form-elements.html#the-legend-element)
pub struct Legend;
impl HtmlElement for Legend {
    const TAG: &'static str = "legend";
}

// =============================================================================
// Interactive Elements
// =============================================================================

/// The `<details>` element - represents a disclosure widget from which the user can obtain additional information.
///
/// # Purpose
///
/// The `<details>` element creates a disclosure widget that users can open and close to
/// reveal or hide additional content. Provides built-in interactive functionality without
/// JavaScript. The first child should be a `<summary>` element that serves as the toggle label.
///
/// # Content Categories
///
/// - Flow Content
/// - Interactive Content
/// - Palpable Content
///
/// # Permitted Content Model
///
/// - One `<summary>` element (as first child)
/// - Followed by flow content
///
/// # Common Use Cases
///
/// - FAQ (Frequently Asked Questions) sections
/// - Accordion-style content
/// - Progressive disclosure of information
/// - Collapsible content sections
/// - Show/hide additional details
///
/// # Key Attributes
///
/// - `open`: Makes the details visible by default
///
/// # Example
///
/// ```html
/// <!-- Basic details element -->
/// <details>
///   <summary>Click to expand</summary>
///   <p>This is the hidden content that appears when you click the summary.</p>
/// </details>
///
/// <!-- FAQ item -->
/// <details>
///   <summary>What is HTML?</summary>
///   <p>HTML (HyperText Markup Language) is the standard markup language for creating web pages.</p>
/// </details>
///
/// <!-- Details open by default -->
/// <details open>
///   <summary>Important Information</summary>
///   <p>This section is expanded by default because of the 'open' attribute.</p>
/// </details>
///
/// <!-- Multiple details forming an accordion -->
/// <details>
///   <summary>Section 1: Introduction</summary>
///   <p>This is the introduction section with detailed information.</p>
/// </details>
/// <details>
///   <summary>Section 2: Features</summary>
///   <ul>
///     <li>Feature A</li>
///     <li>Feature B</li>
///     <li>Feature C</li>
///   </ul>
/// </details>
/// <details>
///   <summary>Section 3: Conclusion</summary>
///   <p>Final thoughts and summary.</p>
/// </details>
///
/// <!-- Details with rich content -->
/// <details>
///   <summary>View shipping options</summary>
///   <table>
///     <thead>
///       <tr>
///         <th>Method</th>
///         <th>Time</th>
///         <th>Cost</th>
///       </tr>
///     </thead>
///     <tbody>
///       <tr>
///         <td>Standard</td>
///         <td>5-7 days</td>
///         <td>$5.99</td>
///       </tr>
///       <tr>
///         <td>Express</td>
///         <td>2-3 days</td>
///         <td>$12.99</td>
///       </tr>
///     </tbody>
///   </table>
/// </details>
///
/// <!-- Nested details -->
/// <details>
///   <summary>Chapter 1</summary>
///   <p>Chapter introduction...</p>
///   <details>
///     <summary>Section 1.1</summary>
///     <p>Detailed content for section 1.1</p>
///   </details>
///   <details>
///     <summary>Section 1.2</summary>
///     <p>Detailed content for section 1.2</p>
///   </details>
/// </details>
/// ```
///
/// # Accessibility
///
/// - Keyboard accessible by default (Space/Enter to toggle)
/// - Screen readers announce the collapsed/expanded state
/// - Use descriptive summary text
/// - Ensure content within is properly structured
/// - Consider adding visual indicators for state
/// - Works without JavaScript
///
/// # WHATWG Specification
///
/// - [4.11.1 The details element](https://html.spec.whatwg.org/multipage/interactive-elements.html#the-details-element)
pub struct Details;
impl HtmlElement for Details {
    const TAG: &'static str = "details";
}
impl FlowContent for Details {}
impl InteractiveContent for Details {}
impl PalpableContent for Details {}

/// The `<summary>` element - represents a summary, caption, or legend for a `<details>` element.
///
/// # Purpose
///
/// The `<summary>` element provides a visible heading or label for its parent `<details>`
/// element. Users click the summary to toggle the visibility of the details content. Acts
/// as the disclosure button/trigger. If omitted, browser provides a default label like
/// "Details".
///
/// # Content Categories
///
/// - None (only valid as first child of `<details>`)
///
/// # Permitted Content Model
///
/// - Phrasing content
/// - Optionally intermixed with heading content
///
/// # Common Use Cases
///
/// - FAQ question labels
/// - Accordion section titles
/// - Toggle button labels
/// - Expandable content headings
/// - Show/hide trigger text
///
/// # Key Attributes
///
/// - Global attributes only
///
/// # Example
///
/// ```html
/// <!-- Simple summary -->
/// <details>
///   <summary>Click to reveal answer</summary>
///   <p>This is the answer to the question.</p>
/// </details>
///
/// <!-- Summary with icon/emoji -->
/// <details>
///   <summary>📋 View Details</summary>
///   <p>Additional information here.</p>
/// </details>
///
/// <!-- Summary with styled text -->
/// <details>
///   <summary><strong>Important Notice</strong></summary>
///   <p>Please read this important information carefully.</p>
/// </details>
///
/// <!-- Summary for FAQ -->
/// <details>
///   <summary>How do I reset my password?</summary>
///   <ol>
///     <li>Click on "Forgot Password"</li>
///     <li>Enter your email address</li>
///     <li>Check your email for reset link</li>
///   </ol>
/// </details>
///
/// <!-- Summary with heading -->
/// <details>
///   <summary><h3>Chapter 1: Introduction</h3></summary>
///   <p>Chapter content goes here...</p>
/// </details>
///
/// <!-- Summary with additional context -->
/// <details>
///   <summary>
///     Product Specifications
///     <small>(click to expand)</small>
///   </summary>
///   <ul>
///     <li>Weight: 2.5 kg</li>
///     <li>Dimensions: 30cm x 20cm x 10cm</li>
///     <li>Material: Aluminum</li>
///   </ul>
/// </details>
///
/// <!-- Summary with custom styling -->
/// <details>
///   <summary style="cursor: pointer; color: #0066cc;">
///     ► Show More Information
///   </summary>
///   <p>Hidden content revealed on click.</p>
/// </details>
/// ```
///
/// # Accessibility
///
/// - Acts as a button control, automatically keyboard accessible
/// - Screen readers announce it as a button with expanded/collapsed state
/// - Use descriptive text that clearly indicates what will be revealed
/// - Ensure summary is meaningful when read alone
/// - Avoid generic text like "Click here" or "More"
/// - Browser typically adds a disclosure triangle/arrow indicator
///
/// # WHATWG Specification
///
/// - [4.11.2 The summary element](https://html.spec.whatwg.org/multipage/interactive-elements.html#the-summary-element)
pub struct Summary;
impl HtmlElement for Summary {
    const TAG: &'static str = "summary";
}

/// The `<dialog>` element - represents a dialog box or other interactive component.
///
/// # Purpose
///
/// The `<dialog>` element represents a dialog box, modal, or subwindow that overlays the
/// main content. Can be shown modally (blocking interaction with the rest of the page) or
/// non-modally. Provides built-in functionality for overlays, keyboard management (ESC to
/// close), and focus trapping without requiring JavaScript libraries.
///
/// # Content Categories
///
/// - Flow Content
///
/// # Permitted Content Model
///
/// - Flow content
///
/// # Common Use Cases
///
/// - Modal confirmation dialogs
/// - Alert and notification popups
/// - Login and signup forms
/// - Settings and preferences panels
/// - Image lightboxes and galleries
///
/// # Key Attributes
///
/// - `open`: Makes the dialog visible (use JavaScript's `show()` or `showModal()` methods instead)
///
/// # Example
///
/// ```html
/// <!-- Basic dialog -->
/// <dialog id="myDialog">
///   <h2>Dialog Title</h2>
///   <p>This is a dialog box.</p>
///   <button onclick="this.closest('dialog').close()">Close</button>
/// </dialog>
/// <button onclick="document.getElementById('myDialog').showModal()">Open Dialog</button>
///
/// <!-- Confirmation dialog -->
/// <dialog id="confirmDialog">
///   <form method="dialog">
///     <h2>Confirm Action</h2>
///     <p>Are you sure you want to proceed?</p>
///     <button value="cancel">Cancel</button>
///     <button value="confirm">Confirm</button>
///   </form>
/// </dialog>
///
/// <!-- Login dialog -->
/// <dialog id="loginDialog">
///   <h2>Log In</h2>
///   <form method="dialog">
///     <label for="username">Username:</label>
///     <input type="text" id="username" name="username" required>
///     
///     <label for="password">Password:</label>
///     <input type="password" id="password" name="password" required>
///     
///     <button type="submit">Log In</button>
///     <button type="button" onclick="this.closest('dialog').close()">Cancel</button>
///   </form>
/// </dialog>
///
/// <!-- Dialog with backdrop -->
/// <dialog id="modalDialog">
///   <h2>Modal Dialog</h2>
///   <p>This dialog blocks interaction with the rest of the page.</p>
///   <button onclick="document.getElementById('modalDialog').close()">Close</button>
/// </dialog>
/// <button onclick="document.getElementById('modalDialog').showModal()">Show Modal</button>
///
/// <!-- Alert dialog -->
/// <dialog id="alertDialog">
///   <div role="alert">
///     <h2>Warning!</h2>
///     <p>This action cannot be undone.</p>
///     <button onclick="this.closest('dialog').close()">OK</button>
///   </div>
/// </dialog>
///
/// <!-- Non-modal dialog -->
/// <dialog id="nonModalDialog">
///   <h2>Non-Modal Dialog</h2>
///   <p>You can still interact with the page behind this dialog.</p>
///   <button onclick="this.closest('dialog').close()">Close</button>
/// </dialog>
/// <button onclick="document.getElementById('nonModalDialog').show()">Show Non-Modal</button>
///
/// <!-- Dialog with return value -->
/// <dialog id="colorDialog">
///   <form method="dialog">
///     <h2>Choose a color</h2>
///     <button value="red">Red</button>
///     <button value="blue">Blue</button>
///     <button value="green">Green</button>
///   </form>
/// </dialog>
/// <script>
///   const dialog = document.getElementById('colorDialog');
///   dialog.addEventListener('close', () => {
///     console.log('Selected color:', dialog.returnValue);
///   });
/// </script>
/// ```
///
/// # Accessibility
///
/// - Focus is automatically moved to the dialog when opened with `showModal()`
/// - ESC key closes modal dialogs by default
/// - Backdrop click behavior can be customized
/// - Use appropriate ARIA roles (role="dialog" or role="alertdialog")
/// - Provide clear close mechanisms
/// - Ensure focus returns to triggering element on close
/// - Use `aria-labelledby` or `aria-label` to identify the dialog
/// - Trap focus within modal dialogs
///
/// # WHATWG Specification
///
/// - [4.11.3 The dialog element](https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element)
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
