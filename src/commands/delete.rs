use anyhow::{Context as AnyhowContext, Result};
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::config::Config;
use crate::embed::{send_action_embed, EmbedColor};
use crate::utils::{get_managed_channels, has_special_role, update_channel_list};

pub fn register() -> CreateCommand {
    let name = "delete";
    let description = "Delete the channel.";

    println!(" > /{} - {}", name, description);

    CreateCommand::new(name).description(description)
}

pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
) -> Result<()> {
    let channel_id = command.channel_id.get();

    let member = command
        .member
        .as_ref()
        .context("Unable to get member data")?;

    if !has_special_role(member, config).await {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("You don't have permission to use this command!")
                .ephemeral(true),
        );
        command.create_response(&ctx.http, response).await?;
        return Ok(());
    }

    // Check if this channel is managed by the bot
    let channels = get_managed_channels(ctx, config).await?;
    let channel_info = channels.iter().find(|c| c.channel_id == channel_id);

    if channel_info.is_none() {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("This channel was not created by the bot and cannot be deleted!")
                .ephemeral(true),
        );
        command.create_response(&ctx.http, response).await?;
        return Ok(());
    }

    let channel_name = channel_info.map(|c| c.name.clone()).unwrap_or_else(|| "unknown".to_string());

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("Deleting channel...")
            .ephemeral(true),
    );
    command.create_response(&ctx.http, response).await?;

    send_action_embed(
        ctx,
        config.log_channel_id(),
        format!("[{}] deleted", channel_name).as_str(),
        command.user.id.get(),
        EmbedColor::Red,
        None,
    )
    .await?;

    // Delete the channel
    command.channel_id.delete(&ctx.http).await?;

    // Update the list (will automatically exclude the deleted channel)
    update_channel_list(ctx, config).await?;

    Ok(())
}