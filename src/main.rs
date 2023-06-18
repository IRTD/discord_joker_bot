use serenity::model::prelude::*;
mod bot;
mod config;
mod quote;

use crate::config::Settings;
use bot::QuoteClient;

#[tokio::main]
async fn main() {
    start_logger();
    let config = Settings::get().unwrap();
    log::warn!("Loaded config");
    let mut client = QuoteClient::try_from(config).await.unwrap();
    log::warn!("Created client and QuoteStack");
    log::warn!("Starting Bot");
    client.discord.start().await.unwrap();
}

fn start_logger() {
    simplelog::SimpleLogger::init(simplelog::LevelFilter::Warn, simplelog::Config::default())
        .unwrap()
}
