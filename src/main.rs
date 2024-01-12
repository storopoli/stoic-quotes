//! # Stoic Quotes
//!
//! `stoic-quotes` is a collection of stoic quotes in an Axum web server
//! that serves stoic quotes with reactivity by the all-mighty
//! [htmx](https://htmx.org) (no YAVASCRIPT).
//!
//! It also has plain-text API GET endpoints at `/` that returns a stoic quote
//! for terminal users with `curl` and `wget`.

use anyhow::{Context, Result};
use axum::serve;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod app;
mod data;
mod pages;

use app::app;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = app();

    // run our app with hyper, listening globally on port 443
    let port = 443_u16;
    let addr = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    info!("router initialized, listening on port {:?}", port);
    serve(addr, app)
        .await
        .context("error while starting server")?;
    Ok(())
}
