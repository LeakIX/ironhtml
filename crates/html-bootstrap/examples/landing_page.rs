//! Example: Building a complete SaaS landing page with Bootstrap
//!
//! This example demonstrates how to build reusable components
//! and compose them into a full landing page - just like React!
//!
//! Run with: cargo run --example landing_page

use html_bootstrap::*;
use html_builder::typed::{Document, Element};
use html_elements::*;

// ============================================================================
// DATA MODELS
// ============================================================================

struct MenuItem {
    href: &'static str,
    label: &'static str,
    active: bool,
}

struct Feature {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
}

struct PricingTier {
    name: &'static str,
    price: &'static str,
    period: &'static str,
    features: &'static [&'static str],
    highlighted: bool,
    cta: &'static str,
}

struct Testimonial {
    quote: &'static str,
    author: &'static str,
    role: &'static str,
}

// ============================================================================
// REUSABLE COMPONENTS
// ============================================================================

/// Navigation bar component - reused across all pages
fn main_navbar(brand: &str, items: &[MenuItem]) -> Element<Nav> {
    navbar::navbar(brand, NavbarExpand::Lg, "mainNav", |ul| {
        items.iter().fold(ul, |ul, item| {
            ul.child::<Li, _>(|_| navbar::nav_item(item.href, item.label, item.active))
        })
    })
}

/// Hero section with gradient background
fn hero_section(
    title: &str,
    subtitle: &str,
    cta_primary: &str,
    cta_secondary: &str,
) -> Element<Div> {
    Element::<Div>::new()
        .class("bg-primary text-white py-5")
        .attr(
            "style",
            "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);",
        )
        .child::<Div, _>(|_| {
            grid::container(|c| {
                c.class("py-5 text-center")
                    .child::<H1, _>(|h| h.class("display-3 fw-bold mb-3").text(title))
                    .child::<P, _>(|p| {
                        p.class("lead mb-4 mx-auto")
                            .attr("style", "max-width: 600px;")
                            .text(subtitle)
                    })
                    .child::<Div, _>(|d| {
                        d.class("d-flex gap-3 justify-content-center")
                            .child::<A, _>(|a| {
                                a.class("btn btn-light btn-lg px-4")
                                    .attr("href", "#signup")
                                    .text(cta_primary)
                            })
                            .child::<A, _>(|a| {
                                a.class("btn btn-outline-light btn-lg px-4")
                                    .attr("href", "#demo")
                                    .text(cta_secondary)
                            })
                    })
            })
        })
}

/// Feature card - reusable component
fn feature_card(feature: &Feature) -> Element<Div> {
    cards::card(|body| {
        body.class("text-center h-100 border-0 shadow-sm")
            .child::<Div, _>(|d| {
                d.class("text-primary mb-3")
                    .attr("style", "font-size: 3rem;")
                    .child::<I, _>(|i| i.class(feature.icon))
            })
            .child::<H5, _>(|h| h.class("card-title").text(feature.title))
            .child::<P, _>(|p| p.class("card-text text-muted").text(feature.description))
    })
}

/// Features section
fn features_section(title: &str, subtitle: &str, features: &[Feature]) -> Element<Section> {
    Element::<Section>::new()
        .class("py-5")
        .id("features")
        .child::<Div, _>(|_| {
            grid::container(|c| {
                c.child::<Div, _>(|d| {
                    d.class("text-center mb-5")
                        .child::<H2, _>(|h| h.class("fw-bold").text(title))
                        .child::<P, _>(|p| p.class("text-muted").text(subtitle))
                })
                .child::<Div, _>(|_| {
                    grid::row_gutter(4, |r| {
                        features.iter().fold(r, |row, feature| {
                            row.child::<Div, _>(|_| {
                                grid::col(4, |col| col.child::<Div, _>(|_| feature_card(feature)))
                            })
                        })
                    })
                })
            })
        })
}

/// Pricing card - reusable component
fn pricing_card(tier: &PricingTier) -> Element<Div> {
    let card_class = if tier.highlighted {
        "card h-100 border-primary shadow"
    } else {
        "card h-100 shadow-sm"
    };

    Element::<Div>::new()
        .class(card_class)
        .child::<Div, _>(|body| {
            let body = body.class("card-body d-flex flex-column");
            let body = if tier.highlighted {
                body.child::<Span, _>(|s| {
                    s.class("badge bg-primary text-white position-absolute")
                        .attr("style", "top: -10px; right: 10px;")
                        .text("Popular")
                })
            } else {
                body
            };

            body.child::<H4, _>(|h| h.class("card-title text-center").text(tier.name))
                .child::<Div, _>(|d| {
                    d.class("text-center mb-4")
                        .child::<Span, _>(|s| s.class("display-4 fw-bold").text(tier.price))
                        .child::<Span, _>(|s| s.class("text-muted").text(tier.period))
                })
                .child::<Ul, _>(|ul| {
                    tier.features
                        .iter()
                        .fold(ul.class("list-unstyled mb-4"), |ul, feature| {
                            ul.child::<Li, _>(|li| {
                                li.class("mb-2")
                                    .child::<I, _>(|i| {
                                        i.class("bi bi-check-circle-fill text-success me-2")
                                    })
                                    .text(*feature)
                            })
                        })
                })
                .child::<Div, _>(|d| {
                    d.class("mt-auto").child::<A, _>(|a| {
                        let class = alloc::format!(
                            "btn btn-{} w-100 py-2",
                            if tier.highlighted {
                                "primary"
                            } else {
                                "outline-primary"
                            }
                        );
                        a.class(&class).attr("href", "#signup").text(tier.cta)
                    })
                })
        })
}

/// Pricing section
fn pricing_section(title: &str, tiers: &[PricingTier]) -> Element<Section> {
    Element::<Section>::new()
        .class("py-5 bg-light")
        .id("pricing")
        .child::<Div, _>(|_| {
            grid::container(|c| {
                c.child::<H2, _>(|h| h.class("text-center fw-bold mb-5").text(title))
                    .child::<Div, _>(|_| {
                        grid::row_gutter(4, |r| {
                            tiers.iter().fold(r, |row, tier| {
                                row.child::<Div, _>(|_| {
                                    grid::col(4, |col| col.child::<Div, _>(|_| pricing_card(tier)))
                                })
                            })
                        })
                    })
            })
        })
}

/// Testimonial card - reusable component
fn testimonial_card(testimonial: &Testimonial) -> Element<Div> {
    cards::card(|body| {
        body.class("h-100 border-0 shadow-sm")
            .child::<Blockquote, _>(|bq| {
                bq.class("blockquote mb-4")
                    .child::<P, _>(|p| {
                        p.child::<I, _>(|i| i.class("bi bi-quote text-primary me-2"))
                            .text(testimonial.quote)
                    })
            })
            .child::<Div, _>(|d| {
                d.class("d-flex align-items-center")
                    .child::<Div, _>(|avatar| {
                        avatar
                            .class("rounded-circle bg-primary text-white d-flex align-items-center justify-content-center me-3")
                            .attr("style", "width: 48px; height: 48px;")
                            .text(testimonial.author.chars().next().unwrap().to_string())
                    })
                    .child::<Div, _>(|info| {
                        info.child::<Strong, _>(|s| s.text(testimonial.author))
                            .child::<Br, _>(|br| br)
                            .child::<Small, _>(|s| s.class("text-muted").text(testimonial.role))
                    })
            })
    })
}

/// Testimonials section
fn testimonials_section(testimonials: &[Testimonial]) -> Element<Section> {
    Element::<Section>::new()
        .class("py-5")
        .id("testimonials")
        .child::<Div, _>(|_| {
            grid::container(|c| {
                c.child::<H2, _>(|h| {
                    h.class("text-center fw-bold mb-5")
                        .text("What Our Customers Say")
                })
                .child::<Div, _>(|_| {
                    grid::row_gutter(4, |r| {
                        testimonials.iter().fold(r, |row, t| {
                            row.child::<Div, _>(|_| {
                                grid::col(4, |col| col.child::<Div, _>(|_| testimonial_card(t)))
                            })
                        })
                    })
                })
            })
        })
}

/// Footer component
fn footer(company: &str, year: &str) -> Element<Footer> {
    Element::<Footer>::new()
        .class("bg-dark text-white py-4")
        .child::<Div, _>(|_| {
            grid::container(|c| {
                c.child::<Div, _>(|_| {
                    grid::row(|r| {
                        r.child::<Div, _>(|_| {
                            grid::col(6, |col| {
                                col.child::<H5, _>(|h| h.text(company)).child::<P, _>(|p| {
                                    p.class("text-muted")
                                        .text("Building the future of web development.")
                                })
                            })
                        })
                        .child::<Div, _>(|_| {
                            grid::col(3, |col| {
                                col.child::<H6, _>(|h| h.text("Product"))
                                    .child::<Ul, _>(|ul| {
                                        ul.class("list-unstyled")
                                            .child::<Li, _>(|li| {
                                                li.child::<A, _>(|a| {
                                                    a.class("text-muted text-decoration-none")
                                                        .attr("href", "#")
                                                        .text("Features")
                                                })
                                            })
                                            .child::<Li, _>(|li| {
                                                li.child::<A, _>(|a| {
                                                    a.class("text-muted text-decoration-none")
                                                        .attr("href", "#")
                                                        .text("Pricing")
                                                })
                                            })
                                            .child::<Li, _>(|li| {
                                                li.child::<A, _>(|a| {
                                                    a.class("text-muted text-decoration-none")
                                                        .attr("href", "#")
                                                        .text("Documentation")
                                                })
                                            })
                                    })
                            })
                        })
                        .child::<Div, _>(|_| {
                            grid::col(3, |col| {
                                col.child::<H6, _>(|h| h.text("Company"))
                                    .child::<Ul, _>(|ul| {
                                        ul.class("list-unstyled")
                                            .child::<Li, _>(|li| {
                                                li.child::<A, _>(|a| {
                                                    a.class("text-muted text-decoration-none")
                                                        .attr("href", "#")
                                                        .text("About")
                                                })
                                            })
                                            .child::<Li, _>(|li| {
                                                li.child::<A, _>(|a| {
                                                    a.class("text-muted text-decoration-none")
                                                        .attr("href", "#")
                                                        .text("Blog")
                                                })
                                            })
                                            .child::<Li, _>(|li| {
                                                li.child::<A, _>(|a| {
                                                    a.class("text-muted text-decoration-none")
                                                        .attr("href", "#")
                                                        .text("Contact")
                                                })
                                            })
                                    })
                            })
                        })
                    })
                })
                .child::<Hr, _>(|hr| hr.class("my-4"))
                .child::<P, _>(|p| {
                    let copyright = alloc::format!("© {} {}. All rights reserved.", year, company);
                    p.class("text-center text-muted mb-0").text(&copyright)
                })
            })
        })
}

// ============================================================================
// MAIN: Compose everything into a full page
// ============================================================================

fn main() {
    // Data for our page
    let menu_items = vec![
        MenuItem {
            href: "#features",
            label: "Features",
            active: false,
        },
        MenuItem {
            href: "#pricing",
            label: "Pricing",
            active: false,
        },
        MenuItem {
            href: "#testimonials",
            label: "Testimonials",
            active: false,
        },
        MenuItem {
            href: "#contact",
            label: "Contact",
            active: false,
        },
    ];

    let features = vec![
        Feature {
            icon: "bi bi-lightning-charge-fill",
            title: "Blazing Fast",
            description:
                "Optimized for performance with zero runtime overhead. Your pages load instantly.",
        },
        Feature {
            icon: "bi bi-shield-check",
            title: "Type Safe",
            description: "Catch errors at compile time. No more runtime surprises in production.",
        },
        Feature {
            icon: "bi bi-puzzle-fill",
            title: "Composable",
            description:
                "Build complex UIs from simple, reusable components. Just like React, but in Rust.",
        },
    ];

    let pricing_tiers = vec![
        PricingTier {
            name: "Starter",
            price: "$9",
            period: "/month",
            features: &[
                "Up to 5 projects",
                "Basic analytics",
                "Community support",
                "1GB storage",
            ],
            highlighted: false,
            cta: "Start Free Trial",
        },
        PricingTier {
            name: "Professional",
            price: "$29",
            period: "/month",
            features: &[
                "Unlimited projects",
                "Advanced analytics",
                "Priority support",
                "10GB storage",
                "Custom domains",
            ],
            highlighted: true,
            cta: "Get Started",
        },
        PricingTier {
            name: "Enterprise",
            price: "$99",
            period: "/month",
            features: &[
                "Everything in Pro",
                "Dedicated support",
                "SLA guarantee",
                "Unlimited storage",
                "SSO integration",
            ],
            highlighted: false,
            cta: "Contact Sales",
        },
    ];

    let testimonials = vec![
        Testimonial {
            quote: "html-bootstrap has revolutionized how we build web applications. The type safety is incredible!",
            author: "Sarah Chen",
            role: "Lead Developer at TechCorp",
        },
        Testimonial {
            quote: "Finally, a way to write Bootstrap UIs in Rust without the chaos of string templates.",
            author: "Marcus Johnson",
            role: "CTO at StartupXYZ",
        },
        Testimonial {
            quote: "The component reuse pattern is exactly what we needed. Our codebase is so much cleaner now.",
            author: "Emily Rodriguez",
            role: "Senior Engineer at BigCo",
        },
    ];

    // Build the complete page
    let page = Document::new()
        .doctype()
        .root::<Html, _>(|html| {
            html.attr("lang", "en")
                .child::<Head, _>(|head| {
                    head.child::<Meta, _>(|m| m.attr("charset", "UTF-8"))
                        .child::<Meta, _>(|m| {
                            m.attr("name", "viewport")
                                .attr("content", "width=device-width, initial-scale=1")
                        })
                        .child::<Title, _>(|t| t.text("html-bootstrap - Type-Safe Bootstrap for Rust"))
                        .child::<Link, _>(|l| {
                            l.attr("href", "https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css")
                                .attr("rel", "stylesheet")
                        })
                        .child::<Link, _>(|l| {
                            l.attr("href", "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.0/font/bootstrap-icons.css")
                                .attr("rel", "stylesheet")
                        })
                })
                .child::<Body, _>(|body| {
                    body
                        // Navigation
                        .child::<Nav, _>(|_| main_navbar("html-bootstrap", &menu_items))
                        // Hero
                        .child::<Div, _>(|_| hero_section(
                            "Build Bootstrap UIs in Rust",
                            "Type-safe, composable, and blazing fast. Create beautiful web applications with the power of Rust's type system.",
                            "Get Started Free",
                            "View Demo",
                        ))
                        // Features
                        .child::<Section, _>(|_| features_section(
                            "Why Choose html-bootstrap?",
                            "Everything you need to build modern web applications",
                            &features,
                        ))
                        // Pricing
                        .child::<Section, _>(|_| pricing_section("Simple, Transparent Pricing", &pricing_tiers))
                        // Testimonials
                        .child::<Section, _>(|_| testimonials_section(&testimonials))
                        // Footer
                        .child::<Footer, _>(|_| footer("html-bootstrap", "2024"))
                        // Bootstrap JS
                        .child::<Script, _>(|s| {
                            s.attr("src", "https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js")
                        })
                })
        });

    // Output the HTML
    println!("{}", page.render());
}

extern crate alloc;
