use anyhow::Result;
use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage, Timestamp};

pub async fn send_log_embed(
    ctx: &Context,
    channel_id: ChannelId,
    description: &str,
    color: Option<u32>,
) -> Result<()> {
    let embed = CreateEmbed::new()
        .description(description)
        .color(color.unwrap_or(5814783))
        .timestamp(Timestamp::now());

    let message = CreateMessage::new().embed(embed);

    channel_id.send_message(&ctx.http, message).await?;
    Ok(())
}

pub async fn send_action_embed(
    ctx: &Context,
    channel_id: ChannelId,
    action: &str,
    user_id: u64,
    color: EmbedColor,
    list_channel_id: Option<ChannelId>,
) -> Result<()> {
    let description = if let Some(list_id) = list_channel_id {
        format!("<#{}> {} by <@{}>", list_id.get(), action, user_id)
    } else {
        format!("{} by <@{}>\n", action, user_id)
    };

    send_log_embed(ctx, channel_id, &description, Some(color.value())).await
}

/// Send an embed for the channel list
/// Format: [#Channel] - description by @User
pub async fn send_list_embed(
    ctx: &Context,
    list_channel_id: ChannelId,
    channel_id: ChannelId,
    description: Option<&String>,
    creator_id: u64,
) -> Result<()> {
    let desc_text = if let Some(desc) = description {
        format!(" - {}", desc)
    } else {
        String::new()
    };

    let embed_description = format!("<#{}>{} by <@{}>", channel_id, desc_text, creator_id);

    let embed = CreateEmbed::new()
        .description(embed_description)
        .color(5814783) // Blue color
        .timestamp(Timestamp::now());

    let message = CreateMessage::new().embed(embed);

    list_channel_id.send_message(&ctx.http, message).await?;
    Ok(())
}

pub enum EmbedColor {
    Blue,
    Green,
    Red,
    Yellow,
}

impl EmbedColor {
    pub fn value(&self) -> u32 {
        match self {
            EmbedColor::Blue => 5814783,
            EmbedColor::Green => 3066993,
            EmbedColor::Red => 15158332,
            EmbedColor::Yellow => 16776960,
        }
    }
}