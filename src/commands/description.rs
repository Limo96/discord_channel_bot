use anyhow::{Context as AnyhowContext, Result};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditChannel, ResolvedOption,
    ResolvedValue,
};

use crate::config::Config;
use crate::embed::{send_action_embed, EmbedColor};
use crate::utils::{is_managed_channel, has_special_role, update_channel_list};

pub fn register() -> CreateCommand {
    let name = "description";
    let description = "Change the channel description.";

    println!(" > /{} - {}", name, description);

    CreateCommand::new(name)
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "text",
                "Channel description (leave empty to remove)",
            )
            .required(false),
        )
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
                .content("You do not have permission to use this command!")
                .ephemeral(true),
        );
        command.create_response(&ctx.http, response).await?;
        return Ok(());
    }

    // Check if this channel is managed by the bot
    if !is_managed_channel(ctx, config, channel_id).await? {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("This channel was not created by the bot!")
                .ephemeral(true),
        );
        command.create_response(&ctx.http, response).await?;
        return Ok(());
    }

    let options = &command.data.options();
    let description = if let Some(ResolvedOption {
        value: ResolvedValue::String(text),
        ..
    }) = options.first()
    {
        if text.is_empty() {
            String::new() // Empty string to clear the topic
        } else {
            text.to_string()
        }
    } else {
        String::new() // No parameter means clear the topic
    };

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("Updating description...")
            .ephemeral(true),
    );
    command.create_response(&ctx.http, response).await?;

    // Update the channel topic on Discord
    command
        .channel_id
        .edit(&ctx.http, EditChannel::new().topic(&description))
        .await?;

    update_channel_list(ctx, config).await?;

    send_action_embed(
        ctx,
        config.log_channel_id(),
        "description updated",
        command.user.id.get(),
        EmbedColor::Yellow,
        Some(command.channel_id),
    )
    .await?;

    command
        .edit_response(
            &ctx.http,
            serenity::all::EditInteractionResponse::new()
                .content("Description updated successfully!"),
        )
        .await?;

    Ok(())
}