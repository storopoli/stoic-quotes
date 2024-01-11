use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result};
use std::clone::Clone;

// Parse the data
lazy_static! {
    static ref QUOTES: Vec<Quote> = read_data().expect("failed to read data");
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

pub fn read_data() -> Result<Vec<Quote>> {
    let content = include_str!("../data/quotes.json");
    from_str(content)
}

pub fn random_quote() -> Quote {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..QUOTES.len());
    QUOTES[index].clone()
}
