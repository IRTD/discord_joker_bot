pub mod event_handler;

use crate::config::Settings;
use crate::quote::QuoteStack;
use event_handler::Handler;

use anyhow::Result;
use serenity::prelude::GatewayIntents;
use serenity::Client;

pub struct QuoteClient {
    pub discord: Client,
}

impl QuoteClient {
    pub async fn try_from(settings: Settings) -> Result<QuoteClient> {
        let quotes = QuoteStack::from_file(settings.quote_file_path)?;
        let handler = Handler::new(quotes);
        let discord = Client::builder(settings.discord_client.token, GatewayIntents::all())
            .event_handler(handler)
            .await?;

        Ok(QuoteClient { discord })
    }
}
