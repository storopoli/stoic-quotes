use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result};
use std::clone::Clone;

lazy_static! {
    /// A vector of [`Quote`]s
    static ref QUOTES: Vec<Quote> = read_data().expect("failed to read data");
}

/// A quote with text and author
///
/// # Arguments
///
/// * `text` - A [`String`] that contains the quote text
/// * `author` - A [`String`] that contains the quote author
///
/// # Examples
///
/// ```
/// let quote = Quote {
///     text: "Hello, world!".to_string(),
///     author: "Anonymous".to_string(),
/// };
/// ```
#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

/// Reads the quotes from the JSON file
/// which is located at `src/data/quotes.json`
/// and returns a [`Vec`] of [`Quote`]s.
pub fn read_data() -> Result<Vec<Quote>> {
    let content = include_str!("../data/quotes.json");
    from_str(content)
}

/// Returns a random [`Quote`] from the [`Vec`] of [`Quote`]s: [`struct@QUOTES`].
/// Currently, this function uses [`rand::thread_rng()`] to generate
/// a random number between 0 and the length of [`struct@QUOTES`].
/// Then, it clones the [`struct@QUOTES`] at the random index and returns it.
pub fn random_quote() -> Quote {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..QUOTES.len());
    QUOTES[index].clone()
}
