use anyhow::{Context as AnyhowContext, Result};
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::config::Config;
use crate::embed::{send_action_embed, EmbedColor};
use crate::utils::{has_manager_role, update_channel_list};

pub fn register() -> CreateCommand {
    let name = "updatelist";
    let description = "Force refresh the channel list.";

    println!(" > /{} - {}", name, description);

    CreateCommand::new(name).description(description)
}

pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
) -> Result<()> {
    let member = command
        .member
        .as_ref()
        .context("Unable to get member data")?;

    if !has_manager_role(member, config).await {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Only managers can use this command!")
                .ephemeral(true),
        );
        command.create_response(&ctx.http, response).await?;
        return Ok(());
    }

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("Rebuilding the list...")
            .ephemeral(true),
    );
    command.create_response(&ctx.http, response).await?;

    // Simply rebuild the list from current Discord channels
    update_channel_list(ctx, config).await?;

    send_action_embed(
        ctx,
        config.log_channel_id(),
        "updated",
        command.user.id.get(),
        EmbedColor::Blue,
        Some(config.list_channel_id()),
    )
    .await?;

    command
        .edit_response(
            &ctx.http,
            serenity::all::EditInteractionResponse::new()
                .content("Channel list successfully rebuilt!"),
        )
        .await?;

    Ok(())
}