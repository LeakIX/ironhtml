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
pub struct Html;
impl HtmlElement for Html {
    const TAG: &'static str = "html";
}

/// The `<head>` element - container for document metadata.
pub struct Head;
impl HtmlElement for Head {
    const TAG: &'static str = "head";
}
impl MetadataContent for Head {}

/// The `<title>` element - document title.
pub struct Title;
impl HtmlElement for Title {
    const TAG: &'static str = "title";
}
impl MetadataContent for Title {}

/// The `<base>` element - base URL for relative URLs.
pub struct Base;
impl HtmlElement for Base {
    const TAG: &'static str = "base";
    const VOID: bool = true;
}
impl MetadataContent for Base {}

/// The `<link>` element - external resource link.
pub struct Link;
impl HtmlElement for Link {
    const TAG: &'static str = "link";
    const VOID: bool = true;
}
impl MetadataContent for Link {}

/// The `<meta>` element - metadata.
pub struct Meta;
impl HtmlElement for Meta {
    const TAG: &'static str = "meta";
    const VOID: bool = true;
}
impl MetadataContent for Meta {}

/// The `<style>` element - embedded CSS.
pub struct Style;
impl HtmlElement for Style {
    const TAG: &'static str = "style";
}
impl MetadataContent for Style {}

// =============================================================================
// Sectioning Root
// =============================================================================

/// The `<body>` element - document body.
pub struct Body;
impl HtmlElement for Body {
    const TAG: &'static str = "body";
}

// =============================================================================
// Content Sectioning Elements
// =============================================================================

/// The `<article>` element - self-contained composition.
pub struct Article;
impl HtmlElement for Article {
    const TAG: &'static str = "article";
}
impl FlowContent for Article {}
impl SectioningContent for Article {}
impl PalpableContent for Article {}

/// The `<section>` element - generic section.
pub struct Section;
impl HtmlElement for Section {
    const TAG: &'static str = "section";
}
impl FlowContent for Section {}
impl SectioningContent for Section {}
impl PalpableContent for Section {}

/// The `<nav>` element - navigation links.
pub struct Nav;
impl HtmlElement for Nav {
    const TAG: &'static str = "nav";
}
impl FlowContent for Nav {}
impl SectioningContent for Nav {}
impl PalpableContent for Nav {}

/// The `<aside>` element - tangentially related content.
pub struct Aside;
impl HtmlElement for Aside {
    const TAG: &'static str = "aside";
}
impl FlowContent for Aside {}
impl SectioningContent for Aside {}
impl PalpableContent for Aside {}

/// The `<h1>` element - level 1 heading.
pub struct H1;
impl HtmlElement for H1 {
    const TAG: &'static str = "h1";
}
impl FlowContent for H1 {}
impl HeadingContent for H1 {}
impl PalpableContent for H1 {}

/// The `<h2>` element - level 2 heading.
pub struct H2;
impl HtmlElement for H2 {
    const TAG: &'static str = "h2";
}
impl FlowContent for H2 {}
impl HeadingContent for H2 {}
impl PalpableContent for H2 {}

/// The `<h3>` element - level 3 heading.
pub struct H3;
impl HtmlElement for H3 {
    const TAG: &'static str = "h3";
}
impl FlowContent for H3 {}
impl HeadingContent for H3 {}
impl PalpableContent for H3 {}

/// The `<h4>` element - level 4 heading.
pub struct H4;
impl HtmlElement for H4 {
    const TAG: &'static str = "h4";
}
impl FlowContent for H4 {}
impl HeadingContent for H4 {}
impl PalpableContent for H4 {}

/// The `<h5>` element - level 5 heading.
pub struct H5;
impl HtmlElement for H5 {
    const TAG: &'static str = "h5";
}
impl FlowContent for H5 {}
impl HeadingContent for H5 {}
impl PalpableContent for H5 {}

/// The `<h6>` element - level 6 heading.
pub struct H6;
impl HtmlElement for H6 {
    const TAG: &'static str = "h6";
}
impl FlowContent for H6 {}
impl HeadingContent for H6 {}
impl PalpableContent for H6 {}

/// The `<hgroup>` element - heading group.
pub struct Hgroup;
impl HtmlElement for Hgroup {
    const TAG: &'static str = "hgroup";
}
impl FlowContent for Hgroup {}
impl HeadingContent for Hgroup {}
impl PalpableContent for Hgroup {}

/// The `<header>` element - introductory content.
pub struct Header;
impl HtmlElement for Header {
    const TAG: &'static str = "header";
}
impl FlowContent for Header {}
impl PalpableContent for Header {}

/// The `<footer>` element - footer content.
pub struct Footer;
impl HtmlElement for Footer {
    const TAG: &'static str = "footer";
}
impl FlowContent for Footer {}
impl PalpableContent for Footer {}

/// The `<address>` element - contact information.
pub struct Address;
impl HtmlElement for Address {
    const TAG: &'static str = "address";
}
impl FlowContent for Address {}
impl PalpableContent for Address {}

/// The `<main>` element - main content.
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
pub struct Div;
impl HtmlElement for Div {
    const TAG: &'static str = "div";
}
impl FlowContent for Div {}
impl PalpableContent for Div {}

/// The `<p>` element - paragraph.
pub struct P;
impl HtmlElement for P {
    const TAG: &'static str = "p";
}
impl FlowContent for P {}
impl PalpableContent for P {}

/// The `<hr>` element - thematic break.
pub struct Hr;
impl HtmlElement for Hr {
    const TAG: &'static str = "hr";
    const VOID: bool = true;
}
impl FlowContent for Hr {}

/// The `<pre>` element - preformatted text.
pub struct Pre;
impl HtmlElement for Pre {
    const TAG: &'static str = "pre";
}
impl FlowContent for Pre {}
impl PalpableContent for Pre {}

/// The `<blockquote>` element - block quotation.
pub struct Blockquote;
impl HtmlElement for Blockquote {
    const TAG: &'static str = "blockquote";
}
impl FlowContent for Blockquote {}
impl PalpableContent for Blockquote {}

/// The `<ol>` element - ordered list.
pub struct Ol;
impl HtmlElement for Ol {
    const TAG: &'static str = "ol";
}
impl FlowContent for Ol {}
impl PalpableContent for Ol {}

/// The `<ul>` element - unordered list.
pub struct Ul;
impl HtmlElement for Ul {
    const TAG: &'static str = "ul";
}
impl FlowContent for Ul {}
impl PalpableContent for Ul {}

/// The `<menu>` element - menu of commands.
pub struct Menu;
impl HtmlElement for Menu {
    const TAG: &'static str = "menu";
}
impl FlowContent for Menu {}
impl PalpableContent for Menu {}

/// The `<li>` element - list item.
pub struct Li;
impl HtmlElement for Li {
    const TAG: &'static str = "li";
}

/// The `<dl>` element - description list.
pub struct Dl;
impl HtmlElement for Dl {
    const TAG: &'static str = "dl";
}
impl FlowContent for Dl {}
impl PalpableContent for Dl {}

/// The `<dt>` element - description term.
pub struct Dt;
impl HtmlElement for Dt {
    const TAG: &'static str = "dt";
}

/// The `<dd>` element - description details.
pub struct Dd;
impl HtmlElement for Dd {
    const TAG: &'static str = "dd";
}

/// The `<figure>` element - self-contained content.
pub struct Figure;
impl HtmlElement for Figure {
    const TAG: &'static str = "figure";
}
impl FlowContent for Figure {}
impl PalpableContent for Figure {}

/// The `<figcaption>` element - figure caption.
pub struct Figcaption;
impl HtmlElement for Figcaption {
    const TAG: &'static str = "figcaption";
}

/// The `<search>` element - search functionality.
pub struct Search;
impl HtmlElement for Search {
    const TAG: &'static str = "search";
}
impl FlowContent for Search {}
impl PalpableContent for Search {}

// =============================================================================
// Inline Text Semantics
// =============================================================================

/// The `<a>` element - hyperlink.
pub struct A;
impl HtmlElement for A {
    const TAG: &'static str = "a";
}
impl FlowContent for A {}
impl PhrasingContent for A {}
impl InteractiveContent for A {}
impl PalpableContent for A {}

/// The `<em>` element - emphasis.
pub struct Em;
impl HtmlElement for Em {
    const TAG: &'static str = "em";
}
impl FlowContent for Em {}
impl PhrasingContent for Em {}
impl PalpableContent for Em {}

/// The `<strong>` element - strong importance.
pub struct Strong;
impl HtmlElement for Strong {
    const TAG: &'static str = "strong";
}
impl FlowContent for Strong {}
impl PhrasingContent for Strong {}
impl PalpableContent for Strong {}

/// The `<small>` element - side comments.
pub struct Small;
impl HtmlElement for Small {
    const TAG: &'static str = "small";
}
impl FlowContent for Small {}
impl PhrasingContent for Small {}
impl PalpableContent for Small {}

/// The `<s>` element - no longer accurate.
pub struct S;
impl HtmlElement for S {
    const TAG: &'static str = "s";
}
impl FlowContent for S {}
impl PhrasingContent for S {}
impl PalpableContent for S {}

/// The `<cite>` element - citation.
pub struct Cite;
impl HtmlElement for Cite {
    const TAG: &'static str = "cite";
}
impl FlowContent for Cite {}
impl PhrasingContent for Cite {}
impl PalpableContent for Cite {}

/// The `<q>` element - inline quotation.
pub struct Q;
impl HtmlElement for Q {
    const TAG: &'static str = "q";
}
impl FlowContent for Q {}
impl PhrasingContent for Q {}
impl PalpableContent for Q {}

/// The `<dfn>` element - defining instance.
pub struct Dfn;
impl HtmlElement for Dfn {
    const TAG: &'static str = "dfn";
}
impl FlowContent for Dfn {}
impl PhrasingContent for Dfn {}
impl PalpableContent for Dfn {}

/// The `<abbr>` element - abbreviation.
pub struct Abbr;
impl HtmlElement for Abbr {
    const TAG: &'static str = "abbr";
}
impl FlowContent for Abbr {}
impl PhrasingContent for Abbr {}
impl PalpableContent for Abbr {}

/// The `<ruby>` element - ruby annotation.
pub struct Ruby;
impl HtmlElement for Ruby {
    const TAG: &'static str = "ruby";
}
impl FlowContent for Ruby {}
impl PhrasingContent for Ruby {}
impl PalpableContent for Ruby {}

/// The `<rt>` element - ruby text.
pub struct Rt;
impl HtmlElement for Rt {
    const TAG: &'static str = "rt";
}

/// The `<rp>` element - ruby parenthesis.
pub struct Rp;
impl HtmlElement for Rp {
    const TAG: &'static str = "rp";
}

/// The `<data>` element - machine-readable value.
pub struct Data;
impl HtmlElement for Data {
    const TAG: &'static str = "data";
}
impl FlowContent for Data {}
impl PhrasingContent for Data {}
impl PalpableContent for Data {}

/// The `<time>` element - date/time.
pub struct Time;
impl HtmlElement for Time {
    const TAG: &'static str = "time";
}
impl FlowContent for Time {}
impl PhrasingContent for Time {}
impl PalpableContent for Time {}

/// The `<code>` element - code fragment.
pub struct Code;
impl HtmlElement for Code {
    const TAG: &'static str = "code";
}
impl FlowContent for Code {}
impl PhrasingContent for Code {}
impl PalpableContent for Code {}

/// The `<var>` element - variable.
pub struct Var;
impl HtmlElement for Var {
    const TAG: &'static str = "var";
}
impl FlowContent for Var {}
impl PhrasingContent for Var {}
impl PalpableContent for Var {}

/// The `<samp>` element - sample output.
pub struct Samp;
impl HtmlElement for Samp {
    const TAG: &'static str = "samp";
}
impl FlowContent for Samp {}
impl PhrasingContent for Samp {}
impl PalpableContent for Samp {}

/// The `<kbd>` element - keyboard input.
pub struct Kbd;
impl HtmlElement for Kbd {
    const TAG: &'static str = "kbd";
}
impl FlowContent for Kbd {}
impl PhrasingContent for Kbd {}
impl PalpableContent for Kbd {}

/// The `<sub>` element - subscript.
pub struct Sub;
impl HtmlElement for Sub {
    const TAG: &'static str = "sub";
}
impl FlowContent for Sub {}
impl PhrasingContent for Sub {}
impl PalpableContent for Sub {}

/// The `<sup>` element - superscript.
pub struct Sup;
impl HtmlElement for Sup {
    const TAG: &'static str = "sup";
}
impl FlowContent for Sup {}
impl PhrasingContent for Sup {}
impl PalpableContent for Sup {}

/// The `<i>` element - idiomatic text.
pub struct I;
impl HtmlElement for I {
    const TAG: &'static str = "i";
}
impl FlowContent for I {}
impl PhrasingContent for I {}
impl PalpableContent for I {}

/// The `<b>` element - bring attention.
pub struct B;
impl HtmlElement for B {
    const TAG: &'static str = "b";
}
impl FlowContent for B {}
impl PhrasingContent for B {}
impl PalpableContent for B {}

/// The `<u>` element - unarticulated annotation.
pub struct U;
impl HtmlElement for U {
    const TAG: &'static str = "u";
}
impl FlowContent for U {}
impl PhrasingContent for U {}
impl PalpableContent for U {}

/// The `<mark>` element - highlighted text.
pub struct Mark;
impl HtmlElement for Mark {
    const TAG: &'static str = "mark";
}
impl FlowContent for Mark {}
impl PhrasingContent for Mark {}
impl PalpableContent for Mark {}

/// The `<bdi>` element - bidirectional isolate.
pub struct Bdi;
impl HtmlElement for Bdi {
    const TAG: &'static str = "bdi";
}
impl FlowContent for Bdi {}
impl PhrasingContent for Bdi {}
impl PalpableContent for Bdi {}

/// The `<bdo>` element - bidirectional override.
pub struct Bdo;
impl HtmlElement for Bdo {
    const TAG: &'static str = "bdo";
}
impl FlowContent for Bdo {}
impl PhrasingContent for Bdo {}
impl PalpableContent for Bdo {}

/// The `<span>` element - generic inline container.
pub struct Span;
impl HtmlElement for Span {
    const TAG: &'static str = "span";
}
impl FlowContent for Span {}
impl PhrasingContent for Span {}
impl PalpableContent for Span {}

/// The `<br>` element - line break.
pub struct Br;
impl HtmlElement for Br {
    const TAG: &'static str = "br";
    const VOID: bool = true;
}
impl FlowContent for Br {}
impl PhrasingContent for Br {}

/// The `<wbr>` element - word break opportunity.
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
