use anyhow::{Context as AnyhowContext, Result};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
};

use crate::config::Config;
use crate::embed::{send_action_embed, EmbedColor};
use crate::utils::{create_channel_with_permissions, has_special_role, update_channel_list};

pub fn register() -> CreateCommand {
    let name = "create";
    let description = "Create a new channel.";

    println!(" > /{} - {}", name, description);

    CreateCommand::new(name)
        .description(description)
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "name", "Name of the channel")
                .required(true),
        )
}

pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
) -> Result<()> {
    let guild_id = command
        .guild_id
        .context("This command can only be used in a server")?;

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

    let options = &command.data.options();
    let channel_name = if let Some(ResolvedOption {
        value: ResolvedValue::String(name),
        ..
    }) = options.first()
    {
        name
    } else {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Channel name is not valid!")
                .ephemeral(true),
        );
        command.create_response(&ctx.http, response).await?;
        return Ok(());
    };

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("Creating channel...")
            .ephemeral(true),
    );
    command.create_response(&ctx.http, response).await?;

    let channel =
        create_channel_with_permissions(ctx, config, channel_name, command.user.id, guild_id)
            .await?;

    update_channel_list(ctx, config).await?;

    send_action_embed(
        ctx,
        config.log_channel_id(),
        "created",
        command.user.id.get(),
        EmbedColor::Green,
        Some(channel.id),
    )
    .await?;

    command
        .edit_response(
            &ctx.http,
            serenity::all::EditInteractionResponse::new()
                .content(format!("Channel created successfully! <#{}>", channel.id)),
        )
        .await?;

    Ok(())
}