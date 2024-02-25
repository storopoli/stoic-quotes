//! Module that has functions and structs that handles all the data parsing
//! into stoic quotes.

use axum_browser_adapter::wasm_compat;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result};
use std::clone::Clone;

/// A quote with text and author
///
/// # Arguments
///
/// * `text` - A [`String`] that contains the quote text
/// * `author` - A [`String`] that contains the quote author
#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

/// Reads the quotes from the JSON file
/// which is located at `src/data/quotes.json`
/// and returns a [`Vec`] of [`Quote`]s.
#[wasm_compat]
pub async fn read_data() -> Result<Vec<Quote>> {
    let content = include_str!("../data/quotes.json");
    from_str(content)
}

/// Returns a random [`Quote`] from the [`Vec`] of [`Quote`]s: [`struct@QUOTES`].
/// Currently, this function uses [`rand::thread_rng()`] to generate
/// a random number between 0 and the length of [`struct@QUOTES`].
/// Then, it clones the [`struct@QUOTES`] at the random index and returns it.
#[wasm_compat]
pub async fn random_quote() -> Quote {
    let quotes = read_data().await.unwrap();
    let mut rng = thread_rng();
    let index = rng.gen_range(0..quotes.len());
    quotes[index].clone()
}
