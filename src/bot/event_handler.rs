use std::{process::Command, time::Instant};
use tokio::sync::Mutex;

use crate::quote::QuoteStack;

use serenity::{
    model::gateway::Ready,
    model::prelude::Message,
    prelude::{Context, EventHandler},
};

const REFRESH_HOURS: usize = 2;

pub struct Handler {
    stack_ref: QuoteStack,
    mcip: Mutex<String>,
    refresh_time: Mutex<Instant>,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        log::warn!("Read new message: {}", msg.content);
        match msg.content.as_str() {
            "!mcip" => {
                let mut refresh_time = self.refresh_time.lock().await;
                let mut mcip = self.mcip.lock().await;
                if refresh_time.elapsed().as_secs() >= ( 60 ^ 2 * REFRESH_HOURS ) as u64  {
                    *mcip = fetch_public_ip().unwrap();
                    *refresh_time = Instant::now();
                }
                match msg
                    .channel_id
                    .say(&ctx, mcip.clone())
                    .await
                {
                    Ok(_) => log::warn!("Responded"),
                    Err(e) => log::error!("EventHandler: {}", e),
                }
            },
            "!quote" => {
                match msg
                    .channel_id
                    .say(&ctx, self.stack_ref.rand_quote().quote())
                    .await
                {
                    Ok(_) => log::warn!("Responded"),
                    Err(e) => log::error!("EventHandler: {}", e),
                }
            },
            _ => {}
        }
    }

    async fn ready(&self, _: Context, r: Ready) {
        println!("Bot is ready as '{}'", r.user.name);
    }
}

impl Handler {
    pub fn new(stack_ref: QuoteStack) -> Handler {
        let mcip = fetch_public_ip().unwrap().into();
        Handler { stack_ref, mcip, refresh_time: Instant::now().into() }
    }
}

fn fetch_public_ip() -> anyhow::Result<String> {
    let mcip_bytes = Command::new("curl")
        .args(["-4", "ifconfig.me"])
        .output()?
        .stdout;
    Ok(String::from_utf8(mcip_bytes)?)
}
