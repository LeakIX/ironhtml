//! # ironhtml-macro
//!
//! Procedural macro for type-safe HTML construction with Rust-like syntax.
//!
//! This crate provides the [`html!`] macro that generates type-safe HTML
//! using the `ironhtml` and `ironhtml-elements` crates. Most users should
//! depend on `ironhtml` with the `macros` feature instead of using this
//! crate directly.
//!
//! See [`ironhtml::html!`](https://docs.rs/ironhtml/latest/ironhtml/macro.html.html)
//! for full documentation and tested examples covering elements, attributes,
//! text content, Rust expressions, loops, and conditionals.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::{braced, token, Expr, Ident, LitStr, Result, Token};

/// The main HTML macro for type-safe HTML construction.
///
/// See the [crate-level documentation](crate) for syntax and examples.
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let node = syn::parse_macro_input!(input as Node);
    let expanded = node.to_token_stream();
    expanded.into()
}

/// A node in the HTML tree: element, text, expression, loop, or conditional.
enum Node {
    Element(ElementNode),
    Text(LitStr),
    Expr(Expr),
    For(ForLoop),
    If(IfNode),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if input.peek(Token![#]) {
            input.parse::<Token![#]>()?;
            Ok(Self::Expr(input.parse()?))
        } else if input.peek(Token![for]) {
            Ok(Self::For(input.parse()?))
        } else if input.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else if input.peek(Ident) {
            Ok(Self::Element(input.parse()?))
        } else {
            Err(input.error("expected element, text literal, or expression"))
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Element(elem) => elem.to_tokens(tokens),
            Self::Text(lit) => {
                tokens.extend(quote! { .text(#lit) });
            }
            Self::Expr(expr) => {
                tokens.extend(quote! { .text(#expr) });
            }
            Self::For(for_loop) => for_loop.to_tokens(tokens),
            Self::If(if_node) => if_node.to_tokens(tokens),
        }
    }
}

/// An HTML element with tag, attributes, and children.
struct ElementNode {
    tag: Ident,
    attrs: Vec<Attribute>,
    children: Vec<Node>,
}

impl Parse for ElementNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag: Ident = input.parse()?;

        // Parse attributes (method chain style: .class("x").id("y"))
        let mut attrs = Vec::new();
        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            attrs.push(input.parse()?);
        }

        // Parse children (inside braces)
        let children = if input.peek(token::Brace) {
            let content;
            braced!(content in input);
            let mut children = Vec::new();
            while !content.is_empty() {
                children.push(content.parse()?);
            }
            children
        } else {
            Vec::new()
        };

        Ok(Self {
            tag,
            attrs,
            children,
        })
    }
}

impl ToTokens for ElementNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let tag = &self.tag;
        let tag_pascal = to_pascal_case(&tag.to_string());
        let tag_ident = Ident::new(&tag_pascal, tag.span());

        // Generate attribute calls
        let attr_calls: Vec<_> = self
            .attrs
            .iter()
            .map(quote::ToTokens::to_token_stream)
            .collect();

        if self.children.is_empty() {
            // No children - just create element with attributes
            tokens.extend(quote! {
                ::ironhtml::typed::Element::<::ironhtml_elements::#tag_ident>::new()
                    #(#attr_calls)*
            });
        } else {
            // Has children - need to generate child calls
            let mut child_tokens = TokenStream2::new();

            for child in &self.children {
                match child {
                    Node::Element(elem) => {
                        let child_tag = &elem.tag;
                        let child_pascal = to_pascal_case(&child_tag.to_string());
                        let child_ident = Ident::new(&child_pascal, child_tag.span());

                        let child_attrs: Vec<_> = elem
                            .attrs
                            .iter()
                            .map(quote::ToTokens::to_token_stream)
                            .collect();

                        if elem.children.is_empty() {
                            child_tokens.extend(quote! {
                                .child::<::ironhtml_elements::#child_ident, _>(|e| e #(#child_attrs)*)
                            });
                        } else {
                            let nested = generate_children(&elem.children);
                            child_tokens.extend(quote! {
                                .child::<::ironhtml_elements::#child_ident, _>(|e| e #(#child_attrs)* #nested)
                            });
                        }
                    }
                    Node::Text(lit) => {
                        child_tokens.extend(quote! { .text(#lit) });
                    }
                    Node::Expr(expr) => {
                        child_tokens.extend(quote! { .text(#expr) });
                    }
                    Node::For(for_loop) => {
                        for_loop.to_tokens(&mut child_tokens);
                    }
                    Node::If(if_node) => {
                        if_node.to_tokens(&mut child_tokens);
                    }
                }
            }

            tokens.extend(quote! {
                ::ironhtml::typed::Element::<::ironhtml_elements::#tag_ident>::new()
                    #(#attr_calls)*
                    #child_tokens
            });
        }
    }
}

/// Generate token stream for a list of child nodes.
fn generate_children(children: &[Node]) -> TokenStream2 {
    let mut tokens = TokenStream2::new();

    for child in children {
        match child {
            Node::Element(elem) => {
                let child_tag = &elem.tag;
                let child_pascal = to_pascal_case(&child_tag.to_string());
                let child_ident = Ident::new(&child_pascal, child_tag.span());

                let child_attrs: Vec<_> = elem
                    .attrs
                    .iter()
                    .map(quote::ToTokens::to_token_stream)
                    .collect();

                if elem.children.is_empty() {
                    tokens.extend(quote! {
                        .child::<::ironhtml_elements::#child_ident, _>(|e| e #(#child_attrs)*)
                    });
                } else {
                    let nested = generate_children(&elem.children);
                    tokens.extend(quote! {
                        .child::<::ironhtml_elements::#child_ident, _>(|e| e #(#child_attrs)* #nested)
                    });
                }
            }
            Node::Text(lit) => {
                tokens.extend(quote! { .text(#lit) });
            }
            Node::Expr(expr) => {
                tokens.extend(quote! { .text(#expr) });
            }
            Node::For(for_loop) => {
                for_loop.to_tokens(&mut tokens);
            }
            Node::If(if_node) => {
                if_node.to_tokens(&mut tokens);
            }
        }
    }

    tokens
}

/// An attribute on an element: name(value) or name (boolean).
struct Attribute {
    name: Ident,
    value: Option<AttrValue>,
}

enum AttrValue {
    Lit(LitStr),
    Expr(Expr),
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let value = if input.peek(token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            if content.peek(Token![#]) {
                content.parse::<Token![#]>()?;
                Some(AttrValue::Expr(content.parse()?))
            } else if content.peek(LitStr) {
                Some(AttrValue::Lit(content.parse()?))
            } else {
                Some(AttrValue::Expr(content.parse()?))
            }
        } else {
            None
        };

        Ok(Self { name, value })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let name_str = name.to_string();

        // Handle special attribute names
        let method_name = match name_str.as_str() {
            "class" | "id" => name.clone(),
            _ => Ident::new("attr", name.span()),
        };

        // Convert attribute name: remove trailing underscore and replace underscores with hyphens
        // e.g., type_ -> type, data_id -> data-id, aria_label -> aria-label
        let convert_attr_name = |s: &str| -> String { s.trim_end_matches('_').replace('_', "-") };

        match &self.value {
            Some(AttrValue::Lit(lit)) => {
                if name_str == "class" || name_str == "id" {
                    tokens.extend(quote! { .#method_name(#lit) });
                } else {
                    let attr_name = convert_attr_name(&name_str);
                    tokens.extend(quote! { .#method_name(#attr_name, #lit) });
                }
            }
            Some(AttrValue::Expr(expr)) => {
                if name_str == "class" || name_str == "id" {
                    tokens.extend(quote! { .#method_name(#expr) });
                } else {
                    let attr_name = convert_attr_name(&name_str);
                    tokens.extend(quote! { .#method_name(#attr_name, #expr) });
                }
            }
            None => {
                // Boolean attribute
                let attr_name = convert_attr_name(&name_str);
                tokens.extend(quote! { .bool_attr(#attr_name) });
            }
        }
    }
}

/// A for loop: for item in #expr { children }
struct ForLoop {
    pat: syn::Pat,
    expr: Expr,
    children: Vec<Node>,
}

impl Parse for ForLoop {
    fn parse(input: ParseStream) -> Result<Self> {
        let for_token: Token![for] = input.parse()?;
        let pat = syn::Pat::parse_single(input)?;
        input.parse::<Token![in]>()?;
        input.parse::<Token![#]>()?;

        // Parse expression but stop before brace (use ExprPath or similar)
        // We need to be careful not to consume the following brace
        let expr = parse_expr_before_brace(input)?;

        let content;
        braced!(content in input);
        let mut children = Vec::new();
        while !content.is_empty() {
            children.push(content.parse()?);
        }

        if children.len() != 1 || !matches!(children.first(), Some(Node::Element(_))) {
            return Err(syn::Error::new(
                for_token.span,
                "for loop body must contain exactly one element",
            ));
        }

        Ok(Self {
            pat,
            expr,
            children,
        })
    }
}

/// Parse an expression that stops before a brace.
fn parse_expr_before_brace(input: ParseStream) -> Result<Expr> {
    // Fork to try parsing without consuming
    let fork = input.fork();

    // Try to parse as a simple path/ident first
    if let Ok(path) = fork.parse::<syn::ExprPath>() {
        // Check if next token is brace
        if fork.peek(token::Brace) {
            input.advance_to(&fork);
            return Ok(Expr::Path(path));
        }
    }

    // Otherwise parse a full expression
    input.parse()
}

impl ToTokens for ForLoop {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pat = &self.pat;
        let expr = &self.expr;

        // For loops need to know the child element type
        // We expect exactly one child element in the loop body
        if let Some(Node::Element(elem)) = self.children.first() {
            let child_tag = &elem.tag;
            let child_pascal = to_pascal_case(&child_tag.to_string());
            let child_ident = Ident::new(&child_pascal, child_tag.span());

            let child_attrs: Vec<_> = elem
                .attrs
                .iter()
                .map(quote::ToTokens::to_token_stream)
                .collect();
            let nested = generate_children(&elem.children);

            tokens.extend(quote! {
                .children(#expr, |#pat, e: ::ironhtml::typed::Element<::ironhtml_elements::#child_ident>| {
                    e #(#child_attrs)* #nested
                })
            });
        }
    }
}

/// An if conditional: if #expr { children }
struct IfNode {
    cond: Expr,
    children: Vec<Node>,
}

impl Parse for IfNode {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![if]>()?;
        input.parse::<Token![#]>()?;

        // Parse expression but stop before brace
        let cond = parse_expr_before_brace(input)?;

        let content;
        braced!(content in input);
        let mut children = Vec::new();
        while !content.is_empty() {
            children.push(content.parse()?);
        }

        Ok(Self { cond, children })
    }
}

impl ToTokens for IfNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let cond = &self.cond;
        let child_tokens = generate_children(&self.children);

        tokens.extend(quote! {
            .when(#cond, |e| e #child_tokens)
        });
    }
}

/// Convert `snake_case` or lowercase to `PascalCase`.
fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    // Handle special cases for HTML elements
    match result.as_str() {
        "A" => "A".to_string(),
        "B" => "B".to_string(),
        "I" => "I".to_string(),
        "P" => "P".to_string(),
        "Q" => "Q".to_string(),
        "S" => "S".to_string(),
        "U" => "U".to_string(),
        "Br" => "Br".to_string(),
        "Hr" => "Hr".to_string(),
        "H1" => "H1".to_string(),
        "H2" => "H2".to_string(),
        "H3" => "H3".to_string(),
        "H4" => "H4".to_string(),
        "H5" => "H5".to_string(),
        "H6" => "H6".to_string(),
        "Dl" => "Dl".to_string(),
        "Dt" => "Dt".to_string(),
        "Dd" => "Dd".to_string(),
        "Li" => "Li".to_string(),
        "Ol" => "Ol".to_string(),
        "Ul" => "Ul".to_string(),
        "Td" => "Td".to_string(),
        "Th" => "Th".to_string(),
        "Tr" => "Tr".to_string(),
        "Em" => "Em".to_string(),
        "Rp" => "Rp".to_string(),
        "Rt" => "Rt".to_string(),
        "Rb" => "Rb".to_string(),
        "Rtc" => "Rtc".to_string(),
        "Wbr" => "Wbr".to_string(),
        "Kbd" => "Kbd".to_string(),
        "Pre" => "Pre".to_string(),
        "Sub" => "Sub".to_string(),
        "Sup" => "Sup".to_string(),
        "Var" => "Var".to_string(),
        "Bdi" => "Bdi".to_string(),
        "Bdo" => "Bdo".to_string(),
        "Col" => "Col".to_string(),
        "Del" => "Del".to_string(),
        "Dfn" => "Dfn".to_string(),
        "Div" => "Div".to_string(),
        "Img" => "Img".to_string(),
        "Ins" => "Ins".to_string(),
        "Map" => "Map".to_string(),
        "Nav" => "Nav".to_string(),
        "Svg" => "Svg".to_string(),
        // Option is special - ironhtml-elements uses Option_
        "Option" => "Option_".to_string(),
        _ => result,
    }
}
