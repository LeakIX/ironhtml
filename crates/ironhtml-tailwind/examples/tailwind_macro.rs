//! Example demonstrating Tailwind CSS with the html! macro
//!
//! Run with: `cargo run --example tailwind_macro`

use ironhtml::html;

#[allow(clippy::too_many_lines)]
fn main() {
    // Simple card component using html! macro with Tailwind classes
    let card = html! {
        div.class("bg-white rounded-lg shadow-md p-6 hover:shadow-lg") {
            h2.class("text-2xl font-bold text-gray-900 mb-2") {
                "Welcome to ironhtml"
            }
            p.class("text-gray-600 mb-4") {
                "Type-safe HTML with Tailwind CSS utilities"
            }
            button.class("bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600") {
                "Get Started"
            }
        }
    };

    println!("<!-- Card Component -->");
    println!("{}", card.render());
    println!();

    // Responsive grid layout with dynamic content
    let features = vec![
        ("Fast", "Blazing fast compilation"),
        ("Safe", "Type-safe at compile time"),
        ("Easy", "Intuitive API design"),
    ];

    let grid = html! {
        div.class("grid grid-cols-1 md:grid-cols-3 gap-6") {
            for feature in #features {
                div.class("bg-gray-50 p-6 rounded-lg border border-gray-200") {
                    h3.class("text-xl font-bold text-blue-700 mb-2") {
                        #feature.0
                    }
                    p.class("text-gray-600") {
                        #feature.1
                    }
                }
            }
        }
    };

    println!("<!-- Feature Grid -->");
    println!("{}", grid.render());
    println!();

    // Conditional rendering with state
    let is_active = true;
    let status_badge = html! {
        div.class("inline-flex items-center") {
            if #is_active {
                span.class("bg-green-100 text-green-800 px-3 py-1 rounded-full text-sm font-medium") {
                    "Active"
                }
            }
        }
    };

    println!("<!-- Status Badge -->");
    println!("{}", status_badge.render());
    println!();

    // Complete page with Tailwind CDN
    let page = html! {
        html {
            head {
                meta.charset("UTF-8")
                meta.name("viewport").content("width=device-width, initial-scale=1.0")
                title { "Tailwind Macro Demo" }
                script.src("https://cdn.tailwindcss.com") { }
            }
            body.class("bg-gray-50 min-h-screen") {
                div.class("container mx-auto px-4 py-8") {
                    h1.class("text-4xl font-bold text-gray-900 mb-8") {
                        "Tailwind CSS with html! Macro"
                    }
                    
                    // Hero section
                    div.class("bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded-xl p-8 mb-8") {
                        h2.class("text-3xl font-bold mb-4") {
                            "Build Beautiful UIs"
                        }
                        p.class("text-lg mb-6") {
                            "Combine the power of Rust's type system with Tailwind CSS"
                        }
                        div.class("flex gap-4") {
                            button.class("bg-white text-blue-600 px-6 py-3 rounded-lg font-semibold hover:bg-gray-100") {
                                "Learn More"
                            }
                            button.class("border-2 border-white px-6 py-3 rounded-lg font-semibold hover:bg-white hover:text-blue-600") {
                                "Documentation"
                            }
                        }
                    }

                    // Feature cards
                    div.class("grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8") {
                        div.class("bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow") {
                            div.class("text-4xl mb-4") { "🚀" }
                            h3.class("text-xl font-bold mb-2") { "Fast Development" }
                            p.class("text-gray-600") {
                                "Rapid prototyping with utility-first CSS"
                            }
                        }
                        div.class("bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow") {
                            div.class("text-4xl mb-4") { "🛡️" }
                            h3.class("text-xl font-bold mb-2") { "Type Safety" }
                            p.class("text-gray-600") {
                                "Catch errors at compile time, not runtime"
                            }
                        }
                        div.class("bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow") {
                            div.class("text-4xl mb-4") { "📦" }
                            h3.class("text-xl font-bold mb-2") { "No Dependencies" }
                            p.class("text-gray-600") {
                                "Pure Rust with no JavaScript build step"
                            }
                        }
                    }

                    // Code example section
                    div.class("bg-gray-900 text-gray-100 rounded-lg p-6") {
                        h3.class("text-xl font-bold mb-4 text-white") {
                            "Example Usage"
                        }
                        pre.class("bg-gray-800 p-4 rounded overflow-x-auto") {
                            code.class("text-sm") {
"let card = html! {
    div.class(\"bg-white p-6 rounded shadow\") {
        h2.class(\"text-xl font-bold\") { \"Hello\" }
        p.class(\"text-gray-600\") { \"World\" }
    }
};"
                            }
                        }
                    }
                }
            }
        }
    };

    println!("<!-- Full Page -->");
    println!("<!DOCTYPE html>");
    println!("{}", page.render());
}
