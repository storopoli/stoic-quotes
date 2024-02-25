//! # Stoic Quotes
//!
//! `stoic-quotes` is a collection of stoic quotes in an Axum web server
//! that serves stoic quotes with reactivity by the all-mighty
//! [htmx](https://htmx.org) (no YAVASCRIPT).
//!
//! It also has plain-text API GET endpoints at `/` that returns a stoic quote
//! for terminal users with `curl` and `wget`.

#![allow(non_snake_case)]

use dioxus_web::launch;

#[cfg(debug_assertions)]
use log::{info, LevelFilter};

mod app;
mod component;
mod data;

use app::App;

pub fn main() {
    #[cfg(debug_assertions)]
    {
        // init logger for Dioxus
        dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    }
    // launch the web app
    #[cfg(debug_assertions)]
    info!("Launching Stoic Quotes app");
    launch(App);
}
