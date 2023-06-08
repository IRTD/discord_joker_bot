mod quote;

use quote::Quote;
use std::fs::read_to_string;

fn main() {
    let content = read_to_string("/home/irtd/code/joker-bot/joker_quote.txt").unwrap();
    for line in content.split("\n") {
        if let Ok(q) = Quote::try_from(line.to_string()) {
            println!("{}", q.quote());
        }
    }
}
