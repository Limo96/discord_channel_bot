mod commands;
mod config;
mod embed;
mod utils;

use anyhow::Result;
use serenity::all::{
    Command, CreateInteractionResponse, CreateInteractionResponseMessage, GatewayIntents,
    Interaction, Ready,
};
use serenity::async_trait;
use serenity::prelude::*;

use config::Config;

struct Handler {
    config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let result = match command.data.name.as_str() {
                "create" => commands::create::run(&ctx, &command, &self.config).await,
                "delete" => commands::delete::run(&ctx, &command, &self.config).await,
                "rename" => commands::rename::run(&ctx, &command, &self.config).await,
                "description" => commands::description::run(&ctx, &command, &self.config).await,
                "updatelist" => commands::updatelist::run(&ctx, &command, &self.config).await,
                _ => Ok(()),
            };

            if let Err(e) = result {
                eprintln!("Error executing command: {:?}", e);
                let response = CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("Error: {}", e))
                        .ephemeral(true),
                );
                let _ = command.create_response(&ctx.http, response).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected!", ready.user.name);

        let commands = vec![
            commands::create::register(),
            commands::delete::register(),
            commands::rename::register(),
            commands::description::register(),
            commands::updatelist::register(),
        ];

        for command in commands {
            if let Err(e) = Command::create_global_command(&ctx.http, command).await {
                eprintln!("Error registering command: {:?}", e);
            }
        }

        println!("Commands registered successfully!");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load("config.toml")?;

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler {
            config: config.clone(),
        })
        .await?;

    println!("Bot is starting...");
    client.start().await?;

    Ok(())
}