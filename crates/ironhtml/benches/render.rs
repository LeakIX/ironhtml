use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ironhtml::html;
use ironhtml::typed::Element;
use ironhtml_elements::Li;

// ============================================================================
// Escape functions
// ============================================================================

fn bench_escape(c: &mut Criterion) {
    let plain = "Hello, World! This is a plain string with no special chars.";
    let mixed = r#"<div class="foo">Hello & "World" it's here</div>"#;

    let mut group = c.benchmark_group("escape");

    group.bench_function("html_plain", |b| {
        b.iter(|| ironhtml::escape_html(black_box(plain)));
    });

    group.bench_function("html_mixed", |b| {
        b.iter(|| ironhtml::escape_html(black_box(mixed)));
    });

    group.bench_function("attr_plain", |b| {
        b.iter(|| ironhtml::escape_attr(black_box(plain)));
    });

    group.bench_function("attr_mixed", |b| {
        b.iter(|| ironhtml::escape_attr(black_box(mixed)));
    });

    group.finish();
}

// ============================================================================
// html! macro benchmarks
// ============================================================================

fn bench_macro_single_element(c: &mut Criterion) {
    c.bench_function("macro/single_div", |b| {
        b.iter(|| {
            html! {
                div.class("container mx-auto").id("main") {
                    "Hello, World!"
                }
            }
            .render()
        });
    });
}

fn bench_macro_deep_nesting(c: &mut Criterion) {
    c.bench_function("macro/deep_nesting_5", |b| {
        b.iter(|| {
            html! {
                div.class("level-1") {
                    div.class("level-2") {
                        div.class("level-3") {
                            div.class("level-4") {
                                div.class("level-5") {
                                    span { "deep" }
                                }
                            }
                        }
                    }
                }
            }
            .render()
        });
    });
}

fn bench_macro_full_page(c: &mut Criterion) {
    c.bench_function("macro/full_page", |b| {
        b.iter(|| {
            html! {
                html.lang("en") {
                    head {
                        meta.charset("UTF-8")
                        meta
                            .name("viewport")
                            .content(
                                "width=device-width, initial-scale=1"
                            )
                        title { "Benchmark Page" }
                    }
                    body {
                        div.class("container") {
                            h1 { "Hello" }
                            p { "A paragraph of text." }
                            ul {
                                li {
                                    a.href("/") { "Home" }
                                }
                                li {
                                    a.href("/about") { "About" }
                                }
                                li {
                                    a.href("/contact") {
                                        "Contact"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            .render()
        });
    });
}

fn bench_macro_children_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("macro/children");

    for size in [10, 50, 100, 500, 1000] {
        let items: Vec<&str> = (0..size).map(|_| "item").collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &items, |b, items| {
            b.iter(|| {
                html! {
                    ul {
                        for item in #items {
                            li { #*item }
                        }
                    }
                }
                .render()
            });
        });
    }

    group.finish();
}

fn bench_macro_table_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("macro/table_rows");

    for size in [10, 50, 100, 500] {
        let rows: Vec<(usize, &str, &str)> = (0..size)
            .map(|i| (i, "Alice", "alice@example.com"))
            .collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &rows, |b, rows| {
            b.iter(|| {
                let rows = black_box(rows);
                html! {
                    table.class("table table-striped") {
                        thead {
                            tr {
                                th { "#" }
                                th { "Name" }
                                th { "Email" }
                            }
                        }
                        tbody {
                            for row in #rows {
                                tr {
                                    td { #row.0.to_string() }
                                    td { #row.1 }
                                    td { #row.2 }
                                }
                            }
                        }
                    }
                }
                .render()
            });
        });
    }

    group.finish();
}

fn bench_macro_conditional(c: &mut Criterion) {
    c.bench_function("macro/conditional_true", |b| {
        let show = true;
        b.iter(|| {
            html! {
                div.class("container") {
                    if #show {
                        div.class("alert alert-success") {
                            "Operation successful!"
                        }
                    }
                    p { "Content below." }
                }
            }
            .render()
        });
    });

    c.bench_function("macro/conditional_false", |b| {
        let show = false;
        b.iter(|| {
            html! {
                div.class("container") {
                    if #show {
                        div.class("alert alert-success") {
                            "Operation successful!"
                        }
                    }
                    p { "Content below." }
                }
            }
            .render()
        });
    });
}

// ============================================================================
// Typed API (direct) for comparison
// ============================================================================

fn bench_typed_children_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("typed/children");

    for size in [10, 50, 100, 500, 1000] {
        let items: Vec<&str> = (0..size).map(|_| "item").collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &items, |b, items| {
            b.iter(|| {
                Element::<ironhtml_elements::Ul>::new()
                    .children(black_box(items), |item, li: Element<Li>| li.text(*item))
                    .render()
            });
        });
    }

    group.finish();
}

// ============================================================================
// Untyped API for comparison
// ============================================================================

fn bench_untyped_children_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("untyped/children");

    for size in [10, 50, 100, 500, 1000] {
        let items: Vec<&str> = (0..size).map(|_| "item").collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &items, |b, items| {
            b.iter(|| {
                ironhtml::Element::new("ul")
                    .children(black_box(items), |item, _| {
                        ironhtml::Element::new("li").text(*item)
                    })
                    .render()
            });
        });
    }

    group.finish();
}

// ============================================================================
// Criterion groups
// ============================================================================

criterion_group!(
    benches,
    bench_escape,
    bench_macro_single_element,
    bench_macro_deep_nesting,
    bench_macro_children_sizes,
    bench_macro_table_sizes,
    bench_macro_full_page,
    bench_macro_conditional,
    bench_typed_children_sizes,
    bench_untyped_children_sizes,
);
criterion_main!(benches);
