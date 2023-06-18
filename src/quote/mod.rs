pub mod quote_error;

use crate::quote::quote_error::QuoteError;

use anyhow::Result;
use rand::Rng;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Quote {
    pub quote: String,
    pub name: String,
}

impl TryFrom<String> for Quote {
    type Error = QuoteError;
    fn try_from(s: String) -> Result<Quote, QuoteError> {
        let (_, first) = match s.split_once('.') {
            Some(f) => f,
            None => return Err(QuoteError),
        };

        let quote = match first.rsplit_once('—') {
            Some((q, _)) => q.to_string(),
            None => return Err(QuoteError),
        };

        let quote = clean_raw_quote(quote);
        let name = "The Joker".to_string();

        Ok(Quote { quote, name })
    }
}

fn clean_raw_quote(quote: String) -> String {
    let quote = quote.replace('"', "").split_off(1);
    let quote = quote[0..quote.len() - 1].to_string();
    quote
}

impl Quote {
    pub fn quote(&self) -> String {
        format!("{:?} - {}", self.quote, self.name)
    }
}

pub struct QuoteStack {
    quotes: Vec<Quote>,
}

impl QuoteStack {
    pub fn from_file<S: Into<PathBuf>>(file: S) -> Result<Self> {
        let content = read_to_string(file.into())?;
        let mut quotes = Vec::new();
        for line in content.split("\n") {
            if let Ok(q) = Quote::try_from(line.to_string()) {
                quotes.push(q);
            }
        }

        Ok(QuoteStack { quotes })
    }

    pub fn rand_quote(&self) -> &Quote {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..self.quotes.len());
        &self.quotes[n]
    }
}
