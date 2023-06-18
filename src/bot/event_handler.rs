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
        if msg.content == "!quote" {
            match msg
                .channel_id
                .say(&ctx, self.stack_ref.rand_quote().quote())
                .await
            {
                Ok(_) => log::warn!("Responded"),
                Err(e) => log::error!("EventHandler: {}", e),
            }
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
