//! Example: Dynamic page generation based on Rust parameters
//!
//! This example demonstrates how to generate static HTML pages that vary
//! based on runtime data and conditionals - perfect for SSG (Static Site Generation).
//!
//! Inspired by: <https://github.com/LeakIX/zcash-web-wallet/>
//!
//! Run with: `cargo run --example wallet_dashboard`

use ironhtml::typed::{Document, Element};
use ironhtml_bootstrap::{alerts, buttons, cards, grid, Breakpoint, Color};
use ironhtml_elements::{
    Body, Br, Button, Code, Div, Footer, Head, Html, Input, Label, Link, Main, Meta, Nav, Script,
    Small, Span, Table, Tbody, Td, Th, Thead, Title, Tr, A, H2, H5, H6, I, P,
};

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

// ============================================================================
// DATA MODELS - These would come from your application
// ============================================================================

#[derive(Clone)]
struct WalletConfig {
    name: String,
    network: Network,
    currency_symbol: String,
    theme: Theme,
}

#[derive(Clone, Copy, PartialEq)]
enum Network {
    Mainnet,
    Testnet,
}

impl Network {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet => "testnet",
        }
    }

    const fn badge_color(self) -> Color {
        match self {
            Self::Mainnet => Color::Success,
            Self::Testnet => Color::Warning,
        }
    }
}

#[derive(Clone, Copy)]
enum Theme {
    Light,
    Dark,
}

struct Transaction {
    tx_id: String,
    amount: f64,
    is_incoming: bool,
    confirmations: u32,
    timestamp: String,
    address: String,
}

struct WalletBalance {
    total: f64,
    available: f64,
    pending: f64,
    locked: f64,
}

struct WalletState {
    config: WalletConfig,
    address: String,
    balance: WalletBalance,
    transactions: Vec<Transaction>,
    is_syncing: bool,
    sync_progress: u8,
    has_pending_tx: bool,
}

// ============================================================================
// REUSABLE COMPONENTS WITH CONDITIONAL RENDERING
// ============================================================================

/// Network badge - different color based on network type
fn network_badge(network: Network) -> Element<Span> {
    let class = format!("badge bg-{}", network.badge_color().as_str());
    Element::<Span>::new().class(&class).text(network.as_str())
}

/// Sync status indicator - shows spinner when syncing
fn sync_status(is_syncing: bool, progress: u8) -> Element<Div> {
    if is_syncing {
        Element::<Div>::new()
            .class("d-flex align-items-center text-warning")
            .child::<Div, _>(|d| {
                d.class("spinner-border spinner-border-sm me-2")
                    .attr("role", "status")
            })
            .child::<Span, _>(|s| {
                let text = format!("Syncing... {progress}%");
                s.text(&text)
            })
    } else {
        Element::<Div>::new()
            .class("text-success")
            .child::<I, _>(|i| i.class("bi bi-check-circle-fill me-2"))
            .child::<Span, _>(|s| s.text("Synced"))
    }
}

/// Balance card with conditional pending indicator
fn balance_card(balance: &WalletBalance, symbol: &str, has_pending: bool) -> Element<Div> {
    cards::card(|body| {
        let body = body
            .class("text-center")
            .child::<H6, _>(|h| h.class("text-muted mb-3").text("Total Balance"))
            .child::<H2, _>(|h| {
                let amount = format!("{symbol} {:.8}", balance.total);
                h.class("mb-3").text(&amount)
            });

        // Conditional: Show pending badge if there are pending transactions
        let body = if has_pending {
            body.child::<Div, _>(|d| {
                d.class("mb-3").child::<Span, _>(|s| {
                    s.class("badge bg-warning text-dark")
                        .child::<I, _>(|i| i.class("bi bi-clock me-1"))
                        .text("Pending transactions")
                })
            })
        } else {
            body
        };

        // Show balance breakdown
        body.child::<Div, _>(|d| {
            d.class("row text-start small")
                .child::<Div, _>(|c| {
                    c.class("col-6")
                        .child::<Div, _>(|row| {
                            let available = format!("{:.8}", balance.available);
                            row.class("d-flex justify-content-between")
                                .child::<Span, _>(|s| s.class("text-muted").text("Available"))
                                .child::<Span, _>(|s| s.text(&available))
                        })
                        .child::<Div, _>(|row| {
                            let pending = format!("{:.8}", balance.pending);
                            row.class("d-flex justify-content-between")
                                .child::<Span, _>(|s| s.class("text-muted").text("Pending"))
                                .child::<Span, _>(|s| {
                                    if balance.pending > 0.0 {
                                        s.class("text-warning").text(&pending)
                                    } else {
                                        s.text(&pending)
                                    }
                                })
                        })
                })
                .child::<Div, _>(|c| {
                    c.class("col-6").child::<Div, _>(|row| {
                        let locked = format!("{:.8}", balance.locked);
                        row.class("d-flex justify-content-between")
                            .child::<Span, _>(|s| s.class("text-muted").text("Locked"))
                            .child::<Span, _>(|s| {
                                if balance.locked > 0.0 {
                                    s.class("text-info").text(&locked)
                                } else {
                                    s.text(&locked)
                                }
                            })
                    })
                })
        })
    })
}

/// Transaction row - styling changes based on transaction properties
fn transaction_row(tx: &Transaction, symbol: &str) -> Element<Tr> {
    let amount_class = if tx.is_incoming {
        "text-success"
    } else {
        "text-danger"
    };

    let amount_prefix = if tx.is_incoming { "+" } else { "-" };
    let amount_text = format!("{amount_prefix}{:.8} {symbol}", tx.amount.abs());

    let status = match tx.confirmations {
        0 => ("Pending", "warning"),
        1..=5 => ("Confirming", "info"),
        _ => ("Confirmed", "success"),
    };

    Element::<Tr>::new()
        .child::<Td, _>(|td| {
            td.child::<Div, _>(|d| {
                d.class("d-flex align-items-center")
                    .child::<I, _>(|i| {
                        let icon = if tx.is_incoming {
                            "bi bi-arrow-down-circle-fill text-success me-2"
                        } else {
                            "bi bi-arrow-up-circle-fill text-danger me-2"
                        };
                        i.class(icon)
                    })
                    .child::<Div, _>(|inner| {
                        inner
                            .child::<Code, _>(|c| c.class("small").text(&tx.tx_id[..16]))
                            .child::<Br, _>(|br| br)
                            .child::<Small, _>(|s| s.class("text-muted").text(&tx.timestamp))
                    })
            })
        })
        .child::<Td, _>(|td| td.child::<Code, _>(|c| c.class("small").text(&tx.address[..20])))
        .child::<Td, _>(|td| {
            td.class("text-end")
                .child::<Span, _>(|s| s.class(amount_class).text(&amount_text))
        })
        .child::<Td, _>(|td| {
            let badge_class = format!("badge bg-{}", status.1);
            td.class("text-end")
                .child::<Span, _>(|s| s.class(&badge_class).text(status.0))
                .child::<Br, _>(|br| br)
                .child::<Small, _>(|s| {
                    let conf_text = format!("{} confirmations", tx.confirmations);
                    s.class("text-muted").text(&conf_text)
                })
        })
}

/// Transaction list - conditionally shows "no transactions" message
fn transaction_list(transactions: &[Transaction], symbol: &str) -> Element<Div> {
    cards::card(|body| {
        let body = body.child::<Div, _>(|d| {
            d.class("d-flex justify-content-between align-items-center mb-3")
                .child::<H5, _>(|h| h.class("mb-0").text("Recent Transactions"))
                .child::<A, _>(|a| {
                    a.class("btn btn-sm btn-outline-primary")
                        .attr("href", "#history")
                        .text("View All")
                })
        });

        if transactions.is_empty() {
            // Conditional: Show empty state when no transactions
            body.child::<Div, _>(|d| {
                d.class("text-center py-5 text-muted")
                    .child::<I, _>(|i| i.class("bi bi-inbox fs-1 mb-3 d-block"))
                    .child::<P, _>(|p| p.text("No transactions yet"))
                    .child::<P, _>(|p| {
                        p.class("small")
                            .text("Your transaction history will appear here")
                    })
            })
        } else {
            body.child::<Div, _>(|d| {
                d.class("table-responsive").child::<Table, _>(|table| {
                    table
                        .class("table table-hover mb-0")
                        .child::<Thead, _>(|thead| {
                            thead.child::<Tr, _>(|tr| {
                                tr.child::<Th, _>(|th| th.text("Transaction"))
                                    .child::<Th, _>(|th| th.text("Address"))
                                    .child::<Th, _>(|th| th.class("text-end").text("Amount"))
                                    .child::<Th, _>(|th| th.class("text-end").text("Status"))
                            })
                        })
                        .child::<Tbody, _>(|tbody| {
                            transactions.iter().fold(tbody, |tbody, tx| {
                                tbody.child::<Tr, _>(|_| transaction_row(tx, symbol))
                            })
                        })
                })
            })
        }
    })
}

/// Action buttons - different actions available based on wallet state
fn action_buttons(can_send: bool, can_receive: bool) -> Element<Div> {
    Element::<Div>::new()
        .class("d-flex gap-2 justify-content-center mb-4")
        .child::<Button, _>(|_| {
            let mut btn = buttons::btn(Color::Primary, "Send");
            if !can_send {
                btn = btn.bool_attr("disabled");
            }
            btn.child::<I, _>(|i| i.class("bi bi-send me-2"))
        })
        .child::<Button, _>(|_| {
            if can_receive {
                buttons::btn(Color::Success, "Receive")
                    .child::<I, _>(|i| i.class("bi bi-qr-code me-2"))
            } else {
                buttons::btn_disabled(Color::Success, "Receive")
            }
        })
        .child::<Button, _>(|_| {
            buttons::btn_outline(Color::Secondary, "History")
                .child::<I, _>(|i| i.class("bi bi-clock-history me-2"))
        })
}

/// Address display with copy button
fn address_display(address: &str) -> Element<Div> {
    Element::<Div>::new()
        .class("input-group mb-3")
        .child::<Span, _>(|s| {
            s.class("input-group-text")
                .child::<I, _>(|i| i.class("bi bi-wallet2"))
        })
        .child::<Input, _>(|input| {
            input
                .attr("type", "text")
                .class("form-control font-monospace")
                .attr("value", address)
                .bool_attr("readonly")
        })
        .child::<Button, _>(|btn| {
            btn.class("btn btn-outline-secondary")
                .attr("type", "button")
                .attr(
                    "onclick",
                    "navigator.clipboard.writeText(this.previousElementSibling.value)",
                )
                .child::<I, _>(|i| i.class("bi bi-clipboard"))
        })
}

// ============================================================================
// PAGE GENERATION - The main function that builds the page from state
// ============================================================================

/// Wallet page navbar with network and sync indicators
fn wallet_navbar(state: &WalletState) -> Element<Nav> {
    Element::<Nav>::new()
        .class("navbar navbar-expand-lg bg-body-tertiary mb-4")
        .child::<Div, _>(|_| {
            grid::container(|c| {
                c.class("d-flex justify-content-between align-items-center")
                    .child::<A, _>(|a| {
                        a.class("navbar-brand fw-bold")
                            .attr("href", "#")
                            .text(&state.config.name)
                    })
                    .child::<Div, _>(|d| {
                        d.class("d-flex align-items-center gap-3")
                            .child::<Span, _>(|_| network_badge(state.config.network))
                            .child::<Div, _>(|_| sync_status(state.is_syncing, state.sync_progress))
                    })
            })
        })
}

/// Generate the complete wallet dashboard based on wallet state
fn generate_wallet_page(state: &WalletState) -> Document {
    let can_send = state.balance.available > 0.0 && !state.is_syncing;
    let can_receive = !state.is_syncing;

    Document::new()
        .doctype()
        .root::<Html, _>(|html| {
            let mut html = html.attr("lang", "en");
            if matches!(state.config.theme, Theme::Dark) {
                html = html.attr("data-bs-theme", "dark");
            }

            html.child::<Head, _>(|head| {
                    head.child::<Meta, _>(|m| m.attr("charset", "UTF-8"))
                        .child::<Meta, _>(|m| {
                            m.attr("name", "viewport")
                                .attr("content", "width=device-width, initial-scale=1")
                        })
                        .child::<Title, _>(|t| {
                            let title = format!("{} Wallet", state.config.name);
                            t.text(&title)
                        })
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
                    let body_class = match state.config.theme {
                        Theme::Light => "bg-light",
                        Theme::Dark => "bg-dark",
                    };

                    body.class(body_class)
                        .child::<Nav, _>(|_| wallet_navbar(state))
                        // Main content
                        .child::<Main, _>(|main| {
                            main.child::<Div, _>(|_| {
                                grid::container(|c| {
                                    c.child::<Div, _>(|_| {
                                        grid::row_gutter(4, |r| {
                                            // Left column: Balance and actions
                                            r.child::<Div, _>(|_| {
                                                grid::col_bp(Breakpoint::Md, 4, |col| {
                                                    col.child::<Div, _>(|_| balance_card(&state.balance, &state.config.currency_symbol, state.has_pending_tx))
                                                        .child::<Div, _>(|d| {
                                                            d.class("mt-4")
                                                                .child::<Div, _>(|_| action_buttons(can_send, can_receive))
                                                        })
                                                        .child::<Div, _>(|d| {
                                                            d.class("mt-4")
                                                                .child::<Label, _>(|l| l.class("form-label small text-muted").text("Your Address"))
                                                                .child::<Div, _>(|_| address_display(&state.address))
                                                        })
                                                        // Conditional: Show testnet warning
                                                        .when(state.config.network == Network::Testnet, |col| {
                                                            col.child::<Div, _>(|_| {
                                                                alerts::alert(Color::Warning, "You are on testnet. Coins have no real value.")
                                                            })
                                                        })
                                                })
                                            })
                                            // Right column: Transactions
                                            .child::<Div, _>(|_| {
                                                grid::col_bp(Breakpoint::Md, 8, |col| {
                                                    col.child::<Div, _>(|_| transaction_list(&state.transactions, &state.config.currency_symbol))
                                                })
                                            })
                                        })
                                    })
                                })
                            })
                        })
                        // Footer
                        .child::<Footer, _>(|f| {
                            f.class("py-3 mt-4")
                                .child::<Div, _>(|_| {
                                    grid::container(|c| {
                                        c.class("text-center text-muted small")
                                            .child::<P, _>(|p| {
                                                let version = format!("{} Wallet v1.0.0", state.config.name);
                                                p.class("mb-0").text(&version)
                                            })
                                    })
                                })
                        })
                        // Bootstrap JS
                        .child::<Script, _>(|s| {
                            s.attr("src", "https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js")
                        })
                })
        })
}

// ============================================================================
// MAIN: Generate pages for different wallet states
// ============================================================================

fn main() {
    // Example 1: Active mainnet wallet with transactions
    let mainnet_wallet = WalletState {
        config: WalletConfig {
            name: "Zcash".into(),
            network: Network::Mainnet,
            currency_symbol: "ZEC".into(),
            theme: Theme::Light,
        },
        address: "t1Rv4exT7bqhZqi2j7xz8bUHDMxwosrjADU".into(),
        balance: WalletBalance {
            total: 12.456_789_01,
            available: 10.123_456_78,
            pending: 2.333_332_23,
            locked: 0.0,
        },
        transactions: vec![
            Transaction {
                tx_id: "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6".into(),
                amount: 5.5,
                is_incoming: true,
                confirmations: 142,
                timestamp: "2024-01-15 14:32".into(),
                address: "t1KzLcPzUnkA8GqXEPrLBLf4bRFzRYLZmCk".into(),
            },
            Transaction {
                tx_id: "b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7".into(),
                amount: 2.333_332_23,
                is_incoming: true,
                confirmations: 2,
                timestamp: "2024-01-15 13:15".into(),
                address: "t1NRzLcPzUnkA8GqXEPrLBLf4bRFzRYLZmC".into(),
            },
            Transaction {
                tx_id: "c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8".into(),
                amount: 1.2,
                is_incoming: false,
                confirmations: 0,
                timestamp: "2024-01-15 12:00".into(),
                address: "t1XYzLcPzUnkA8GqXEPrLBLf4bRFzRYLZmC".into(),
            },
        ],
        is_syncing: false,
        sync_progress: 100,
        has_pending_tx: true,
    };

    // Example 2: New testnet wallet (no transactions)
    let testnet_wallet = WalletState {
        config: WalletConfig {
            name: "Zcash".into(),
            network: Network::Testnet,
            currency_symbol: "TAZ".into(),
            theme: Theme::Dark,
        },
        address: "tm9k2VqE9xPVdN8NNqXTABPeJr4GtSM8GGo".into(),
        balance: WalletBalance {
            total: 0.0,
            available: 0.0,
            pending: 0.0,
            locked: 0.0,
        },
        transactions: vec![],
        is_syncing: true,
        sync_progress: 67,
        has_pending_tx: false,
    };

    // Generate both pages
    println!("=== MAINNET WALLET ===\n");
    let mainnet_html = generate_wallet_page(&mainnet_wallet).render();
    println!("{mainnet_html}");

    println!("\n\n=== TESTNET WALLET (syncing, empty) ===\n");
    let testnet_html = generate_wallet_page(&testnet_wallet).render();
    println!("{testnet_html}");

    // Show that the same function generates different HTML based on state
    println!("\n\n=== DEMONSTRATION ===");
    println!("The generate_wallet_page() function produces different HTML based on:");
    println!("  - Network (mainnet/testnet) → different badge colors, warning messages");
    println!("  - Theme (light/dark) → different body classes, data attributes");
    println!("  - Sync status → spinner vs checkmark");
    println!("  - Balance amounts → conditional styling for pending/locked");
    println!("  - Transaction list → empty state vs table");
    println!("  - Confirmations → different status badges");
    println!("  - Can send/receive → enabled/disabled buttons");
}
