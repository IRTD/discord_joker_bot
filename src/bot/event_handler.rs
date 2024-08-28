use crate::quote::QuoteStack;

use serenity::{
    model::gateway::Ready,
    model::prelude::Message,
    prelude::{Context, EventHandler},
};

pub struct Handler {
    stack_ref: QuoteStack,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        log::warn!("Read new message: {}", msg.content);
        match msg.content.as_str() {
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
            "!mcip" => {
                let ip = match public_ip::addr_v4().await {
                    Some(ip) => format!("{:?}", ip),
                    None => String::from("Uh idk no IP lol"),
                };
                match msg
                    .channel_id
                    .say(&ctx, ip)
                    .await
                {
                    Ok(_) => log::warn!("Responded"),
                    Err(e) => log::error!("EventHandler: {}", e),
                }
            }
            _ => {}
        }
    }

    async fn ready(&self, _: Context, r: Ready) {
        println!("Bot is ready as '{}'", r.user.name);
    }
}

impl Handler {
    pub fn new(stack_ref: QuoteStack) -> Handler {
        Handler { stack_ref }
    }
}
