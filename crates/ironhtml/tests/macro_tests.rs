//! Integration tests for the html! macro.

#![cfg(feature = "macros")]

use ironhtml::html;

#[test]
fn test_simple_element() {
    let elem = html! { div };
    assert_eq!(elem.render(), "<div></div>");
}

#[test]
fn test_element_with_class() {
    let elem = html! { div.class("container") };
    assert_eq!(elem.render(), r#"<div class="container"></div>"#);
}

#[test]
fn test_element_with_multiple_classes() {
    let elem = html! { div.class("btn").class("btn-primary") };
    assert_eq!(elem.render(), r#"<div class="btn btn-primary"></div>"#);
}

#[test]
fn test_element_with_id() {
    let elem = html! { div.id("main") };
    assert_eq!(elem.render(), r#"<div id="main"></div>"#);
}

#[test]
fn test_element_with_text() {
    let elem = html! { p { "Hello, World!" } };
    assert_eq!(elem.render(), "<p>Hello, World!</p>");
}

#[test]
fn test_element_with_multiple_text() {
    let elem = html! { p { "Hello, " "World!" } };
    assert_eq!(elem.render(), "<p>Hello, World!</p>");
}

#[test]
fn test_nested_elements() {
    let elem = html! {
        div {
            span { "Hello" }
        }
    };
    assert_eq!(elem.render(), "<div><span>Hello</span></div>");
}

#[test]
fn test_deeply_nested() {
    let elem = html! {
        div.class("outer") {
            div.class("inner") {
                span { "Deep" }
            }
        }
    };
    assert_eq!(
        elem.render(),
        r#"<div class="outer"><div class="inner"><span>Deep</span></div></div>"#
    );
}

#[test]
fn test_multiple_children() {
    let elem = html! {
        ul {
            li { "One" }
            li { "Two" }
            li { "Three" }
        }
    };
    assert_eq!(
        elem.render(),
        "<ul><li>One</li><li>Two</li><li>Three</li></ul>"
    );
}

#[test]
fn test_void_element() {
    let elem = html! { br };
    assert_eq!(elem.render(), "<br />");
}

#[test]
fn test_img_element() {
    let elem = html! { img.src("image.jpg").alt("An image") };
    assert_eq!(elem.render(), r#"<img src="image.jpg" alt="An image" />"#);
}

#[test]
fn test_input_element() {
    let elem = html! { input.type_("text").name("username").placeholder("Enter username") };
    assert_eq!(
        elem.render(),
        r#"<input type="text" name="username" placeholder="Enter username" />"#
    );
}

#[test]
fn test_anchor_element() {
    let elem = html! { a.href("/").target("_blank") { "Home" } };
    assert_eq!(elem.render(), r#"<a href="/" target="_blank">Home</a>"#);
}

#[test]
fn test_boolean_attribute() {
    let elem = html! { input.type_("checkbox").checked.disabled };
    assert_eq!(
        elem.render(),
        r#"<input type="checkbox" checked disabled />"#
    );
}

#[test]
fn test_expression_in_text() {
    let name = "World";
    let elem = html! { p { "Hello, " #name "!" } };
    assert_eq!(elem.render(), "<p>Hello, World!</p>");
}

#[test]
fn test_expression_in_attribute() {
    let class_name = "container";
    let elem = html! { div.class(#class_name) };
    assert_eq!(elem.render(), r#"<div class="container"></div>"#);
}

#[test]
fn test_for_loop() {
    let items = ["Apple", "Banana", "Cherry"];
    let elem = html! {
        ul {
            for item in #items {
                li { #item }
            }
        }
    };
    assert_eq!(
        elem.render(),
        "<ul><li>Apple</li><li>Banana</li><li>Cherry</li></ul>"
    );
}

#[test]
fn test_for_loop_with_index() {
    let items: Vec<String> = vec!["A", "B", "C"]
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{}: {}", i + 1, item))
        .collect();
    let elem = html! {
        ol {
            for item in #items {
                li { #item }
            }
        }
    };
    assert_eq!(
        elem.render(),
        "<ol><li>1: A</li><li>2: B</li><li>3: C</li></ol>"
    );
}

#[test]
fn test_conditional() {
    let show = true;
    let elem = html! {
        div {
            if #show {
                span { "Visible" }
            }
        }
    };
    assert_eq!(elem.render(), "<div><span>Visible</span></div>");
}

#[test]
fn test_conditional_false() {
    let show = false;
    let elem = html! {
        div {
            if #show {
                span { "Hidden" }
            }
        }
    };
    assert_eq!(elem.render(), "<div></div>");
}

#[test]
fn test_table() {
    let elem = html! {
        table.class("table") {
            thead {
                tr {
                    th { "Name" }
                    th { "Age" }
                }
            }
            tbody {
                tr {
                    td { "Alice" }
                    td { "30" }
                }
            }
        }
    };
    let html = elem.render();
    assert!(html.contains(r#"<table class="table">"#));
    assert!(html.contains("<th>Name</th>"));
    assert!(html.contains("<td>Alice</td>"));
}

#[test]
fn test_form() {
    let elem = html! {
        form.action("/submit").method("post") {
            input.type_("text").name("email")
            button.type_("submit") { "Submit" }
        }
    };
    let html = elem.render();
    assert!(html.contains(r#"<form action="/submit" method="post">"#));
    assert!(html.contains(r#"<input type="text" name="email" />"#));
    assert!(html.contains("<button type=\"submit\">Submit</button>"));
}

#[test]
fn test_nav_with_links() {
    let elem = html! {
        nav.class("navbar") {
            ul.class("nav") {
                li { a.href("/") { "Home" } }
                li { a.href("/about") { "About" } }
                li { a.href("/contact") { "Contact" } }
            }
        }
    };
    let html = elem.render();
    assert!(html.contains(r#"<nav class="navbar">"#));
    assert!(html.contains(r#"<a href="/">Home</a>"#));
    assert!(html.contains(r#"<a href="/about">About</a>"#));
}

#[test]
fn test_select_options() {
    let elem = html! {
        select.name("country") {
            option.value("us") { "United States" }
            option.value("uk") { "United Kingdom" }
            option.value("ca") { "Canada" }
        }
    };
    let html = elem.render();
    assert!(html.contains(r#"<select name="country">"#));
    assert!(html.contains(r#"<option value="us">United States</option>"#));
}

#[test]
fn test_data_attribute() {
    let elem = html! { div.data_id("123").data_action("submit") };
    assert_eq!(
        elem.render(),
        r#"<div data-id="123" data-action="submit"></div>"#
    );
}

#[test]
fn test_aria_attribute() {
    let elem = html! { button.aria_label("Close").aria_hidden("false") { "X" } };
    assert_eq!(
        elem.render(),
        r#"<button aria-label="Close" aria-hidden="false">X</button>"#
    );
}

#[test]
fn test_complex_page() {
    let title = "My Page";
    let items = vec!["Item 1", "Item 2"];

    let elem = html! {
        div.class("page") {
            header {
                h1 { #title }
            }
            main {
                p { "Welcome to my page" }
                ul {
                    for item in #items {
                        li { #item }
                    }
                }
            }
            footer {
                p { "Copyright 2024" }
            }
        }
    };

    let html = elem.render();
    assert!(html.contains("<h1>My Page</h1>"));
    assert!(html.contains("<li>Item 1</li>"));
    assert!(html.contains("<li>Item 2</li>"));
    assert!(html.contains("Copyright 2024"));
}
