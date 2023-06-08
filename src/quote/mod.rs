pub mod quote_error;

use crate::quote::quote_error::QuoteError;

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
