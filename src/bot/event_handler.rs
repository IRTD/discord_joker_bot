use std::process::Command;

use crate::quote::QuoteStack;

use serenity::{
    model::gateway::Ready,
    model::prelude::Message,
    prelude::{Context, EventHandler},
};

pub struct Handler {
    stack_ref: QuoteStack,
    mcip: String,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        log::warn!("Read new message: {}", msg.content);
        match msg.content.as_str() {
            "!mcip" => {
                match msg
                    .channel_id
                    .say(&ctx, self.mcip.clone())
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
        let mcip_bytes = Command::new("curl")
            .args(["-4", "ifconfig.me"])
            .output()
            .unwrap()
            .stdout;
        let mcip = String::from_utf8(mcip_bytes).unwrap();
        Handler { stack_ref, mcip }
    }
}
